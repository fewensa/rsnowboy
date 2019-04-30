extern crate cc;
//extern crate bindgen;

use std::path::PathBuf;

fn main() {
//  println!("cargo:rustc-link-lib=static=stdc++");
//  println!("cargo:rustc-link-lib=gslcblas");


  cc::Build::new()
    .cpp(true)
    .file("rsnowboywrapper/rsnowboy.cpp")
    .include("rsnowboywrapper")
    .flag("-c")
    .flag("-lstdc++")
    .flag("-D_GLIBCXX_USE_CXX11_ABI=0")
    .flag("-fPIC")
    .flag("-std=c++0x")
    .flag("-Wall")
    .flag("-Wno-sign-compare")
    .flag("-Wno-unused-local-typedefs")
    .flag("-Winit-self")
    .flag("-rdynamic")
    .flag("-DHAVE_POSIX_MEMALIGN")
    .flag("-O3")
    .compile("librsnowboywrapper.a");


//  println!("cargo:rustc-link-lib=bz2");
//  let bindings = bindgen::Builder::default()
//    .header("rsnowboywrapper/rsnowboy.h")
//    .enable_cxx_namespaces()
//    .layout_tests(false)
//    .opaque_type("std::.*")
//    .generate()
//    .expect("Unable to generate bindings");
//
//  bindings
//    .write_to_file(PathBuf::from("src/rawrsnoboy.rs").as_path())
//    .expect("Couldn't write bindings!");
}
