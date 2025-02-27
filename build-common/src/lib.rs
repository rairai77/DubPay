use std::fs;

pub fn compile_protos(proto_dir: &str) {
    let proto_files: Vec<String> = fs::read_dir(proto_dir)
        .expect("Failed to read proto directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "proto" {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    if proto_files.is_empty() {
        panic!("No .proto files found in directory: {}", proto_dir);
    }

    tonic_build::configure()
        .compile_protos(
            &proto_files.iter().map(String::as_str).collect::<Vec<_>>(),
            &[proto_dir],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));
}
