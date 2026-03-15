use std::{
    cmp::{max, min},
    ops::RangeBounds,
};

use anyhow::Result;
use quote::quote;
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use syn::{
    parse_file, parse_quote,
    visit::{self, Visit},
    visit_mut::VisitMut,
    Attribute, Block, Stmt,
};

pub struct ExtraneousUnsafeMutator;

impl ExtraneousUnsafeMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = ExtraneousUnsafeVisitor;
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct ExtraneousUnsafeVisitor;

impl ExtraneousUnsafeVisitor {
    fn stmt_filter(item: &Stmt) -> bool {
        match item {
            Stmt::Expr(_, _) => {}
            _ => return false,
        }

        // Expressions which return conditionally and return values (i.e. #[cfg(..)])
        // Cannot be surrounded in unsafe as doing so may change the return value from
        // whatever it was to () if the conditional code is not ran.
        let mut has_attr = false;
        StmtAttrVisitor::with_before(|_| has_attr = true).visit(item);
        !has_attr
    }
}

impl VisitMut for ExtraneousUnsafeVisitor {
    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        UnsafeAdder::new(block, &(1..)).add_unsafes();
    }

    fn visit_macro_mut(&mut self, _: &mut syn::Macro) {}

    fn visit_expr_unsafe_mut(&mut self, _: &mut syn::ExprUnsafe) {}
}

/// A struct which adds unsafe expressions recursively to a `Block`.
struct UnsafeAdder<'a, Range: RangeBounds<usize>> {
    stmt_index: usize,
    stmt_count: usize,
    block: &'a mut Block,
    unsafe_count: &'a Range,
    rng: ThreadRng,
    stmt_indices: Vec<usize>,
}

impl<'a, Range: RangeBounds<usize>> UnsafeAdder<'a, Range> {
    /// `unsafe_count` is the (min, max) number of unsafe blocks to be added.
    /// The minimum can only be satisfied if enough expressions exist.
    fn new(block: &'a mut Block, unsafe_count: &'a Range) -> Self {
        Self {
            stmt_index: 0,
            stmt_count: 0,
            block,
            unsafe_count,
            rng: rand::rng(),
            stmt_indices: Vec::new(),
        }
    }

    /// Inserts `unsafe` blocks into the stored `Block` recursively.
    /// Returns the number of blocks inserted.
    fn add_unsafes(mut self) -> usize {
        // Count number of replaceable statements
        StmtVisitor::with_before(Self::create_stmt_filter(|_| self.stmt_count += 1))
            .visit(self.block);

        if self.stmt_count <= 0 {
            return 0;
        }

        // Calculate range of avaiable statements
        let stmt_amount_range = {
            let bottom_index = {
                let start = match self.unsafe_count.start_bound() {
                    std::ops::Bound::Included(i) => *i,
                    std::ops::Bound::Excluded(i) => *i + 1,
                    _ => 0,
                };
                min(self.stmt_count, start)
            };

            let top_index = {
                let end = match self.unsafe_count.end_bound() {
                    std::ops::Bound::Included(i) => *i,
                    std::ops::Bound::Excluded(i) => max(*i, 1) - 1,
                    _ => usize::MAX,
                };
                min(self.stmt_count, end)
            };
            bottom_index..=top_index
        };

        if stmt_amount_range.is_empty() {
            return 0;
        }

        let stmt_amount = self.rng.random_range(stmt_amount_range);
        self.stmt_indices = (0..self.stmt_count).choose_multiple(&mut self.rng, stmt_amount);

        StmtVisitor::new(
            |stmt| {
                if !Self::create_stmt_filter(|_| {})(stmt) {
                    return false;
                }

                let contains = self.stmt_indices.contains(&self.stmt_index);
                self.stmt_index += 1;
                contains
            },
            |stmt, should_replace| {
                if should_replace {
                    *stmt = parse_quote! {
                        unsafe {
                            #stmt
                        }
                    }
                }
            },
        )
        .visit(self.block);

        self.stmt_indices.len()
    }

