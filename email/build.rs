fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto/email.prot");
    tonic_build::compile_protos("../proto/email.proto")?;
    Ok(())
}
