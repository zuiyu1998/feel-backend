use std::process::Command;

pub trait BuilderExt {
    fn with_serde(self, path: &[&str]) -> Self;
    fn with_hash_and_eq(self, path: &[&str]) -> Self;
}

impl BuilderExt for tonic_build::Builder {
    fn with_serde(self, path: &[&str]) -> Self {
        path.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(serde::Serialize, serde::Deserialize)]")
        })
    }

    fn with_hash_and_eq(self, path: &[&str]) -> Self {
        path.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(Hash, Eq)]")
        })
    }
}

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_serde(&["RegisterUserReq", "UserLoginReq"])
        .with_hash_and_eq(&[])
        .compile_protos(&["protos/feel_sdk.proto"], &["protos"])
        .unwrap();

    // execute cargo fmt command
    Command::new("cargo").arg("fmt").output().unwrap();

    println!("cargo: rerun-if-changed=abi/protos/feel_sdk.proto");
}
