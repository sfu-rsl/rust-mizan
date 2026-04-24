use anyhow::Result;
use quote::quote;
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Generics, ItemStruct, ItemTrait, Signature,
};
pub struct RemoveExplicitWhereMutator;

impl RemoveExplicitWhereMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = RemoveExplicitWhereVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct RemoveExplicitWhereVisitor;

fn is_simple_movable_type_predicate(
    type_predicate: &syn::PredicateType,
    generics: &Generics,
) -> bool {
    if type_predicate.lifetimes.is_some() || type_predicate.bounds.is_empty() {
        return false;
    }

    let bound_type_ident = match &type_predicate.bounded_ty {
        syn::Type::Path(type_path)
            if type_path.qself.is_none() && type_path.path.segments.len() == 1 =>
        {
            &type_path.path.segments[0].ident
        }
        _ => return false,
    };

    if !generics
        .type_params()
        .any(|type_param| type_param.ident == *bound_type_ident)
    {
        return false;
    }

    // Keep the transformation conservative: skip higher-ranked bounds and
    // other complex forms
    type_predicate.bounds.iter().all(|bound| match bound {
        syn::TypeParamBound::Lifetime(_) => true,
        syn::TypeParamBound::Trait(trait_bound) => {
            trait_bound.lifetimes.is_none()
                && matches!(trait_bound.modifier, syn::TraitBoundModifier::None)
        }
        _ => false,
    })
}

fn move_where_clause_bounds_to_type_params(generics: &mut Generics) {
    let predicates = match &mut generics.where_clause {
        Some(wc) => std::mem::take(&mut wc.predicates),
        None => return,
    };

    let mut remaining_predicates = syn::punctuated::Punctuated::new();

    for predicate in predicates {
        match predicate {
            // moving simple type bounds only
            syn::WherePredicate::Type(type_predicate)
                if is_simple_movable_type_predicate(&type_predicate, generics) =>
            {
                let mut moved = false;

                if let syn::Type::Path(type_path) = &type_predicate.bounded_ty {
                    if type_path.qself.is_none() && type_path.path.segments.len() == 1 {
                        let ident = &type_path.path.segments[0].ident;
                        if let Some(type_param) = generics
                            .type_params_mut()
                            .find(|type_param| type_param.ident == *ident)
                        {
                            for bound in type_predicate.bounds.iter().cloned() {
                                type_param.bounds.push(bound);
                            }
                            moved = true;
                        }
                    }
                }

                if !moved {
                    remaining_predicates.push(syn::WherePredicate::Type(type_predicate));
                }
            }
            other => remaining_predicates.push(other),
        }
    }

    if remaining_predicates.is_empty() {
        generics.where_clause = None;
    } else if let Some(where_clause) = &mut generics.where_clause {
        where_clause.predicates = remaining_predicates;
    }
}

impl VisitMut for RemoveExplicitWhereVisitor {
    fn visit_signature_mut(&mut self, sig: &mut Signature) {
        move_where_clause_bounds_to_type_params(&mut sig.generics);

        visit_mut::visit_signature_mut(self, sig);
    }

    fn visit_item_struct_mut(&mut self, item_struct: &mut ItemStruct) {
        move_where_clause_bounds_to_type_params(&mut item_struct.generics);

        visit_mut::visit_item_struct_mut(self, item_struct);
    }

    fn visit_item_trait_mut(&mut self, item_trait: &mut ItemTrait) {
        move_where_clause_bounds_to_type_params(&mut item_trait.generics);

        visit_mut::visit_item_trait_mut(self, item_trait);
    }

    fn visit_item_enum_mut(&mut self, item_enum: &mut syn::ItemEnum) {
        move_where_clause_bounds_to_type_params(&mut item_enum.generics);

        visit_mut::visit_item_enum_mut(self, item_enum);
    }

    fn visit_item_impl_mut(&mut self, item_impl: &mut syn::ItemImpl) {
        move_where_clause_bounds_to_type_params(&mut item_impl.generics);

        visit_mut::visit_item_impl_mut(self, item_impl);
    }
}
