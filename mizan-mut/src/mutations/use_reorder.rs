use anyhow::Result;
use quote::quote;
use rand::{rng, seq::SliceRandom};
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    ItemUse, UseTree,
};

pub struct UseReorderMutator;

impl UseReorderMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = UseReorderVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct UseReorderVisitor {
    rng: rand::rngs::ThreadRng,
}

impl UseReorderVisitor {
    fn new() -> Self {
        Self { rng: rng() }
    }

    fn reorder_use_tree(&mut self, tree: &mut UseTree) {
        match tree {
            UseTree::Group(group) => {
                let mut items: Vec<UseTree> = group.items.iter().cloned().collect();

                if items.len() >= 2 {
                    items.shuffle(&mut self.rng);

                    group.items.clear();
                    for (i, item) in items.into_iter().enumerate() {
                        if i > 0 {
                            group.items.push_punct(syn::token::Comma::default());
                        }
                        group.items.push_value(item);
                    }
                }

                for item in group.items.iter_mut() {
                    self.reorder_use_tree(item);
                }
            }
            UseTree::Path(path) => {
                self.reorder_use_tree(&mut path.tree);
            }
            _ => {}
        }
    }
}

impl VisitMut for UseReorderVisitor {
    fn visit_item_use_mut(&mut self, item_use: &mut ItemUse) {
        self.reorder_use_tree(&mut item_use.tree);
        visit_mut::visit_item_use_mut(self, item_use);
    }
}
