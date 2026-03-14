use anyhow::Result;
use quote::quote;
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    Expr, Lit,
};

pub struct ArithmeticIdentityMutator;

impl ArithmeticIdentityMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ArithmeticIdentityVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ArithmeticIdentityVisitor;

impl VisitMut for ArithmeticIdentityVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        if let Expr::Lit(expr_lit) = expr {
            if matches!(&expr_lit.lit, Lit::Int(_)) {
                let original = expr.clone();
                *expr = syn::parse2(quote! { (#original * 1) }).unwrap();
            }
        }
    }
}
