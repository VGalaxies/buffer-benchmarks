mod constants;

pub mod capnproto;
pub mod flatbuffers;
pub mod protobuf;

mod message_rust_capnp {
    include!(concat!(env!("OUT_DIR"), "/message_rust_capnp.rs"));
}
