use criterion::{criterion_group, criterion_main, Criterion};

use buffer_benchmarks::{capnproto, flatbuffers, protobuf};

fn bench_protobuf(c: &mut Criterion) {
    c.bench_function("encode_protobuf", |b| b.iter(|| protobuf::encode()));
    let buf = protobuf::encode();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_protobuf", |b| b.iter(|| protobuf::decode(&buf)));
}

fn bench_flatbuffers(c: &mut Criterion) {
    c.bench_function("encode_flatbuffers", |b| b.iter(|| flatbuffers::encode()));
    let buf = flatbuffers::encode();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_flatbuffers", |b| {
        b.iter(|| flatbuffers::decode(&buf))
    });
}

fn bench_capnproto(c: &mut Criterion) {
    c.bench_function("encode_capnproto", |b| b.iter(|| capnproto::encode()));
    let buf = capnproto::encode();
    println!("Wire format size (bytes) = {}", buf.len());
    c.bench_function("decode_capnproto", |b| b.iter(|| capnproto::decode(&buf)));
}

criterion_group!(benches, bench_protobuf, bench_flatbuffers, bench_capnproto);
criterion_main!(benches);
