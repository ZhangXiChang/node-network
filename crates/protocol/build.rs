fn main() -> Result<(), std::io::Error> {
    prost_build::compile_protos(&["./protocol.proto"], &["./"])
}
