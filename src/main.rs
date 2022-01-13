use std::process::Command;
use rayon::prelude::*;
use std::path::PathBuf;
use glob::glob;
use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    /// file path pattern
    file_path_pattern: String,
    /// target directory
    target_dir: String,
}

fn main() {
    // configure the global thread pool
    // commenting out the below line will by default use all logical cores on CPU
    rayon::ThreadPoolBuilder::new().num_threads(32).build_global().unwrap();
    // parse arguments
    let opts: Opts = Opts::parse();
    let target_dir = PathBuf::from(opts.target_dir);
    // find file with patterns
    let paths = glob(opts.file_path_pattern.as_str()).unwrap()
        .collect::<Vec<Result<_, _>>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    // parallelization
    paths
        .par_iter()
        .for_each(|path| {
            let file_stem = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let mut mkdir_path = target_dir.clone();
            mkdir_path.push(file_stem);
            let dir_path = mkdir_path.to_str().unwrap();
            let mut mkdir_command_exec = Command::new("mkdir")
                .args([dir_path])
                .spawn()
                .unwrap();
            mkdir_command_exec.wait().expect("mkdir failed"); // wait for its completion
            let tar_args = [
                "-xf", path.to_str().unwrap(),
                "--directory", dir_path
            ];
            let mut tar_command_exec = Command::new("tar")
                .args(tar_args)
                .spawn()
                .unwrap();
            tar_command_exec.wait().expect("tar failed"); // wait for its completion
        });
    println!("Finished---------------------");
}
