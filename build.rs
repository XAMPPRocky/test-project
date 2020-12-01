fn main() {
    cargo_sysroot::SysrootBuilder::new(cargo_sysroot::Sysroot::CompilerBuiltins)
        .target("wasm32-unknown-unknown".into())
        .output(std::env::var("OUT_DIR").unwrap().into())
        .build()
        .unwrap();
}
