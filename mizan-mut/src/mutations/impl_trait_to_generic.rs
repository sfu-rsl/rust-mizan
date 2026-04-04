use anyhow::Result;
use proc_macro2::Span;
use quote::quote;
use rand::{self, rngs::ThreadRng, Rng};
use syn::{
    parse_file, parse_quote,
    visit_mut::{self, VisitMut},
    FnArg, GenericParam, Generics, Ident, ImplItemFn, Item, ItemFn, Signature, Type,
};

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
    rng: ThreadRng,
    param_names: Vec<String>,
}

impl ImplTraitToGenericVisitor {
    fn new() -> Self {
        Self {
            rng: rand::rng(),
            param_names: Vec::new(),
        }
    }

    fn remove_impl_traits(&mut self, sig: &mut Signature) {
        let old_size = self.param_names.len();
        self.param_names
            .append(&mut Self::get_generic_names(&sig.generics));

        for mut arg in sig.inputs.iter_mut() {
            if let FnArg::Typed(pat) = &mut arg {
                if let Type::ImplTrait(_) = &*pat.ty {
                    self.replace_impl_arg(&mut sig.generics, &mut pat.ty);
                }
            }
        }

        // NOTE: Return types can't be transformed because rust uses the return type found in the
        // function to infer the type of the impl trait

        self.param_names.truncate(old_size);
    }

    fn replace_impl_arg(&mut self, generics_list: &mut Generics, param_type: &mut Type) {
        let Type::ImplTrait(impl_trait) = &param_type else {
            return;
        };
        let name = self.generate_trait_name();
        let bounds = &impl_trait.bounds;
        let name_ident = Ident::new(&name, Span::call_site());
        generics_list
            .params
            .push(GenericParam::Type(parse_quote! { #name_ident: #bounds }));
        *param_type = Type::Verbatim(quote! {#name_ident});
        self.param_names.push(name);
    }

    fn generate_trait_name(&mut self) -> String {
        let mut current_num = self.rng.random_range(256..u32::MAX);
        let mut new_name: String = String::from("Type");
        loop {
            new_name += &current_num.to_string();
            if !self.param_names.contains(&new_name) {
                break;
            }
            current_num = self.rng.random_range(256..u32::MAX);
        }
        return new_name;
    }

    fn get_generic_names(generics: &Generics) -> Vec<String> {
        generics
            .params
            .iter()
            .filter_map(|param| match param {
                GenericParam::Type(tparam) => Some(tparam.ident.to_string()),
                GenericParam::Const(cparam) => Some(cparam.ident.to_string()),
                _ => None,
            })
            .collect()
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

    fn visit_item_mut(&mut self, item: &mut Item) {
        let Item::Impl(impl_item) = item else {
            return;
        };

        let mut gen_names = Self::get_generic_names(&impl_item.generics);
        let old_size = self.param_names.len();
        self.param_names.append(&mut gen_names);
        visit_mut::visit_item_mut(self, item);
        self.param_names.truncate(old_size);
    }
}
