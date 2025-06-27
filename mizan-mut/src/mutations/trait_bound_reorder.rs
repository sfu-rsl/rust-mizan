use anyhow::Result;
use quote::quote;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use syn::{
    parse_file,
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
    Token, TypeParamBound, WhereClause, WherePredicate,
};
pub struct TraitBoundReorderMutator;

impl TraitBoundReorderMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = TraitBoundReorderVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct TraitBoundReorderVisitor {
    rng: ThreadRng,
}

impl TraitBoundReorderVisitor {
    fn new() -> Self {
        Self { rng: rand::rng() }
    }

    fn flip_where_clause(&mut self, where_clause: &mut WhereClause) {
        // First, randomly shuffle the predicates (e.g., T: A, U: B => U: B, T: A)
        let mut predicates: Vec<_> = where_clause.predicates.iter().cloned().collect();
        if predicates.len() >= 2 {
            predicates.shuffle(&mut self.rng);
            where_clause.predicates = predicates.into_iter().collect();
        }

        // Then, for each predicate, flip the trait bounds
        for predicate in &mut where_clause.predicates {
            match predicate {
                WherePredicate::Type(pred_type) => {
                    self.flip_type_bounds(&mut pred_type.bounds);
                }
                _ => {} // Ignore lifetime and eq predicates
            }
        }
    }

    fn flip_type_bounds(&mut self, bounds: &mut Punctuated<TypeParamBound, Token![+]>) {
        // Collect trait bounds (ignore lifetime bounds)
        let mut trait_bounds: Vec<TypeParamBound> = Vec::new();
        let mut other_bounds: Vec<TypeParamBound> = Vec::new();

        for bound in bounds.iter() {
            match bound {
                TypeParamBound::Trait(_) => trait_bounds.push(bound.clone()),
                _ => other_bounds.push(bound.clone()),
            }
        }

        // Only flip if we have 2 or more trait bounds
        if trait_bounds.len() >= 2 {
            trait_bounds.shuffle(&mut self.rng);
        }

        // Reconstruct bounds with flipped trait bounds but preserved lifetime bounds
        bounds.clear();
        for (i, bound) in other_bounds.into_iter().chain(trait_bounds).enumerate() {
            if i > 0 {
                bounds.push_punct(Default::default());
            }
            bounds.push_value(bound);
        }
    }
}

impl VisitMut for TraitBoundReorderVisitor {
    fn visit_item_fn_mut(&mut self, item_fn: &mut syn::ItemFn) {
        if let Some(where_clause) = &mut item_fn.sig.generics.where_clause {
            self.flip_where_clause(where_clause);
        }
        visit_mut::visit_item_fn_mut(self, item_fn);
    }

    fn visit_item_struct_mut(&mut self, item_struct: &mut syn::ItemStruct) {
        if let Some(where_clause) = &mut item_struct.generics.where_clause {
            self.flip_where_clause(where_clause);
        }
        visit_mut::visit_item_struct_mut(self, item_struct);
    }

    fn visit_item_enum_mut(&mut self, item_enum: &mut syn::ItemEnum) {
        if let Some(where_clause) = &mut item_enum.generics.where_clause {
            self.flip_where_clause(where_clause);
        }
        visit_mut::visit_item_enum_mut(self, item_enum);
    }

    fn visit_item_trait_mut(&mut self, item_trait: &mut syn::ItemTrait) {
        if let Some(where_clause) = &mut item_trait.generics.where_clause {
            self.flip_where_clause(where_clause);
        }
        visit_mut::visit_item_trait_mut(self, item_trait);
    }

    fn visit_item_impl_mut(&mut self, item_impl: &mut syn::ItemImpl) {
        if let Some(where_clause) = &mut item_impl.generics.where_clause {
            self.flip_where_clause(where_clause);
        }
        visit_mut::visit_item_impl_mut(self, item_impl);
    }

    fn visit_item_type_mut(&mut self, item_type: &mut syn::ItemType) {
        if let Some(where_clause) = &mut item_type.generics.where_clause {
            self.flip_where_clause(where_clause);
        }
        visit_mut::visit_item_type_mut(self, item_type);
    }
}