    /// Filters statements to ones which we want to add `unsafe` to, while also allowing some code
    /// to run after successful filters
    fn create_stmt_filter<F: FnMut(&Stmt) -> R, R>(mut after: F) -> impl FnMut(&mut Stmt) -> bool {
        move |item: &mut Stmt| {
            let ret = ExtraneousUnsafeVisitor::stmt_filter(item);
            if ret {
                after(item);
            }
            return ret;
        }
    }
}

/// Visit `Stmt`s, while allowing you to call a function when doing so.
struct StmtVisitor<F: FnMut(&mut Stmt) -> R, G: FnMut(&mut Stmt, R) -> S, R, S> {
    before: F,
    after: G,
}

impl<F: FnMut(&mut Stmt) -> R, G: FnMut(&mut Stmt, R) -> S, R, S> StmtVisitor<F, G, R, S> {
    fn new(before: F, after: G) -> Self {
        Self { before, after }
    }

    fn visit(mut self, block: &mut Block) {
        self.visit_block_mut(block);
    }
}

impl<F: FnMut(&mut Stmt) -> R, R> StmtVisitor<F, fn(&mut Stmt, R) -> (), R, ()> {
    fn with_before(before: F) -> Self {
        Self::new(before, |_, _| {})
    }
}

impl<F: FnMut(&mut Stmt) -> R, G: FnMut(&mut Stmt, R) -> S, R, S> VisitMut
    for StmtVisitor<F, G, R, S>
{
    fn visit_stmt_mut(&mut self, item: &mut Stmt) {
        let r = (self.before)(item);
        syn::visit_mut::visit_stmt_mut(self, item);
        (self.after)(item, r);
    }
    fn visit_macro_mut(&mut self, _: &mut syn::Macro) {}

    fn visit_expr_unsafe_mut(&mut self, _: &mut syn::ExprUnsafe) {}
}

/// Visit (top-level) `Attr`s, while allowing you to call a function when doing so.
struct StmtAttrVisitor<F: FnMut(&Attribute) -> R, G: FnMut(&Attribute, R) -> S, R, S> {
    before: F,
    after: G,
    found_stmt: bool,
    found_attr: bool,
}

impl<F: FnMut(&Attribute) -> R, G: FnMut(&Attribute, R) -> S, R, S> StmtAttrVisitor<F, G, R, S> {
    fn new(before: F, after: G) -> Self {
        Self {
            before,
            after,
            found_stmt: false,
            found_attr: false,
        }
    }
}

impl<F: FnMut(&Attribute) -> R, R> StmtAttrVisitor<F, fn(&Attribute, R) -> (), R, ()> {
    fn with_before(before: F) -> Self {
        Self::new(before, |_, _| {})
    }

    fn visit(mut self, stmt: &Stmt) {
        self.visit_stmt(stmt);
    }
}

impl<'ast, F: FnMut(&Attribute) -> R, G: FnMut(&Attribute, R) -> S, R, S> Visit<'ast>
    for StmtAttrVisitor<F, G, R, S>
{
    fn visit_stmt(&mut self, stmt: &'ast syn::Stmt) {
        if self.found_stmt {
            return;
        }

        self.found_stmt = true;

        let old_found_attr = self.found_attr;
        visit::visit_stmt(self, stmt);
        if self.found_attr == old_found_attr {
            self.found_stmt = false;
        }
    }

    fn visit_attribute(&mut self, item: &'ast Attribute) {
        self.found_attr = true;
        let r = (self.before)(item);
        syn::visit::visit_attribute(self, item);
        (self.after)(item, r);
    }

    fn visit_macro(&mut self, _: &'ast syn::Macro) {}

    fn visit_expr_unsafe(&mut self, _: &'ast syn::ExprUnsafe) {}
}
