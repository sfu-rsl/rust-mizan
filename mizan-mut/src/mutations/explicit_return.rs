use anyhow::Result;
use quote::quote;
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Stmt, ReturnType, ItemFn, Block, ImplItemFn, ExprBlock, Expr
};

pub struct ExplicitReturnMutator;

impl ExplicitReturnMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ExplicitReturnVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ExplicitReturnVisitor;

impl VisitMut for ExplicitReturnVisitor {
    fn visit_impl_item_fn_mut(&mut self, item: &mut ImplItemFn) {
        visit_mut::visit_visibility_mut(self, &mut item.vis);
        visit_mut::visit_signature_mut(self, &mut item.sig);
        visit_mut::visit_block_mut(self, &mut item.block);

        // Skip functions returning ()
        if matches!(item.sig.output, ReturnType::Default) {
            return;
        }

        let stmts: &mut Vec<Stmt> = &mut item.block.stmts;

        if stmts.is_empty() {
            return;
        }

        let stmts_len = stmts.len();
        let last_stmt: &mut Stmt = &mut stmts[stmts_len - 1];

        match last_stmt {
            Stmt::Expr(Expr::Block(b), semi) => {
                // Only apply if last statement doesn't end in semi-colon
                if semi.is_none() && b.attrs.is_empty() {
                    let ret_expr_tokens = quote! {
                        return {#b};
                    };

                    if let Ok(ret_expr) = syn::parse2(ret_expr_tokens) {
                        *last_stmt = ret_expr;
                    }
                }
            }
            Stmt::Expr(expr, semi) => {
                // Only apply if last statement doesn't end in semi-colon
                if semi.is_none() {
                    let ret_expr_tokens = quote! {
                        return {#expr};
                    };

                    if let Ok(ret_expr) = syn::parse2(ret_expr_tokens) {
                        *last_stmt = ret_expr;
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_item_fn_mut(&mut self, item: &mut ItemFn) {
        visit_mut::visit_visibility_mut(self, &mut item.vis);
        // visit_mut::visit_attributes_mut(self, &mut item.attrs);
        visit_mut::visit_signature_mut(self, &mut item.sig);
        visit_mut::visit_block_mut(self, &mut item.block);

        // Skip functions returning ()
        if matches!(item.sig.output, ReturnType::Default) {
            return;
        }

        let stmts : &mut Vec<Stmt> = &mut item.block.stmts;

        if stmts.is_empty() {
            visit_mut::visit_item_fn_mut(self, item);
            return;
        }

        let stmts_len = stmts.len();
        let last_stmt : &mut Stmt = &mut stmts[stmts_len-1];

        match last_stmt {
            Stmt::Expr(Expr::Block(b), semi) => {
                // Only apply if last statement doesn't end in semi-colon
                if semi.is_none() && b.attrs.is_empty() {
                    let ret_expr_tokens = quote! {
                        return {#b};
                    };

                    if let Ok(ret_expr) = syn::parse2(ret_expr_tokens) {
                        *last_stmt = ret_expr;
                    }
                }
            }
            _ => {}
        }

    }

}
