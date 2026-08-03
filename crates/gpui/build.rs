#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        #[cfg(feature = "windows-manifest")]
        embed_resource();
    }
}

#[cfg(feature = "windows-manifest")]
fn embed_resource() {
    let root = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let manifest = root.join("resources/windows/gpui.manifest.xml");
    println!("cargo:rerun-if-changed={}", manifest.display());
    // The resource script is generated into OUT_DIR with an absolute manifest
    // path: resource compilers resolve paths inside a script against their own
    // working directory, and llvm-rc runs on embed-resource's preprocessed
    // copy in OUT_DIR, where the checked-in script's relative path dangles.
    let rc_file = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("gpui.rc");
    let path = manifest.display().to_string().replace('\\', "\\\\");
    std::fs::write(
        &rc_file,
        format!("#define RT_MANIFEST 24\n1 RT_MANIFEST \"{path}\"\n"),
    )
    .unwrap();
    embed_resource::compile(&rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
