use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};
use syn::{
    parse_file,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
    ItemFn, Lifetime, Result,
};

pub struct RenameLifetimeMutator;

impl RenameLifetimeMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = RenameLifetimeVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

pub struct RenameLifetimeVisitor;

impl VisitMut for RenameLifetimeVisitor {
    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        // Find all unique lifetime names in this function
        let mut collector = LifetimeCollector::default();
        collector.visit_item_fn(i);

        // Create a mapping of Old Name -> New Unique Name
        let mut rename_map = HashMap::new();
        // Deterministic ordering of lifetimes
        let mut sorted_lifetimes: Vec<_> = collector.names.iter().collect();
        sorted_lifetimes.sort();

        let mut counter = 0;

        for old_name in sorted_lifetimes {
            // Skip 'static - never rename the static lifetime
            if old_name == "static" || old_name == "_" {
                continue;
            }

            // Generate a name that is guaranteed to be unique
            let mut new_name = format!("__life{}", counter);
            // Ensure new name doesn't conflict with existing names
            while collector.names.contains(&new_name) {
                counter += 1;
                new_name = format!("__life{}", counter);
            }

            rename_map.insert(old_name.clone(), new_name);
            counter += 1;
        }

        // If we found lifetimes to rename, run the ScopedRenamer
        if !rename_map.is_empty() {
            let mut renamer = ScopedRenamer { rename_map };
            // Use the Default VisitMut implementation to traverse the function using our renamer.
            // We pass &mut renamer and i.
            // This will visit all children of i. If it encounters a nested ItemFn,
            // it will call renamer.visit_item_fn_mut(child), which we have overridden to do nothing.
            visit_mut::visit_item_fn_mut(&mut renamer, i);
        }
    }
}

#[derive(Default)]
struct LifetimeCollector {
    names: HashSet<String>,
}

impl<'ast> Visit<'ast> for LifetimeCollector {
    fn visit_lifetime(&mut self, i: &'ast Lifetime) {
        self.names.insert(i.ident.to_string());
    }
}

struct ScopedRenamer {
    rename_map: HashMap<String, String>,
}

impl VisitMut for ScopedRenamer {
    fn visit_lifetime_mut(&mut self, i: &mut Lifetime) {
        let old_name = i.ident.to_string();
        if let Some(new_name) = self.rename_map.get(&old_name) {
            i.ident = format_ident!("{}", new_name);
        }
        // Continue walking for nested structures
        visit_mut::visit_lifetime_mut(self, i);
    }

    // Stop at nested function definitions to prevent shadowing issues
    fn visit_item_fn_mut(&mut self, _i: &mut ItemFn) {}
}
