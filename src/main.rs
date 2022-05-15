use clap::{arg, Command};
use std::io::{Read, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use rayon::prelude::*;
use std::process;
use rand::distributions::Alphanumeric;
use rand::Rng;
use thread_id;
use std::path::Path;

fn main() {

    let mut argf = String::new();
    let mut binf = String::new();
    let mut earg = String::new();

    let cmd = Command::new("conm")
        .version("0.1")
        .author("zxq_yx_007@163.com")
        .arg(arg!(-b --bin  <bin_file>  "binary file to execute").required(true))
        .arg(arg!(-a --args [extra_args]   "extra args passwd to").required(false).allow_hyphen_values(true))
        .arg(arg!(-f --file <arg_file>  "file to read args from").required(true));

    let matches = cmd.get_matches();

    if let Some(f) = matches.value_of("file") {
        argf = f.to_string();
    }

    if let Some(b) = matches.value_of("bin") {
        binf = b.to_string();
    }

    if let Some(e) = matches.value_of("args") {
        earg = e.to_string();
    }

    let mut f = File::open(argf).unwrap();

    let mut cont = String::new();
    f.read_to_string(&mut cont).unwrap();

    let lines:Vec<String> = cont.split('\n').map(String::from).collect();
    let randpath: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();
    let output_dir = format!("output-{}", randpath);
    println!("Output Dir: {}\n", &output_dir);
    let logdir = Path::new(&output_dir);
    if ! logdir.exists() {
        fs::create_dir(&output_dir).unwrap();
    }

    lines.par_iter()
    .for_each(|arg| {
        let m = arg.trim();
        let mut exec = format!("{}", &binf);
        if m != "" {
            let cmd = format!("{} {} {}", &binf, m, &earg);
            println!("Executing: {}", cmd);

            let mut cmds: Vec<String> = vec![];
            if binf.find(char::is_whitespace) != None {
                exec = binf.split_whitespace().nth(0).unwrap().to_string();
                let bs: Vec<&str> = binf.split_whitespace().collect();
                for s in bs {
                    if s == exec {
                        continue;
                    }
                    cmds.push(s.to_string());
                }
            }

            cmds.push(m.to_string());
            if earg != "" {
                let eargs: Vec<&str> = earg.split_whitespace().collect();
                let mut qs = String::new();
                for s in eargs {
                    if s.starts_with('"') {
                        qs = s[1..].to_string()
                    } else if s.ends_with('"') {
                        qs = qs + " " + &s[..s.len()-1];
                        cmds.push(qs);
                        qs = "".to_string();
                        continue;
                    } else if qs != "" {
                        qs = qs + " " + s;
                    } else {
                        cmds.push(s.to_string());
                    }
                }
            }
    
            let tid = thread_id::get();
            let logf = format!("{}/{}-{}-{}.log", &output_dir, &exec, m, &tid);

            let mut lf = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&logf).unwrap();

            let output = process::Command::new(&exec)
                                .args(cmds)
                                .output()
                                .expect("failed to execute!");
            let ret = format!("{}\n\n", output.status);
            lf.write_all(&ret.as_bytes()).unwrap();
            lf.write_all(b"STDOUT:\n\n").unwrap();
            lf.write_all(&output.stdout).unwrap();
            lf.write_all(b"\nSTDERR:\n\n").unwrap();
            lf.write_all(&output.stderr).unwrap();
        }
    });

    println!("\nAll done!");
}
