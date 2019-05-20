use pyo3::prelude::*;
use sprs::CsVec;
use std::collections::HashMap;
use std::ops::Add;

#[pyclass]
#[derive(Debug)]
struct KmerVec {
    rep: CsVec<f32>,
}

fn base2num(c: char) -> usize {
    match c {
        'A' => 0,
        'C' => 1,
        'T' => 2,
        'G' => 3,
        _ => panic!("unexpected base pair: {}", c),
    }
}

fn encode(coll: &str) -> usize {
    let mut v: usize = 0;
    for (i, c) in coll.chars().enumerate() {
        v += 4usize.pow(i as u32) * base2num(c)
    }
    v
}

fn build(coll: &str, k: u32) -> KmerVec {
    let n = coll.len() as u32;
    let mut counter = HashMap::new();
    for i in 0..=n - k {
        let key = encode(&coll[(i as usize)..((i + k) as usize)]);
        *counter.entry(key).or_insert(0) += 1;
    }
    let mut keys: Vec<usize> = vec![];
    let mut vals: Vec<f32> = vec![];
    let mut tt_mag: f32 = 0.0;
    for (_, v) in counter.iter() {
        tt_mag += ((*v) as i32).pow(2) as f32;
    }
    tt_mag = tt_mag.sqrt();
    for (k, v) in counter.iter() {
        keys.push(*k);
        vals.push((*v) as f32 / tt_mag);
    }
    let rep = CsVec::new(4u32.pow(k) as usize, keys, vals);
    KmerVec { rep: rep }
}

#[pymethods]
impl KmerVec {
    #[new]
    fn new(obj: &PyRawObject, coll: &str, k: i32) {
        obj.init(build(coll, k as u32));
    }

    // fn pure(k : u32) -> KmerVec {
    //     KmerVec {rep: CsVec::empty(4u32.pow(k) as usize)}
    // }

    fn similarity(&self, rhs: &KmerVec) -> f32 {
        self.rep.dot(&rhs.rep)
    }

    fn similarity_dist(&self, rhs: &KmerVec) -> f32 {
        1.0 - self.similarity(rhs)
    }
}

// impl Add<KmerVec> for KmerVec {
//     type Output = KmerVec;

//     fn add(self, rhs: KmerVec) -> KmerVec {
//         KmerVec {
//             rep: (self.rep + rhs.rep)
//         }
//     }
// }

#[pymodule]
fn kmer_vec(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KmerVec>()
}


#[test]
fn main() {
    // let kmer0 = build("ACTG", 1);
    // let kmer2 = build("AGTGTGCATCG", 1);
    // let kmer3 = build("AAAAAAAAAAA", 1);
    // println!("{:?}", encode("A"));
    // println!("{:?}", kmer0.similarity(&kmer3));
    // println!("{:?}", kmer0.similarity(&kmer0));
}
