use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_file, parse_quote,
    spanned::Spanned,
    visit_mut::{self, VisitMut},
    Expr, ExprForLoop, Ident,
};

pub struct ForToWhileMutator;

impl ForToWhileMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ForToWhileVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ForToWhileVisitor {
    counter: usize,
}

impl ForToWhileVisitor {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn is_simple_expr(expr: &Expr) -> bool {
        match expr {
            // Simple identifiers: x, vec, items
            Expr::Path(_) => true,
            // Simple method calls on identifiers: vec.iter(), items.into_iter()
            Expr::MethodCall(method_call) => {
                matches!(&*method_call.receiver, Expr::Path(_))
            }
            // Range expressions: 0..10, 1..=n
            Expr::Range(_) => true,
            // Simple calls: Vec::new(), but NOT macro calls like vec![]
            Expr::Call(call) => {
                matches!(&*call.func, Expr::Path(_))
            }
            // Macro calls: vec![1,2,3]
            Expr::Macro(_) => true,
            _ => false,
        }
    }

    fn transform_for_loop(&mut self, for_loop: &ExprForLoop) -> Option<TokenStream> {
        // Only transform if the iterator expression is simple
        if !Self::is_simple_expr(&for_loop.expr) {
            return None;
        }

        let pat = &for_loop.pat;
        let expr = &for_loop.expr;
        let body = &for_loop.body;

        // Generate a unique iterator variable name
        self.counter += 1;
        let iter_var = Ident::new(&format!("__iter{}", self.counter), expr.span());

        Some(quote! {
            {
                let mut #iter_var = ::std::iter::IntoIterator::into_iter(#expr);
                while let ::std::option::Option::Some(#pat) = ::std::iter::Iterator::next(&mut #iter_var) {
                    #body
                }
            }
        })
    }
}

impl VisitMut for ForToWhileVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::ForLoop(for_loop) = expr {
            // Only transform for loops without labels
            if for_loop.label.is_none() {
                if let Some(transformed) = self.transform_for_loop(for_loop) {
                    *expr = parse_quote!(#transformed);
                }
            }
        }

        visit_mut::visit_expr_mut(self, expr);
    }
}
