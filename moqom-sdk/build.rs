//! Regenerates the protobuf bindings, on request only.
//!
//! The generated file is checked in and this does nothing unless
//! `MOQOM_REGENERATE_PROTO` is set, which is how `tools/generate-proto.sh` invokes
//! it. A published SDK that generated at build time would require every consumer to
//! have `protoc` installed at a compatible version, which is a poor thing to demand
//! for code that changes when the API does and not otherwise.

fn main() {
    println!("cargo:rerun-if-env-changed=MOQOM_REGENERATE_PROTO");

    if std::env::var_os("MOQOM_REGENERATE_PROTO").is_none() {
        return;
    }

    // The contract lives in the platform repository, which is a different
    // repository to this one: `go get` and `cargo add` both resolve a package by
    // its own URL, so an SDK is only consumable from a repository of its own.
    // `MOQOM_PROTO_ROOT` is how `tools/generate-proto.sh` says where it is
    // calling from; the fallback assumes the two repositories are siblings,
    // which is where they land if you cloned both into the same directory.
    let root = match std::env::var_os("MOQOM_PROTO_ROOT") {
        Some(given) => std::path::PathBuf::from(given),
        None => std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../moqom/proto"),
    }
    .canonicalize()
    .expect(
        "the proto directory is where it is expected: set MOQOM_PROTO_ROOT, \
         or check out moqom-cloud/moqom beside this repository",
    );
    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generated");
    std::fs::create_dir_all(&out).expect("creating the generated directory");

    let files = [
        root.join("moqom/admin/v1/common.proto"),
        root.join("moqom/admin/v1/rooms.proto"),
        root.join("moqom/admin/v1/moderation.proto"),
        root.join("moqom/admin/v1/tokens.proto"),
        root.join("moqom/admin/v1/events.proto"),
    ];
    for file in &files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out)
        .compile_protos(&files, &[root])
        .expect("compiling the admin protos");
}
