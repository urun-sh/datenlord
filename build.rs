fn main() {
    // grpcio depends on cmake, g++ and protoc,
    // run the following command to install:
    // `sudo apt install cmake g++ libprotobuf-dev protobuf-compiler`
    protoc_grpcio::compile_grpc_protos(
        &[
            "./src/csi/proto/csi.proto",
            "./src/csi/proto/datenlord_worker.proto",
        ], // inputs
        &["./src/csi/proto"], // includes
        "src/csi/proto",      // output
        None,                 // customizations
    )
    .unwrap_or_else(|e| panic!("Failed to compile gRPC definitions, the error is: {e}"));

    // rust-protobuf 2.x emits `#![allow(box_pointers)]` in the generated
    // files. The `box_pointers` lint has been removed from rustc and merely
    // referencing it now triggers the `removed_lints` diagnostic, which the
    // crate-level `#![deny(warnings)]` turns into a hard error. Strip the
    // attribute after codegen.
    for file in [
        "csi.rs",
        "csi_grpc.rs",
        "datenlord_worker.rs",
        "datenlord_worker_grpc.rs",
    ] {
        let path = std::path::Path::new("src/csi/proto").join(file);
        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Failed to read generated file {path:?}, the error is: {e}")
        });
        let stripped = content
            .lines()
            .filter(|l| !l.contains("box_pointers"))
            .collect::<Vec<&str>>()
            .join("\n");
        std::fs::write(&path, format!("{stripped}\n")).unwrap_or_else(|e| {
            panic!("Failed to write generated file {path:?}, the error is: {e}")
        });
    }
}
