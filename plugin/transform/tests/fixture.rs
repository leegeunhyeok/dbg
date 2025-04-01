use std::path::PathBuf;

use swc_common::sync::OnceCell;
use swc_core::{
    common::{Mark, SyntaxContext},
    ecma::{ast::Pass, transforms::base::resolver, visit::VisitMut},
    plugin::proxies::PluginSourceMapProxy,
};
use swc_dbg::dbg;
use swc_ecma_parser::{Syntax, TsSyntax};
use swc_ecma_transforms_testing::test_fixture;

fn tr() -> impl VisitMut + Pass {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();
    let cm = PluginSourceMapProxy {
        source_file: OnceCell::new(),
    };

    (
        resolver(unresolved_mark, top_level_mark, false),
        dbg(cm, SyntaxContext::empty().apply_mark(unresolved_mark)),
    )
}

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let filename = input.to_string_lossy();
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: filename.ends_with(".tsx"),
            ..Default::default()
        }),
        &|_| tr(),
        &input,
        &output,
        Default::default(),
    );
}
