use std::process::Command;

fn main() {
    let mut config = prost_build::Config::new();

    config.bytes(["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Command::new("cargo")
        .args(["fmt", "--", "src/*.rs"])
        .status()
        .expect("cmd fmt failed");

    println!("cargo:return-if-changed=build.rs");
    println!("cargo:return-if-changed=abi.proto");
}
