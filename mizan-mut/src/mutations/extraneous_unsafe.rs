use anyhow::Result;
use quote::quote;
use rand::{rngs::ThreadRng, seq::IteratorRandom};
use syn::{
    parse_file, parse_quote,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
    Attribute, Block, Expr, ImplItemFn, ItemFn, Stmt,
};

pub struct ExtraneousUnsafeMutator;

impl ExtraneousUnsafeMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ExtraneousUnsafeVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ExtraneousUnsafeVisitor {
    rng: ThreadRng,
}

impl ExtraneousUnsafeVisitor {
    fn new() -> ExtraneousUnsafeVisitor {
        ExtraneousUnsafeVisitor { rng: rand::rng() }
    }
}

struct AttrVisitor {
    in_expr: bool,
}

impl AttrVisitor {}

impl Visit<'_> for AttrVisitor {
    fn visit_attribute<'ast>(&mut self, node: &'ast Attribute) {}

    fn visit_expr<'ast>(&mut self, node: &'ast Expr) {
        self.in_expr = true;
        self.in_expr = false;
    }
}

impl ExtraneousUnsafeVisitor {
    fn add_unsafe(&mut self, block: &mut Block) {
        let statements = &mut block.stmts;

        let max_unsafe_blocks = 1usize;

        let real_items = statements
            .iter_mut()
            .filter(|item| matches!(item, Stmt::Expr(_, _)))
            .filter(|item| match item {
                Stmt::Expr(Expr::Block(b), None) => b.attrs.is_empty(),
                _ => true,
            })
            .choose_multiple(&mut self.rng, max_unsafe_blocks);

        for item in real_items {
            match item {
                Stmt::Expr(expr, semi) => {
                    let new_expr: Stmt = parse_quote! {
                        unsafe {
                            #expr #semi
                        }
                    };
                    *item = new_expr;
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
}

impl VisitMut for ExtraneousUnsafeVisitor {
    fn visit_item_fn_mut(&mut self, fun: &mut ItemFn) {
        self.add_unsafe(&mut fun.block);

        visit_mut::visit_item_fn_mut(self, fun);
    }

    fn visit_impl_item_fn_mut(&mut self, fun: &mut ImplItemFn) {
        self.add_unsafe(&mut fun.block);

        visit_mut::visit_impl_item_fn_mut(self, fun);
    }
}
