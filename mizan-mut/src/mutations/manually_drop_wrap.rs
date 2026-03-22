use anyhow::Result;
use quote::quote;
use syn::{
    Block, Expr, ImplItemFn, ItemFn, Pat, Stmt, parse_file, parse_quote, visit_mut::{self, VisitMut}
};

pub struct ManuallyDropWrapMutator;

impl ManuallyDropWrapMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ManuallyDropWrapVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ManuallyDropWrapVisitor;

impl ManuallyDropWrapVisitor {
    fn add_to_block(block: &mut Block) {
        let mut to_add: Vec<(usize, [Stmt; 2])> = Vec::new();

        for i in 0..block.stmts.len() {
            let statment = &mut block.stmts[i];
            let mut ident_name = None;
            let mut mutability = None;

            let Stmt::Local(local) = statment else { continue };

            if !local.attrs.is_empty() {
                continue;
            }

            match &mut local.pat {
                Pat::Ident(ident) => {
                    mutability = Some(&mut ident.mutability);
                    ident_name = Some(&ident.ident);
                }
                _ => {}
            }

            let Some(ident) = ident_name else { continue };
            let Some(init) = &mut local.init else { continue };

            if matches!(*init.expr, Expr::Macro(_) | Expr::Reference(_)) {
                continue;
            }

            to_add.push((
                i + 1,
                [
                    parse_quote! {
                        let #ident = ::std::mem::ManuallyDropWrap::new(#ident);
                    },
                    parse_quote! {
                        let #mutability #ident = ::std::mem::ManuallyDropWrap::into_inner(#ident);
                    },
                ],
            ));

            mutability.map(|m| *m = None);
        }

        let mut add_count = 0;
        for mut item in to_add.drain(..) {
            item.0 += add_count;
            add_count += item.1.len();
            block.stmts.splice(item.0..item.0, item.1);
        }
    }
}

impl VisitMut for ManuallyDropWrapVisitor {
    fn visit_item_fn_mut(&mut self, fun: &mut ItemFn) {
        Self::add_to_block(&mut fun.block);

        visit_mut::visit_item_fn_mut(self, fun);
    }

    fn visit_impl_item_fn_mut(&mut self, fun: &mut ImplItemFn) {
        Self::add_to_block(&mut fun.block);

        visit_mut::visit_impl_item_fn_mut(self, fun);
    }
}
