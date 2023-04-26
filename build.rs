fn main() {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .compile(&["./protos/types.proto"], &["protos"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .compile(&["./protos/alirs.proto"], &["protos"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));
}
