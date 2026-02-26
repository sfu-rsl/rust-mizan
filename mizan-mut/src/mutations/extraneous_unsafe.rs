use anyhow::Result;
use quote::quote;
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Block, ImplItemFn, ItemFn, Stmt,
};

pub struct ExtraneousUnsafeMutator;

impl ExtraneousUnsafeMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ExtraneousUnsafeVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ExtraneousUnsafeVisitor;

impl ExtraneousUnsafeVisitor {
    fn add_unsafe(block: &mut Block) {
        let statements = &mut block.stmts;

        for item in statements.iter_mut() {
            match item {
                Stmt::Expr(expr, semi) => {
                    let new_expr = quote! {
                        unsafe {
                            #expr #semi
                        }
                    };
                    *item = syn::parse2(new_expr).unwrap();
                    break;
                }
                _ => {}
            }
        }
    }
}

impl VisitMut for ExtraneousUnsafeVisitor {
    fn visit_item_fn_mut(&mut self, fun: &mut ItemFn) {
        ExtraneousUnsafeVisitor::add_unsafe(&mut fun.block);

        visit_mut::visit_item_fn_mut(self, fun);
    }

    fn visit_impl_item_fn_mut(&mut self, fun: &mut ImplItemFn) {
        ExtraneousUnsafeVisitor::add_unsafe(&mut fun.block);

        visit_mut::visit_impl_item_fn_mut(self, fun);
    }
}
