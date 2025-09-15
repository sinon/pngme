# A list of available rules and their signatures can be found here: https://buck2.build/docs/prelude/globals/

rust_binary(
    name = "cli",
    srcs = glob(["crates/pngme-cli/src/*.rs"]),
    crate_root = "crates/pngme-cli/src/main.rs",
    deps = [
        "root//crates/pngme:library",
        "//third-party/rust:clap",
    ],
)