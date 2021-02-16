extern crate tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    let out_dir = "src/proto/out";
    fs::create_dir_all(out_dir)?;
    tonic_build::configure()
        .format(true)
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir)
        .compile(
            &[
                "src/proto/proto/helloworld.proto",
                "src/proto/proto/tinykvpb.proto",
                "src/proto/proto/raft_cmdpb.proto",
                "src/proto/proto/schedulerpb.proto",
            ],
            &["src/proto/include/", "src/proto/proto/"],
        )?;
    Ok(())
}
