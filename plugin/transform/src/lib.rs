//! AST transform implementation for `unplugin-dbg`.

use swc_core::{
    common::SyntaxContext,
    ecma::{
        ast::Pass,
        visit::{visit_mut_pass, VisitMut},
    },
    plugin::proxies::PluginSourceMapProxy,
};
use transformer::DbgTransformer;

/// Creates an SWC pass that transforms unresolved `dbg` calls.
pub fn dbg(
    cm: PluginSourceMapProxy,
    unresolved_ctxt: SyntaxContext,
    enabled: bool,
) -> impl VisitMut + Pass {
    visit_mut_pass(DbgTransformer::new(cm, unresolved_ctxt, enabled))
}

mod transformer;
mod types;
