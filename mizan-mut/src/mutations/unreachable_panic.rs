use anyhow::Result;
use quote::quote;
use rand::{rng, seq::SliceRandom};
use syn::{
    parse_file, parse_quote,
    visit_mut::{self, VisitMut},
    Expr, File, Item, ItemFn, ItemStatic,
};

pub struct UnreachblePanicMutator;

impl UnreachblePanicMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = UnreachablePanicVisitor::new();

        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct UnreachablePanicVisitor {
}

impl UnreachablePanicVisitor {
    fn new() -> Self {
        Self {}
    }

    fn ensure_flag(&self, items: &mut Vec<syn::Item>) -> bool {
        let exists = items.iter().any(|item| {
            if let syn::Item::Const(c) = item {
                c.ident == "__MIZAN_PANIC_FLAG"
            } else {
                false
            }
        });

        if exists {
            false
        } else {
            let flag = parse_quote! { const __MIZAN_PANIC_FLAG: bool = true; };
            items.insert(0, flag);
            true
        }
    }
}

impl VisitMut for UnreachablePanicVisitor {
    fn visit_file_mut(&mut self, file: &mut syn::File) {
        if self.ensure_flag(&mut file.items) {
            visit_mut::visit_file_mut(self, file);
        }
    }


    fn visit_item_mod_mut(&mut self, module: &mut syn::ItemMod) {
        match &mut module.content {
            Some((brace, items)) => {
                if !self.ensure_flag(items) {
                    return;
                }
            },
            _ => {}
        }

        visit_mut::visit_item_mod_mut(self, module);
    }


    fn visit_trait_item_fn_mut(&mut self, func: &mut syn::TraitItemFn) {
        visit_mut::visit_trait_item_fn_mut(self, func);

        let guard :Expr = parse_quote!(__MIZAN_PANIC_FLAG);
            if let Some(original_block) = &mut func.default {

            let new_block: syn::Block = parse_quote! {
                {
                    match #guard {
                        true => #original_block,
                        false => panic!(),
                    }
                }
            };

            func.default = Some(new_block);
        }
    }

    fn visit_impl_item_fn_mut(&mut self, func: &mut syn::ImplItemFn) {
        visit_mut::visit_impl_item_fn_mut(self, func);

        let guard :Expr= parse_quote!(__MIZAN_PANIC_FLAG);
        let original_block = &func.block;

        let new_block: syn::Block = parse_quote! {
            {
                match #guard {
                    true => #original_block,
                    false => panic!(),
                }
            }
        };

        func.block = new_block;
    }

    fn visit_item_fn_mut(&mut self, func: &mut ItemFn) {
        visit_mut::visit_item_fn_mut(self, func);

        let guard :Expr= parse_quote!(__MIZAN_PANIC_FLAG);
        let original_block = &func.block;

        let new_block: syn::Block = parse_quote! {
            {
                match #guard {
                    true => #original_block,
                    false => panic!(),
                }
            }
        };

        func.block = Box::new(new_block);
    }
}
