use spirv_builder::{options::{Source, SourceKind}, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    SpirvBuilder::new("./shader", &out_dir)
        // Currently `librustc_codegen_spirv` and the `spirv-unknown-unknown`
        // sysroot are not packaged with Rust, so we have to build it ourselves.
        // See `Source` for more information on different ways to provide or
        // compile the sources.
        .codegen_source(Source {
            compile_source: true,
            kind: SourceKind::Git {
                repository: "https://github.com/EmbarkStudios/rust-gpu".into(),
                into: out_dir.join("rust-gpu"),
                commitish: None,
            }
        })
        .sysroot_location(Source {
            compile_source: true,
            kind: SourceKind::Path(out_dir.join("sysroot"))
        })
        // Emit cargo build script metadata for the SPIR-V project.
        .emit_build_script_metadata()
        .build()?;
    Ok(())
}
