extern crate anyhow;
extern crate fuzz_targets;
extern crate proptest_helper;
extern crate crypto;
extern crate hex;

use std::{fmt, ops::Deref, str::FromStr};
use std::{
    env,
    ffi::OsString,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use self::anyhow::{anyhow,bail, Result, Error, Context};
use self::proptest_helper::SeedGen;
use self::fuzz_targets::*;
use self::crypto::digest::Digest;


pub fn gen_corpus(target: &str, corpus_dir: PathBuf, num_items: usize) -> Result<PathBuf> {
    let corpus_dir = create_dir(&target, corpus_dir)?;
    let mut gen = SeedGen::new();

    let mut idx :usize = 0;
    while idx < num_items {
        let seeds = fuzz_mmr_push_bytes_seeds(&mut gen);
        println!("{:?}", seeds);
        let name = hex::encode(&seeds);
        let filename = corpus_dir.join(name);
        let mut f = fs::File::create(&filename).with_context(|| format!("Failed to create file: {:?}",filename))?;
        f.write_all(&seeds).with_context(|| format!("Failed to write to file: {:?}", filename))?;
        println!("Saving seed at {:?}", filename);
        idx += 1;
    }
    Ok(corpus_dir)

}


fn create_dir(target: &str, corpus_dir: PathBuf) -> Result<(PathBuf)> {
    let custom_dir = corpus_dir.join(target);
    if custom_dir.exists() {
        println!("Directory exits, check if corpus already exist or use -d for custom directory.");
    }

    println!("Saving corpus at {:?}", &custom_dir);
    fs::create_dir_all(&custom_dir).expect("Failed to create a corpus directory");
    Ok(custom_dir)


}

pub fn run_libfuzzer(target: String, corpus_dir: PathBuf, artifacts_dir: PathBuf, num_items: usize) -> Result<()> {
    println!("Hello from libfuzzer");

    let custom_corpus = gen_corpus(&target,corpus_dir,num_items);
    let targets = Command::new("cargo")
        .arg("fuzz")
        .arg("run")
        .arg(&target)
        .args(&custom_corpus)
        .status()
        .context("cargo fuzz run target")?;
    if !targets.success() {
        bail!("cargo fuzz run failed with status {}", targets);
    }
    Ok(())

}

pub fn run_afl() -> Result<()> {
    println!("Hello from afl fuzz");
    Ok(())
}

pub fn run_hfuzz() -> Result<()> {
    println!("Hello from hfuzz");
    Ok(())
}
