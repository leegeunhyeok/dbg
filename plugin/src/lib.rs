use swc_core::{
    common::SyntaxContext,
    ecma::ast::Program,
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
use swc_dbg::dbg;

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(&mut dbg(
        metadata.source_map,
        SyntaxContext::empty().apply_mark(metadata.unresolved_mark),
    ))
}
