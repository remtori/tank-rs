use std::{fs, path::Path};

use npm_rs::{NodeEnv, NpmEnv};
use protobuf_codegen_pure::Customize;

const INCLUDE_DIR: &'static str = "schema";
const OUTPUT_DIR: &'static str = "generated";

const INPUTS: &[&'static str] = &["schema/game.proto"];

#[cfg(target_os = "windows")]
const TS_PROTOC_PLUGIN: &'static str = "protoc-gen-ts_proto=.\\node_modules\\.bin\\protoc-gen-ts_proto.cmd";

#[cfg(not(target_os = "windows"))]
const TS_PROTOC_PLUGIN: &'static str = "node_modules/.bin/protoc-gen-ts_proto";

fn main() {
    let rust_out_dir = format!("{}/rust", OUTPUT_DIR);
    let typescript_out_dir = format!("{}/typescript", OUTPUT_DIR);

    println!("cargo:rerun-if-changed={}", INCLUDE_DIR);

    // Refresh generated dir
    if Path::new(OUTPUT_DIR).exists() {
        fs::remove_dir_all(OUTPUT_DIR).unwrap();
    }

    fs::create_dir(OUTPUT_DIR).unwrap();
    fs::create_dir(&rust_out_dir).unwrap();
    fs::create_dir(&typescript_out_dir).unwrap();

    // protobuf rust code gen
    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(rust_out_dir)
        .inputs(INPUTS)
        .include(INCLUDE_DIR)
        .run()
        .unwrap();

    // Ensure we has run npm install ts-proto
    if !Path::new(TS_PROTOC_PLUGIN).exists() {
        let status = NpmEnv::default()
            .with_node_env(&NodeEnv::Production)
            .init_env()
            .install(None)
            .exec()
            .unwrap();

        if !status.success() {
            panic!("npm install failed");
        }
    }

    // protobuf typescript code gen
    protoc::ProtocLangOut::new()
        .lang("ts_proto")
        .plugin(TS_PROTOC_PLUGIN)
        .out_dir(typescript_out_dir)
        .inputs(INPUTS)
        .include(INCLUDE_DIR)
        .run()
        .unwrap();
}
