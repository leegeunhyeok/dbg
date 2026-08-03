//! SWC plugin entry point for `unplugin-dbg`.
#![allow(
    missing_docs,
    reason = "the SWC plugin macro generates undocumented exported shims"
)]

use serde::Deserialize;
use swc_core::{
    common::SyntaxContext,
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_dbg::dbg;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DbgConfig {
    enabled: bool,
}

/// Applies the `dbg` transform using the plugin metadata configuration.
///
/// # Panics
///
/// Panics when the plugin configuration is missing or invalid.
#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<DbgConfig>(
        &metadata
            .get_transform_plugin_config()
            .expect("failed to get plugin config for unplugin-dbg"),
    )
    .expect("invalid config for unplugin-dbg");

    program.apply(&mut dbg(
        metadata.source_map,
        SyntaxContext::empty().apply_mark(metadata.unresolved_mark),
        config.enabled,
    ))
}
