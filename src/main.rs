#[macro_use]
extern crate r2pipe;
#[macro_use]
extern crate clap;

use r2pipe::R2Pipe;
use std::fs::File;
use std::io::Write;

enum Format {
    Dot,
    Json,
    Gml,
}

impl Format {
    pub fn new(f: String) -> Self {
        use Format::*;
        match &f[..] {
            "dot" => Dot,
            "json" => Json,
            "gml" => Gml,
            _ => unreachable!(),
        }
    }

    pub fn to_cmd(&self) -> &'static str {
        use Format::*;
        match self {
            Dot => "agCd",
            Json => "agCj",
            Gml => "agCg",
        }
    }

    pub fn extention(&self) -> &'static str {
        use Format::*;
        match self {
            Dot => ".dot",
            Json => ".json",
            Gml => ".gml",
        }
    }
}

fn main() {
    let yml = load_yaml!("cmd.yml");
    let args = clap::App::from_yaml(yml).get_matches();

    let format = Format::new(String::from(args.value_of("format").unwrap()).to_lowercase());
    let binfile = args.value_of("binfile");
    
    let mut pipe = open_pipe!(binfile).unwrap();
    let cfg = gen_cfg(&mut pipe, &format);

    let mut out = match args.value_of("out") {
        Some(path) => File::create(path).unwrap(),
        None => File::create(format!("{}{}", binfile.unwrap(), format.extention())).unwrap(),
    };

    write!(out, "{}", cfg).unwrap();
}

fn gen_cfg(pipe: &mut R2Pipe, fmt: &Format) -> String {
    pipe.cmd("aa").unwrap();
    pipe.cmd(fmt.to_cmd()).unwrap()
}
