use swc_core::{
    common::SyntaxContext,
    ecma::{
        ast::Pass,
        visit::{visit_mut_pass, VisitMut},
    },
    plugin::proxies::PluginSourceMapProxy,
};
use transformer::DbgTransformer;

pub fn dbg(
    cm: PluginSourceMapProxy,
    unresolved_ctxt: SyntaxContext,
    enabled: bool,
) -> impl VisitMut + Pass {
    visit_mut_pass(DbgTransformer::new(cm, unresolved_ctxt, enabled))
}

mod transformer;
mod types;
