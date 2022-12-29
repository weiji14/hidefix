#![feature(test)]
extern crate test;
use hidefix::prelude::*;
use hidefix::reader::uring::*;
use hidefix::idx::DatasetD;
use test::Bencher;

#[bench]
fn coads(b: &mut Bencher) {
    let i = Index::index("tests/data/coads_climatology.nc4").unwrap();
    let ds = if let DatasetD::D3(ds) = i.dataset("SST").unwrap() {
        ds
    } else {
        panic!()
    };
    let mut r = UringReader::with_dataset(ds, i.path().unwrap()).unwrap();


    b.iter(|| r.values::<f32>(None, None).unwrap())
}

#[bench]
fn coads_uring(b: &mut Bencher) {
    let i = Index::index("tests/data/coads_climatology.nc4").unwrap();
    let ds = if let DatasetD::D3(ds) = i.dataset("SST").unwrap() {
        ds
    } else {
        panic!()
    };
    let r = UringReader::with_dataset(ds, i.path().unwrap()).unwrap();


    b.iter(|| tokio_uring::start(async {r.values_uring::<f32>(None, None).await.unwrap() }));
}