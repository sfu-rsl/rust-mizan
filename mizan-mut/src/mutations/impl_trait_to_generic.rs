use anyhow::Result;
use quote::quote;
use rand::{
    self, distr::{Alphanumeric, SampleString}, rngs::ThreadRng
};
use syn::{
    parse_file, Token, parse_quote,
    visit_mut::{self, VisitMut},
    punctuated::Punctuated,
     ImplItemFn, ItemFn, Signature, GenericParam, TypeParamBound, Ident, Type, FnArg
};
use proc_macro2::Span;

pub struct ImplTraitToGenericMutator;

impl ImplTraitToGenericMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ImplTraitToGenericVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ImplTraitToGenericVisitor {
    rng: ThreadRng
}

impl ImplTraitToGenericVisitor {
    fn new() -> ImplTraitToGenericVisitor {
        ImplTraitToGenericVisitor {
            rng: rand::rng(),
        }
    }

    fn remove_impl_traits(&mut self, sig: &mut Signature) {
        let mut names: Vec<String> = sig.generics.params.iter().filter_map(|param| match param {
                GenericParam::Type(tparam) => {
                    Some(tparam.ident.to_string())
                }
                GenericParam::Const(cparam) => {
                    Some(cparam.ident.to_string())
                }
                _ => None
        }).collect();

        for mut arg in sig.inputs.iter_mut() {
            if let FnArg::Typed(pat) = &mut arg {
                let mut new_name: Option<(String, &Punctuated<TypeParamBound, Token![+]>)> = None;

                if let Type::ImplTrait(impl_trait) = &*pat.ty {
                    new_name = Some((self.generate_trait_name(&names), &impl_trait.bounds));
                }

                if let Some((name, bounds)) = new_name {
                    let name_ident = Ident::new(&name, Span::call_site());
                    sig.generics
                        .params
                        .push(GenericParam::Type(parse_quote! { #name_ident: #bounds }));
                    pat.ty = Box::new(Type::Verbatim(quote! {#name_ident}));
                    names.push(name);
                }
            }
        }
    }

    fn generate_trait_name(&mut self, current_names: &Vec<String>) -> String {
        let mut current_len = 1;
        let mut new_name: String;
        loop {
            new_name = Alphanumeric.sample_string(&mut self.rng, current_len);
            if !current_names.contains(&new_name) {
                break;
            }
            current_len += 1;
        }
        return new_name;
    }
}

impl VisitMut for ImplTraitToGenericVisitor {
    fn visit_item_fn_mut(&mut self, fun: &mut ItemFn) {
        self.remove_impl_traits(&mut fun.sig);

        visit_mut::visit_item_fn_mut(self, fun);
    }

    fn visit_impl_item_fn_mut(&mut self, fun: &mut ImplItemFn) {
        self.remove_impl_traits(&mut fun.sig);

        visit_mut::visit_impl_item_fn_mut(self, fun);
    }
}
