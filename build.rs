fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/transaction.proto"
            ],
            &["proto"],
        )
        .expect("Failed to compile Protobuf files");
}