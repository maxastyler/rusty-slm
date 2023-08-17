use anyhow::*;

fn main() -> Result<()> {
    tonic_build::compile_protos("protos/rusty_slm/slm.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    Ok(())
}
