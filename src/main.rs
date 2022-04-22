use clap::{arg, Command};
use std::path::Path;
use std::io::Read;
use std::fs::File;
use rayon::prelude::*;

fn main() {

    let mut argf = String::new();

    let cmd = Command::new("conm")
        .version("0.1")
        .author("zxq_yx_007@163.com")
        .arg(arg!(-f --file [arg_file]  "file to read args from").required(true));

    let matches = cmd.get_matches();

    if let Some(f) = matches.value_of("file") {
       argf = f.to_string();
    }

    let mut f = File::open(argf).unwrap();

    let mut cont = String::new();
    f.read_to_string(&mut cont);

    let lines: Vec<String> = cont.split('\n').collect();
    lines().par_iter()
    .map(|arg| { println!("{}", arg) })
    
}
