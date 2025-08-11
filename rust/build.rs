use std::io::Result;

fn main() -> Result<()> {
    protobuf_codegen::Codegen::new()
        .protoc()
        .includes(&["../schemas"])
        .input("../schemas/message.proto")
        .cargo_out_dir("protobuf")
        .run_from_script();

    capnpc::CompilerCommand::new()
        .file("../schemas/message-rust.capnp")
        .src_prefix("../schemas")
        .run()
        .expect("compiling schema");

    Ok(())
}
