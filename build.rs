//extern crate cc;
//extern crate bindgen;

//use std::path::PathBuf;
use std::env;
//use std::process::Command;

fn get_lib_folder(arch_sub: &str, os: &str, abi: &str) -> Result<&'static str, String> {
  match os {
    "ios" => {Ok("ios")}
    "darwin" => {Ok("osx")}
    "linux" => {
      match abi {
        "android" | "androideabi" => { // Android
          match arch_sub {
            "aarch64" => {Ok("android/armv8-aarch64")},
            "armv7" => {Ok("android/armv7a")},
            other_arch => {
              Err(format!("architecture '{}' not supported for Android", other_arch))
            }
          }
        },
        _ => { // Generic Linux
          match arch_sub {
            "x86_64" => {Ok("ubuntu64")},
            "aarch64" => {Ok("aarch64-ubuntu1604")},
            "armv6" | "armv7" => {Ok("rpi")},
            other_arch => {
              Err(format!("architecture '{}' not supported for Linux", other_arch))
            }
          }
        }
      }
    }
    other_os => {
      Err(format!("target OS ({}), not supported", other_os))
    }
  }
}

fn main() {
  //  println!("cargo:rustc-link-lib=static=stdc++");
  println!("cargo:rustc-link-lib=gslcblas");
  //  println!("cargo:rustc-link-lib=snowboy-detect");

  // Link to the corresponding snowboy library for the current platform
  match env::var("TARGET") {
    Ok(target_triplet) => {
      let target_vec: Vec<&str> = target_triplet.split('-').collect();

      let (arch_sub, os, abi) = match target_vec.as_slice() {
        [arch_sub, _vendor, os, abi] => {(arch_sub, os, abi)},
        [arch_sub, os, abi] => {(arch_sub, os, abi)},
        _ => { eprintln!("Couldn't match target triplet, compilation might fail");
          (&"unknown", &"unknown", &"unknown")
        }
      };

      match get_lib_folder(arch_sub, os, abi) {
        Ok(folder) => {println!("cargo:rustc-link-search=native=lib/{}", folder);},
        Err(reason) => {eprintln!("Couldn't determine which library to use: {}, compilation might fail", reason);}
      }
    }
    Err(e) => {eprintln!("Couldn't read  'TARGET' environment variable ({}), can't choose native lib, compilation might fail", e);}
  }

  println!("cargo:rustc-link-lib=static=snowboy-detect");

  // build dependency for snowboy-detect
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


  //// if you want generate rust code everytime, release this comment.
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
