use anyhow::Result;
use quote::quote;
use syn::{
    Attribute, Block, ImplItemFn, ItemFn, Stmt, TraitItemFn, parse_file, parse_quote, visit_mut::{self, VisitMut}
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

struct AttributeCheckVisitor {
    attrs: Vec<Attribute>,
    entered_expr: bool,
}

impl AttributeCheckVisitor {
    fn new() -> Self {
        Self {
            attrs: vec![],
            entered_expr: false,
        }
    }
}

impl VisitMut for AttributeCheckVisitor {
    fn visit_stmt_mut(&mut self, _: &mut syn::Stmt) {}

    fn visit_expr_mut(&mut self, e: &mut syn::Expr) {
        // We only want to collect attributes at the current level
        if self.entered_expr {
            return;
        }

        self.entered_expr = true;
        visit_mut::visit_expr_mut(self, e);
    }

    fn visit_attributes_mut(&mut self, attrs: &mut Vec<syn::Attribute>) {
        std::mem::swap(&mut self.attrs, attrs);
    }
}

struct ExplicitReturnVisitor;

impl ExplicitReturnVisitor {
    fn apply_return_mut(block: &mut Block) {
        let stmts = &mut block.stmts;

        if stmts.is_empty() {
            return;
        }

        let mut last_stmt = stmts.last_mut().unwrap();

        // None ensures there is no semi-colon
        if let Stmt::Expr(ref mut expr, None) = &mut last_stmt {
            // Collect the attributes since we can't wrap them in return expressions.
            // We'll re-apply them manually down below.
            let mut attr_visitor = AttributeCheckVisitor::new();
            attr_visitor.visit_expr_mut(expr);

            let attrs = attr_visitor.attrs;

            let ret_expr = parse_quote! {
                #(#attrs)*
                return #expr;
            };

            *last_stmt = ret_expr;
        }
    }
}

impl VisitMut for ExplicitReturnVisitor {
    fn visit_impl_item_fn_mut(&mut self, item: &mut ImplItemFn) {
        visit_mut::visit_impl_item_fn_mut(self, item);
        Self::apply_return_mut(&mut item.block);
    }

    fn visit_item_fn_mut(&mut self, item: &mut ItemFn) {
        visit_mut::visit_item_fn_mut(self, item);
        Self::apply_return_mut(&mut item.block);
    }

    fn visit_trait_item_fn_mut(&mut self, item: &mut TraitItemFn) {
        visit_mut::visit_trait_item_fn_mut(self, item);

        // Apply them to traits with default impl's as well
        if let Some(block) = &mut item.default {
            Self::apply_return_mut(block);
        }
    }
}
