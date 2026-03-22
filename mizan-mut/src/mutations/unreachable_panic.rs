use anyhow::Result;
use proc_macro2::Span;
use quote::quote;
use rand::RngCore;
use syn::{
    parse_file, parse_quote,
    visit_mut::{self, VisitMut},
    Ident, Item, ItemFn,
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
    // Our mutations currently only operate at the file level, so the flag cannot be global to the crate.
    flag_ident: syn::Ident,
    flag_name: String,
    mod_level: usize,
}

impl UnreachablePanicVisitor {
    fn new() -> Self {
        let mut rng = rand::rng();
        let random_id = rng.next_u32();
        let flag_name = String::from("__MIZAN_PANIC_FLG_") + &random_id.to_string();
        let flag_ident = Ident::new(&flag_name, Span::call_site());

        Self {
            flag_ident: flag_ident,
            flag_name: flag_name,
            mod_level: 0,
        }
    }

    fn ensure_flag(&self, items: &mut Vec<syn::Item>) -> bool {
        // Immediately return if there is a name collision
        for item in items.iter() {
            if let Item::Const(c) = item {
                if c.ident == self.flag_ident {
                    return false;
                }
            }
        }

        let flag_ident = &self.flag_ident;
        let flag_item = parse_quote! { const #flag_ident: bool = true; };
        items.insert(0, flag_item);

        true
    }

    fn apply_panic_to_block(&self, block: &mut syn::Block) -> syn::Block {
        let flag_ident = &self.flag_ident;
        let original_block = block;

        let new_block: syn::Block = parse_quote! {
            {
                match #flag_ident {
                    true => #original_block,
                    false => panic!(),
                }
            }
        };

        new_block
    }

    fn create_flag_use_stmt(&self) -> Option<syn::Item> {
        // Construct the use statement as a String
        let mut use_str = String::from("use ");

        let mut path_str = String::from("super::");
        path_str = path_str.repeat(self.mod_level);
        path_str.push_str(&self.flag_name);
        path_str.push(';');

        use_str.push_str(&path_str);

        let use_result: Result<Item, _> = syn::parse_str(&use_str);

        match &use_result {
            Ok(use_item) => Some(parse_quote! { #use_item }),
            _ => None,
        }
    }
}

impl VisitMut for UnreachablePanicVisitor {
    fn visit_file_mut(&mut self, file: &mut syn::File) {
        // Safety check at the file level for name collision
        if self.ensure_flag(&mut file.items) {
            visit_mut::visit_file_mut(self, file);
        }
    }

    fn visit_item_mod_mut(&mut self, module: &mut syn::ItemMod) {
        // Include a use stmt for super module's flag declaration.
        // Only do this if the module has content.
        if let Some((_, items)) = &mut module.content {
            self.mod_level += 1;

            if let Some(use_item) = self.create_flag_use_stmt() {
                items.insert(0, use_item);
                visit_mut::visit_item_mod_mut(self, module);
            }

            self.mod_level -= 1;
        }
    }

    fn visit_trait_item_fn_mut(&mut self, func: &mut syn::TraitItemFn) {
        visit_mut::visit_trait_item_fn_mut(self, func);

        if let Some(original_block) = &mut func.default {
            let new_block = self.apply_panic_to_block(original_block);
            func.default = Some(new_block);
        }
    }

    fn visit_impl_item_fn_mut(&mut self, func: &mut syn::ImplItemFn) {
        visit_mut::visit_impl_item_fn_mut(self, func);

        let new_block = self.apply_panic_to_block(&mut func.block);
        func.block = new_block;
    }

    fn visit_item_fn_mut(&mut self, func: &mut ItemFn) {
        visit_mut::visit_item_fn_mut(self, func);

        let new_block = self.apply_panic_to_block(&mut func.block);
        func.block = Box::new(new_block);
    }
}
