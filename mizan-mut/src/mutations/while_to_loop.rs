use anyhow::Result;
use quote::quote;
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Expr,
};

pub struct WhileToLoopMutator;

impl WhileToLoopMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = WhileToLoopVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct WhileToLoopVisitor;

impl VisitMut for WhileToLoopVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::While(while_expr) => {
                // Only transform simple while loops (not while let)
                // Check if the condition is not a let expression
                if while_expr.label.is_none() && !matches!(&*while_expr.cond, Expr::Let(_)) {
                    let condition = &while_expr.cond;
                    let body = &while_expr.body;

                    let loop_expr = quote! {
                        loop {
                            if !(#condition) {
                                break;
                            }
                            #body
                        }
                    };

                    *expr = syn::parse2(loop_expr).unwrap();
                }
            }
            _ => {}
        }

        // Continue visiting nested expressions
        visit_mut::visit_expr_mut(self, expr);
    }
}
