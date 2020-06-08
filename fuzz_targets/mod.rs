extern crate proptest_helper;
extern crate proptest;
extern crate tari_crypto;
extern crate digest;
extern crate tari_mmr;
extern crate lazy_static;

use lazy_static::lazy_static;

pub mod fuzz_mmr;

lazy_static! {
    #[derive(Debug, Copy, Clone)]
    pub static ref TARGETS: Vec<Box< &'static str>> = vec![
    // List fuzz targets here in this format.
    Box::new("fuzz_mmr_push_bytes"),
    Box::new("bar"),
];
    static ref COUNT: usize = TARGETS.len();

}

pub fn list_targets() {
    for target in &*TARGETS {
        println!("{:?}", target)
    }
}

pub fn get_target(target: &str) -> Results<()> {
    if TARGETS.iter().find(|xy| *xy == target).is_none() {
        panic!(
            "Could not find the target {}, please use valid/available target", target
        );
    }
}