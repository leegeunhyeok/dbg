use swc_core::ecma::{
    ast::Pass,
    visit::{visit_mut_pass, VisitMut},
};
use transformer::DbgTransformer;

pub fn dbg() -> impl VisitMut + Pass {
    visit_mut_pass(DbgTransformer {})
}

mod transformer;
