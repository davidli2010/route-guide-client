use tonic_build;

fn main() {
    let proto_root = "proto";
    println!("cargo:rerun-if-changed={}", proto_root);

    tonic_build::configure()
        .build_server(false)
        .out_dir("src")
        .type_attribute(
            ".routeguide.Point",
            "#[derive(Copy, Eq, ::serde::Deserialize)]",
        )
        .type_attribute(".routeguide.Feature", "#[derive(::serde::Deserialize)]")
        .type_attribute(
            ".routeguide.FeatureDatabase",
            "#[derive(::serde::Deserialize)]",
        )
        .compile(&["proto/route_guide.proto"], &[proto_root])
        .expect("Failed to compile route_guide.proto");
}
