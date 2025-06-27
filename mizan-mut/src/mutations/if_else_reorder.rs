use anyhow::Result;
use quote::quote;
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Expr, ExprIf,
};

pub struct IfElseReorderMutator;

impl IfElseReorderMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = IfElseReorderVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}
struct IfElseReorderVisitor;

impl VisitMut for IfElseReorderVisitor {
    fn visit_expr_if_mut(&mut self, expr_if: &mut ExprIf) {
        // Skip if this is an if let pattern
        if matches!(&*expr_if.cond, Expr::Let(_)) {
            visit_mut::visit_expr_if_mut(self, expr_if);
            return;
        }

        if let Some((else_token, else_expr)) = &expr_if.else_branch {
            if let Expr::Block(else_block) = else_expr.as_ref() {
                let original_cond = expr_if.cond.clone();
                let then_block = expr_if.then_branch.clone();
                let else_block = else_block.block.clone();
                let negated_cond = quote! {
                    !(#original_cond)
                };
                if let Ok(new_cond) = syn::parse2(negated_cond) {
                    expr_if.cond = Box::new(new_cond);
                    expr_if.then_branch = else_block;
                    expr_if.else_branch = Some((
                        else_token.clone(),
                        Box::new(Expr::Block(syn::ExprBlock {
                            attrs: vec![],
                            label: None,
                            block: then_block,
                        })),
                    ));
                }
            }
        }
        visit_mut::visit_expr_if_mut(self, expr_if);
    }
}
