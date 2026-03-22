use std::collections::HashMap;

use anyhow::Result;
use quote::quote;
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use syn::{
    parse_file, parse_quote,
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
    Block, Ident, Pat, Stmt,
};
pub struct RepeatedShadowingMutator;

impl RepeatedShadowingMutator {
    pub fn mutate(source: &str) -> Result<String> {
        let mut file = parse_file(source)?;
        let mut visitor = RepeatedShadowingVisitor::new();
        visitor.visit_file_mut(&mut file);
        Ok(quote!(#file).to_string())
    }
}

struct RepeatedShadowingVisitor {
    rng: ThreadRng,
    shadow_prob: f64,
    max_shadow_count: usize,
}

impl RepeatedShadowingVisitor {
    fn new() -> Self {
        Self::with_probability(0.5)
    }

    fn with_probability(p: f64) -> Self {
        Self::with_randomness(p, 10)
    }

    fn with_randomness(p: f64, max_shadow_count: usize) -> Self {
        Self {
            rng: rand::rng(),
            shadow_prob: p,
            max_shadow_count: max_shadow_count,
        }
    }
}

impl VisitMut for RepeatedShadowingVisitor {
    fn visit_block_mut(&mut self, block: &mut Block) {
        let mut new_stmts = Vec::new();
        let stmt_count = block.stmts.len();
        let mut stmt_index = 0;

        for stmt in block.stmts.drain(..) {
            new_stmts.push(stmt.clone());

            // Conditionally apply shadow with specified probability
            if stmt_index < stmt_count - 1 && self.rng.random_bool(self.shadow_prob) {
                if let Stmt::Local(local) = stmt {
                    if local.init.is_some() && local.attrs.is_empty() {
                        if let Pat::Ident(pat_ident) = &local.pat {
                            let ident = &pat_ident.ident;
                            let shadow_stmt = parse_quote! { let mut #ident = #ident; };
                            let shadow_count = self.rng.random_range(0..self.max_shadow_count);

                            new_stmts.resize(new_stmts.len()+shadow_count, shadow_stmt);
                        }
                    }
                }
            }

            stmt_index += 1;
        }

        block.stmts = new_stmts;
        visit_mut::visit_block_mut(self, block);
    }
}
