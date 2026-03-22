use anyhow::Result;
use quote::quote;
use syn::{
    parse_file, parse_quote,
    visit_mut::{self, VisitMut},
    Generics, ItemStruct, ItemTrait, Signature, WherePredicate,
};
pub struct ExplicitWhereMutator;

impl ExplicitWhereMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ExplicitWhereVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ExplicitWhereVisitor;

fn move_type_bounds_to_where_clause(generics: &mut Generics) {
    let mut new_predicates = Vec::new();

    for param in &mut generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            if !type_param.bounds.is_empty() {
                let type_ident = &type_param.ident;
                let bounds = &type_param.bounds;

                let predicate: WherePredicate = parse_quote!(#type_ident: #bounds);
                new_predicates.push(predicate);

                type_param.bounds.clear();
            }
        }
    }

    if !new_predicates.is_empty() {
        let where_clause = generics.make_where_clause();
        for pred in new_predicates {
            where_clause.predicates.push(pred);
        }
    }
}

impl VisitMut for ExplicitWhereVisitor {
    fn visit_signature_mut(&mut self, sig: &mut Signature) {
        move_type_bounds_to_where_clause(&mut sig.generics);

        visit_mut::visit_signature_mut(self, sig);
    }

    fn visit_item_struct_mut(&mut self, item_struct: &mut ItemStruct) {
        move_type_bounds_to_where_clause(&mut item_struct.generics);

        visit_mut::visit_item_struct_mut(self, item_struct);
    }

    fn visit_item_trait_mut(&mut self, item_trait: &mut ItemTrait) {
        move_type_bounds_to_where_clause(&mut item_trait.generics);

        visit_mut::visit_item_trait_mut(self, item_trait);
    }

    fn visit_item_enum_mut(&mut self, item_enum: &mut syn::ItemEnum) {
        move_type_bounds_to_where_clause(&mut item_enum.generics);

        visit_mut::visit_item_enum_mut(self, item_enum);
    }

    fn visit_item_impl_mut(&mut self, item_impl: &mut syn::ItemImpl) {
        move_type_bounds_to_where_clause(&mut item_impl.generics);

        visit_mut::visit_item_impl_mut(self, item_impl);
    }
}
