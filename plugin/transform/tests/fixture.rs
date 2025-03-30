use std::path::PathBuf;

use swc_dbg::dbg;
use swc_ecma_parser::{Syntax, TsSyntax};
use swc_ecma_transforms_testing::test_fixture;

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let filename = input.to_string_lossy();
    let output = input.with_file_name("output.js");

    test_fixture(
        Syntax::Typescript(TsSyntax {
            tsx: filename.ends_with(".tsx"),
            ..Default::default()
        }),
        &|_| dbg(),
        &input,
        &output,
        Default::default(),
    );
}
