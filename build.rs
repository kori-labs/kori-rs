fn main() {
    tonic_build::compile_protos("protos/auth.proto").unwrap();
    tonic_build::compile_protos("protos/searcher.proto").unwrap();
    tonic_build::compile_protos("protos/dto.proto").unwrap();
    tonic_build::compile_protos("protos/block_engine.proto").unwrap();
}
