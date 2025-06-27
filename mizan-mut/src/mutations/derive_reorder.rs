use anyhow::Result;
use quote::quote;
use rand::{rng, seq::SliceRandom};
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Attribute, Meta,
};

pub struct DeriveReorderMutator;

impl DeriveReorderMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = DeriveReorderVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct DeriveReorderVisitor {
    rng: rand::rngs::ThreadRng,
}

impl DeriveReorderVisitor {
    fn new() -> Self {
        Self { rng: rng() }
    }

    fn reorder_derive_attr(&mut self, attr: &mut Attribute) {
        if attr.path().is_ident("derive") {
            // Parse the attribute's meta
            if let Meta::List(meta_list) = &mut attr.meta {
                // Convert tokens to string and parse trait names
                let tokens_str = meta_list.tokens.to_string();
                let mut traits: Vec<&str> = tokens_str
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                // Only reorder if we have 2 or more traits
                if traits.len() >= 2 {
                    // Shuffle the traits
                    traits.shuffle(&mut self.rng);

                    // Reconstruct the token stream
                    let reordered = traits.join(", ");
                    if let Ok(new_tokens) = reordered.parse() {
                        meta_list.tokens = new_tokens;
                    }
                }
            }
        }
    }
}

impl VisitMut for DeriveReorderVisitor {
    fn visit_item_struct_mut(&mut self, item_struct: &mut syn::ItemStruct) {
        for attr in &mut item_struct.attrs {
            self.reorder_derive_attr(attr);
        }
        visit_mut::visit_item_struct_mut(self, item_struct);
    }

    fn visit_item_enum_mut(&mut self, item_enum: &mut syn::ItemEnum) {
        for attr in &mut item_enum.attrs {
            self.reorder_derive_attr(attr);
        }
        visit_mut::visit_item_enum_mut(self, item_enum);
    }

    fn visit_item_union_mut(&mut self, item_union: &mut syn::ItemUnion) {
        for attr in &mut item_union.attrs {
            self.reorder_derive_attr(attr);
        }
        visit_mut::visit_item_union_mut(self, item_union);
    }
}
