use std::{fs, path::Path};

use protobuf_codegen_pure::Customize;

const INCLUDE_DIR: &'static str = "schema";
const OUTPUT_DIR: &'static str = "generated";

const INPUTS: &[&'static str] = &["schema/game.proto"];

fn main() {
    if Path::new(OUTPUT_DIR).exists() {
        fs::remove_dir_all(OUTPUT_DIR).unwrap();
    }
    fs::create_dir(OUTPUT_DIR).unwrap();

    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(OUTPUT_DIR)
        .inputs(INPUTS)
        .include(INCLUDE_DIR)
        .run()
        .unwrap();
}
