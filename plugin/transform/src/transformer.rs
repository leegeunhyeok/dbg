use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut};

pub struct DbgTransformer {}

impl VisitMut for DbgTransformer {
    noop_visit_mut_type!();
}
