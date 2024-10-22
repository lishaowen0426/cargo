#![allow(unused_imports)]
use cargo_test_support::main_file;
use cargo_test_support::prelude::*;
use cargo_test_support::project;
use cargo_test_support::str;
use tracing::subscriber::set_global_default;

#[ctor::ctor]
fn init() {
    let subs = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    set_global_default(subs).expect("set global default subscriber failed");
}

#[cargo_test]
fn simple_isolate() {
    tracing::debug!("hello");
    let p = project()
        .file(
            "Cargo.toml",
            r#"
                [package]
                name = "foo"
                version = "0.0.1"
                authors = []
                license = "MIT"
                description = "foo"

                [dependencies]
                time = "0.1.12"

                [isolation]
                time = {}

        "#,
        )
        .file("src/main.rs", &main_file("foo", &[]))
        .build();

    p.cargo("metadata --format-version 1 --filter-platform x86_64-unknown-linux-gnu")
        .stream()
        .run();
}
