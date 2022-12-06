use swift_rs::build;

fn link_native() {
  use std::{fs, path::Path};

  let swift_lib_dir = fs::canonicalize(Path::new("./swift-lib/")).unwrap();
  build::link_swift("10.15");
  build::link_swift_package("swift-lib", 
    &format!("{}/", swift_lib_dir.to_str().unwrap().to_string()));
}

fn main() {
  link_native();
  tauri_build::build();
}
