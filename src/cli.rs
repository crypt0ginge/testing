extern crate once_cell;
extern crate structopt;
extern crate fuzz_targets;

use structopt::StructOpt;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::ffi::OsString;
use std::str::FromStr;
use std::string::ParseError;
use fuzz_targets::list_targets;

mod runner;

#[derive(Debug, StructOpt)]
#[structopt(name = "fuzzer", about = "Tari fuzzer")]
struct Cli {
    /// Print debug message
    #[structopt(long = "debug")]
    debug: bool,
    #[structopt(subcommand)]
    cmd: Run,
}
//Constants for our defaults
const  ENGINE_DEFAULT_RUN: &str = "libfuzzer";
const CORPUS_DEFAULT_DIR: &str = "./common/corpus";
const ARTIFACT_DEFAULT_DIR: &str = "./common/artifact";
const  CORPUS_ITEMS:  usize = 128;
static CORPUS_ITEMS_STR: Lazy<String> = Lazy::new(|| CORPUS_ITEMS.to_string());

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Fuzzer {
    AFL,
    Hongfuzz,
    Libfuzzer,
}

impl FromStr for Fuzzer {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AFL" | "afl"=> Ok(Fuzzer::AFL),
            "hongfuzz" | "hfuzz" => Ok(Fuzzer::Hongfuzz),
            _ => Ok(Fuzzer::Libfuzzer),

        }
    }
}

#[derive(Debug, StructOpt)]
enum Run {
    ///Generate corpus with proptest
    #[structopt(name = "corpus", usage = "fuzzer corpus")]
    Corpus {
        /// Number of items to generate in the corpus
        #[structopt(
        short = "n",
        long = "num_items",
        default_value = &CORPUS_ITEMS_STR,
        )]
        num_items: usize,
        /// Directory for the corpus, optional.
        /// Default is "./common/corpus/target
        #[structopt(long = "corpus_dir", short = "dir", default_value = &CORPUS_DEFAULT_DIR)]
        corpus_dir: PathBuf,
        #[structopt(name = "TARGET")]
        /// Fuzz target for the corpus to be generated
        target: String,

    },
    /// Run fuzzer on specified target
    #[structopt(name = "fuzz", usage = "fuzzer fuzz <TARGET> -- [ARGS]")]
    Fuzz {
        /// Target to fuzz (use `targets` to list targets)
        #[structopt(name = "TARGET", required = true)]
        target: String,
        /// Engine to use (use `engines` to list them)
        #[structopt(short = "engine", default_value=&ENGINE_DEFAULT_RUN)]
        engine: Fuzzer,
        /// Custom directory for corpus
        #[structopt(long = "corpus_dir", default_value = &CORPUS_DEFAULT_DIR, parse(from_os_str))]
        corpus_dir: PathBuf,
        /// Custom directory for artifacts
        #[structopt(long = "artifact_dir",default_value = &ARTIFACT_DEFAULT_DIR, parse(from_os_str))]
        artifact_dir: PathBuf,
        /// Number of items to generate in the corpus
        #[structopt(
        short = "n",
        long = "num_items",
        default_value = &CORPUS_ITEMS_STR,
        )]
        num_items: usize,
    },
    /// List available engines
    #[structopt(name = "engines")]
    ListEngines {
    },
    /// List fuzz targets
    #[structopt(name = "targets")]
    ListTargets {

    },

}

fn main() {
    let cli: Cli = Cli::from_args();
    match cli.cmd {
        Run::Corpus {
            num_items,
            corpus_dir,
            target,
        } => {
            println!("Wrote {:?} items to corpus", corpus_dir);
            println!("Wrote {:?} items to corpus", num_items);
            println!("Target {:?} is chosen", target);
            runner::gen_corpus(&target, corpus_dir, num_items);
        }
        Run::Fuzz {
            corpus_dir,
            artifact_dir,
            target,
            engine,
            num_items,
        } => {
            //check if valid target
            Some(fuzz_targets::get_target(&target));

            //generating corpus
            println!("Fuzz test {:?}", target);
            match engine {
                Fuzzer::AFL => runner::run_afl(),
                Fuzzer::Hongfuzz => runner::run_hfuzz(),
                Fuzzer::Libfuzzer => runner::run_libfuzzer(target, corpus_dir, artifact_dir, num_items),
            };
        }

        //List available engines
        Run::ListEngines {} => {
            println!("Current supported engines are (1) {:?} (2) {:?} (3) {:?}",
                     Fuzzer::AFL, Fuzzer::Libfuzzer, Fuzzer::Hongfuzz);
        }
        //List available targets
        Run::ListTargets {}=> {
            println!("List targets 1234");
            fuzz_targets::list_targets();
        }
    }
}

