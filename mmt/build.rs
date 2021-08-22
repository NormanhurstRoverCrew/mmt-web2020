fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/email.proto");
    tonic_build::compile_protos("../proto/email.proto")?;
    Ok(())
}
