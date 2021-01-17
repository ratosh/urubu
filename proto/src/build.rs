extern crate protoc_rust;

use protoc_rust::Customize;

use std::io;

fn main() -> io::Result<()> {
    // protoc_rust::Codegen::new()
    //     .out_dir("src/protos")
    //     .inputs(&["src/protos/training_chunk.proto"])
    //     .include("protos")
    //     .run()
    // .expect("Running protoc failed.");
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/protos",
        input: &["src/protos/network.proto"],
        includes: &["src/protos"],
        customize: Customize {
            carllerche_bytes_for_bytes: Some(true),
            carllerche_bytes_for_string: Some(true),
            ..Default::default()
        },
    })
        .expect("Running protoc failed.");
    Ok(())
}
