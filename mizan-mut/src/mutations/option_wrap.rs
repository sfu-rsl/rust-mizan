use anyhow::Result;
use quote::quote;
use syn::{
    parse_file, parse_quote,
    visit_mut::{self, VisitMut},
    Expr, Pat, Stmt,
};

pub struct OptionWrapMutator;

impl OptionWrapMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = OptionWrapVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct OptionWrapVisitor;

impl OptionWrapVisitor {
    fn wrap_with_option(expr: &Expr) -> Expr {
        Expr::MethodCall(parse_quote! {
            ::std::option::Option::Some(#expr).unwrap()
        })
    }
}

impl VisitMut for OptionWrapVisitor {
    fn visit_stmt_mut(&mut self, statment: &mut syn::Stmt) {
        let Stmt::Local(local) = statment else {
            return visit_mut::visit_stmt_mut(self, statment);
        };

        match &local.pat {
            Pat::Ident(_) | Pat::Wild(_) | Pat::Type(_) => {
                if let Some(init) = &mut local.init {
                    if !matches!(*init.expr, Expr::Macro(_) | Expr::Reference(_)) {
                        *init.expr = Self::wrap_with_option(&*init.expr);
                    }
                }
            }
            _ => visit_mut::visit_local_mut(self, local)
        }
    }

    fn visit_macro_mut(&mut self, _: &mut syn::Macro) {
    }
}
