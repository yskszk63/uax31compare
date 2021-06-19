use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::Command;
use std::env;
use std::io::{BufReader, BufRead};

struct Rust;
impl Rust {
    fn test(&self, c: &str) -> bool {
        let name = "work/rust.rs";
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(name).unwrap();
        let prog = format!(include_str!("template/rust.rs"), c);
        file.write_all(prog.as_bytes()).unwrap();
        Command::new("rustc").arg(name).arg("-o").arg("work/rust").status().unwrap().success()
    }
}

struct Go;

impl Go {
    fn test(&self, c: &str) -> bool {
        let name = "work/go.go";
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(name).unwrap();
        let prog = format!(include_str!("template/go.go"), c);
        file.write_all(prog.as_bytes()).unwrap();
        Command::new("go").arg("build").arg("-o").arg("work/go").arg(name).status().unwrap().success()
    }
}

enum Lang {
    Rust(Rust),
    Go(Go),
}

impl Lang {
    fn name(&self) -> &'static str {
        match self {
            Self::Rust(..) => "rust",
            Self::Go(..) => "go",
        }
    }

    fn test(&self, c: &str) -> bool {
        match self {
            Self::Rust(lang) => lang.test(c),
            Self::Go(lang) => lang.test(c),
        }
    }
}

fn main() {
    let input = env::args().nth(1).unwrap();

    let langs = [
        Lang::Rust(Rust),
        Lang::Go(Go),
    ];

    for line in BufReader::new(File::open(input).unwrap()).lines() {
        let line = line.unwrap();
        for lang in &langs {
            let ok = lang.test(&line);
            println!("{}\t{}\t{:?}", line, lang.name(), ok);
        }
    }
}
