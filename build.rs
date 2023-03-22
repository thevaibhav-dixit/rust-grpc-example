// build from the proto file
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("./src")
        .file_descriptor_set_path(out_dir.join("todo_descriptor.bin"))
        .compile(&["proto/todo.proto"], &["proto"])
        .unwrap();
}
