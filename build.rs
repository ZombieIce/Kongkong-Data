fn main() {
    tonic_build::configure().compile(&["proto/data.proto"], &["proto"]).unwrap();
}
