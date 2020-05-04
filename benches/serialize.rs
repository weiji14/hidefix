#![feature(test)]
extern crate test;
use test::Bencher;

use hidefix::idx::Index;

mod serde_bincode {
    use super::*;

    #[bench]
    fn serialize_coads(b: &mut Bencher) {
        let i = Index::index("tests/data/coads_climatology.nc4").unwrap();

        b.iter(|| bincode::serialize(&i).unwrap())
    }

    #[bench]
    fn deserialize_coads(b: &mut Bencher) {
        let i = Index::index("tests/data/coads_climatology.nc4").unwrap();
        let bb = bincode::serialize(&i).unwrap();

        b.iter(|| bincode::deserialize::<Index>(&bb).unwrap())
    }

    #[bench]
    fn serialize_coads_file(b: &mut Bencher) {
        let i = Index::index("tests/data/coads_climatology.nc4").unwrap();

        b.iter(|| {
            let f = std::fs::File::create("/tmp/coads.idx.bc").unwrap();
            let w = std::io::BufWriter::new(f);
            bincode::serialize_into(w, &i).unwrap()
        })
    }

    #[bench]
    fn deserialize_coads_file(b: &mut Bencher) {
        let i = Index::index("tests/data/coads_climatology.nc4").unwrap();
        let f = std::fs::File::create("/tmp/coads.idx.bc").unwrap();
        bincode::serialize_into(f, &i).unwrap();

        b.iter(|| {
            let f = std::fs::File::open("/tmp/coads.idx.bc").unwrap();
            let r = std::io::BufReader::new(f);
            bincode::deserialize_from::<_, Index>(r).unwrap()
        })
    }

    #[ignore]
    #[bench]
    fn serialize_meps_bincode(b: &mut Bencher) {
        let i = Index::index("../data/meps_det_vc_2_5km_latest.nc").unwrap();

        b.iter(|| bincode::serialize(&i).unwrap())
    }

    #[ignore]
    #[bench]
    fn deserialize_meps_bincode(b: &mut Bencher) {
        let i = Index::index("../data/meps_det_vc_2_5km_latest.nc").unwrap();
        let bb = bincode::serialize(&i).unwrap();

        b.iter(|| bincode::deserialize::<Index>(&bb).unwrap())
    }

    #[ignore]
    #[bench]
    fn serialize_meps_bincode_file(b: &mut Bencher) {
        let i = Index::index("../data/meps_det_vc_2_5km_latest.nc").unwrap();

        b.iter(|| {
            let f = std::fs::File::create("/tmp/meps.idx.bc").unwrap();
            let w = std::io::BufWriter::new(f);
            bincode::serialize_into(w, &i).unwrap()
        })
    }

    #[ignore]
    #[bench]
    fn deserialize_meps_bincode_file(b: &mut Bencher) {
        let i = Index::index("../data/meps_det_vc_2_5km_latest.nc").unwrap();
        let f = std::fs::File::create("/tmp/meps.idx.bc").unwrap();
        bincode::serialize_into(f, &i).unwrap();

        b.iter(|| {
            let f = std::fs::File::open("/tmp/meps.idx.bc").unwrap();
            let r = std::io::BufReader::new(f);
            bincode::deserialize_from::<_, Index>(r).unwrap()
        })
    }
}
