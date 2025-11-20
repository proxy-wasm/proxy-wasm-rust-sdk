use std::fs;

fn main() {
    let out_dir = "src/generated";
    fs::create_dir_all(out_dir).unwrap();
    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&["src/set_envoy_filter_state.proto"], &["src/"])
        .unwrap();
    println!("cargo:rerun-if-changed=src/set_envoy_filter_state.proto");
}
