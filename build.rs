extern crate svd2rust;
extern crate form;

use std::env;
use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(
        env::var_os("OUT_DIR")
            .expect("cannot read OUT_DIR")
    );

    if env::var_os("CARGO_FEATURE_RT").is_some() {
        println!("cargo:rustc-link-search={}", out.display());
    }
    println!("cargo:rerun-if-changed=MK64F12.svd");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let xml = &mut String::new();
    File::open(PathBuf::from("MK64F12.svd"))
        .expect("couldn't open the SVD file")
        .read_to_string(xml)
        .expect("couldn't read the SVD file");

    let gen = svd2rust::generate(&xml, svd2rust::Target::CortexM, false)
        .expect("code generation failed");

    form::create_directory_structure(PathBuf::from("src"), gen.lib_rs)
        .expect("formatting module content failed");

    if let Some(device_specific) = gen.device_specific {
        File::create(out.join("device.x"))
            .expect("cannot create device.c")
            .write_all(device_specific.device_x.as_bytes())
            .expect("cannot write device.x");
    }
}
