fn main() {
    prost_build::compile_protos(&["src/GUIProt0.proto"], &["src"]).unwrap();
}
