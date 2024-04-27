use std::fmt::Error;

fn main() -> Result<(), Error> {

    tonic_build::configure()
    .compile(
        &["api.proto"], // Files in the path
        &["protos"], // The path to search
    )
    .unwrap();

    Ok(())
}