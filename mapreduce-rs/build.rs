fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["src/proto/rpc.proto"], &["src/proto"])?;
    Ok(())
}
