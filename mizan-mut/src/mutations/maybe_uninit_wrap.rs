use anyhow::Result;
use quote::quote;
use syn::{
    parse_file, parse_quote,
    visit_mut::{self, VisitMut},
    Expr, Pat, Stmt,
};

pub struct MaybeUninitWrapMutator;

impl MaybeUninitWrapMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = MaybeUninitWrapVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct MaybeUninitWrapVisitor;

impl MaybeUninitWrapVisitor {
    fn add_maybe_uninit(expr: &Expr) -> Expr {
        Expr::Unsafe(parse_quote! {
            unsafe {
                let __real_expr = #expr;
                let __var_name = ::std::mem::MaybeUninit::new(__real_expr);
                __var_name.assume_init()
            }
        })
    }
}

impl VisitMut for MaybeUninitWrapVisitor {
    fn visit_stmt_mut(&mut self, statement: &mut syn::Stmt) {
        let Stmt::Local(local) = statement else {
            return visit_mut::visit_stmt_mut(self, statement);
        };

        match &local.pat {
            Pat::Ident(_) | Pat::Wild(_) | Pat::Type(_) => {
                if let Some(init) = &mut local.init {
                    if !matches!(*init.expr, Expr::Macro(_) | Expr::Reference(_)) {
                        *init.expr = Self::add_maybe_uninit(&*init.expr);
                    }
                }
            }
            _ => visit_mut::visit_local_mut(self, local),
        }
    }

    fn visit_macro_mut(&mut self, _: &mut syn::Macro) {}
}
