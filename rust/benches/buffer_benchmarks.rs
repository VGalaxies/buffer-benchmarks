use buffer_benchmarks::{capnproto, flatbuffers, protobuf};
use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use std::hint::black_box;

#[library_benchmark]
fn encode_protobuf() {
    black_box(protobuf::encode());
}

#[library_benchmark]
fn decode_protobuf() {
    let buf = protobuf::encode();
    black_box(protobuf::decode(&buf));
}

#[library_benchmark]
fn encode_flatbuffers() {
    black_box(flatbuffers::encode());
}

#[library_benchmark]
fn decode_flatbuffers() {
    let buf = flatbuffers::encode();
    black_box(flatbuffers::decode(&buf));
}

#[library_benchmark]
fn encode_capnproto() {
    black_box(capnproto::encode());
}

#[library_benchmark]
fn decode_capnproto() {
    let buf = capnproto::encode();
    black_box(capnproto::decode(&buf));
}

library_benchmark_group!(
    name = serialization_benches;
    benchmarks =
        encode_protobuf,
        decode_protobuf,
        encode_flatbuffers,
        decode_flatbuffers,
        encode_capnproto,
        decode_capnproto
);

main!(library_benchmark_groups = serialization_benches);
