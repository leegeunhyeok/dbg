//! Fixture tests for the `dbg` transform.

use std::path::{Path, PathBuf};

use swc_common::sync::OnceCell;
use swc_core::{
    common::{Mark, SyntaxContext},
    ecma::{ast::Pass, transforms::base::resolver, visit::VisitMut},
    plugin::proxies::PluginSourceMapProxy,
};
use swc_dbg::dbg;
use swc_ecma_parser::{Syntax, TsSyntax};
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};

fn tr(enabled: bool) -> impl VisitMut + Pass {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();
    let cm = PluginSourceMapProxy {
        source_file: OnceCell::new(),
    };

    (
        resolver(unresolved_mark, top_level_mark, false),
        dbg(
            cm,
            SyntaxContext::empty().apply_mark(unresolved_mark),
            enabled,
        ),
    )
}

fn run_fixture(input: &Path, enabled: bool) {
    let filename = input.to_string_lossy();
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: filename.ends_with(".tsx"),
            ..Default::default()
        }),
        &|_| tr(enabled),
        input,
        &output,
        FixtureTestConfig::default(),
    );
}

#[testing::fixture("tests/fixture/enabled/**/input.js")]
#[allow(
    clippy::needless_pass_by_value,
    reason = "the fixture macro supplies an owned path"
)]
fn fixture_enabled(input: PathBuf) {
    run_fixture(&input, true);
}

#[testing::fixture("tests/fixture/disabled/**/input.js")]
#[allow(
    clippy::needless_pass_by_value,
    reason = "the fixture macro supplies an owned path"
)]
fn fixture_disabled(input: PathBuf) {
    run_fixture(&input, false);
}
