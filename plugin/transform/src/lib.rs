use swc_core::{
    common::SyntaxContext,
    ecma::{
        ast::Pass,
        visit::{visit_mut_pass, VisitMut},
    },
    plugin::proxies::PluginSourceMapProxy,
};
use transformer::DbgTransformer;

pub fn dbg(sm_proxy: PluginSourceMapProxy, unresolved_ctxt: SyntaxContext) -> impl VisitMut + Pass {
    visit_mut_pass(DbgTransformer::new(sm_proxy, unresolved_ctxt))
}

mod transformer;
mod types;
