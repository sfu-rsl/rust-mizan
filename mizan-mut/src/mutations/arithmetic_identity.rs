use anyhow::Result;
use quote::quote;
use rand::{rngs::ThreadRng, Rng};
use syn::{
    parse_file,
    visit_mut::{self, VisitMut},
    BinOp, Expr, ExprBinary, Lit, LitInt,
};

pub struct ArithmeticIdentityMutator;

impl ArithmeticIdentityMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ArithmeticIdentityVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote ! (# file).to_string())
    }
}

struct ArithmeticIdentityVisitor {
    rng: ThreadRng,
}

impl ArithmeticIdentityVisitor {
    fn new() -> Self {
        Self { rng: rand::rng() }
    }
    /// Check if an expression is a literal integer or float
    fn is_numeric_literal(expr: &Expr) -> bool {
        match expr {
            Expr::Lit(expr_lit) => matches!(&expr_lit.lit, Lit::Int(_) | Lit::Float(_)),
            _ => false,
        }
    }
    /// Check if this binary expression is suitable for arithmetic identity mutation
    fn is_simple_numeric_addition(binary: &ExprBinary) -> bool {
        if !matches!(binary.op, BinOp::Add(_)) {
            return false;
        }
        Self::is_numeric_literal(&binary.left) || Self::is_numeric_literal(&binary.right)
    }
}

impl VisitMut for ArithmeticIdentityVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        visit_mut::visit_expr_mut(self, expr);
        if let Expr::Binary(binary) = expr {
            if Self::is_simple_numeric_addition(binary) {
                let random_value = self.rng.random_range(1..=100);
                let original_expr = expr.clone();
                let lit_str = random_value.to_string();
                let lit = LitInt::new(&lit_str, proc_macro2::Span::call_site());
                let identity_expr = quote! { (# original_expr + # lit - # lit) };
                *expr = syn::parse2(identity_expr).unwrap();
            }
        }
    }
}
