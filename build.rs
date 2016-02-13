extern crate pkg_config;

use std::process::Command;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::File;

// useful links:
// http://alexcrichton.com/pkg-config-rs/pkg_config/struct.Library.html
// https://github.com/carllerche/curl-rust/blob/master/curl-sys/Cargo.toml

fn try_gcc(libname: &str, msg: &str) -> (Vec<String>, Vec<String>, Vec<String>) {

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("main.c");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"
        #include <wand/MagickWand.h>
        int main() {
        }
    ").unwrap();

    match pkg_config::find_library(libname) {
        Err(_) => panic!("{}", msg),
        Ok(lib) => {
            let libpaths = lib.link_paths.iter()
                .map(|x| x.clone().into_os_string().into_string().unwrap())
                .map(|x| "-L".to_string() + &x)
                .collect::<Vec<_>>();

            let libs = lib.libs.iter()
                .map(|x| "-l".to_string() + &x)
                .collect::<Vec<_>>();

            let includepaths = lib.include_paths.iter()
                .map(|x| x.clone().into_os_string().into_string().unwrap())
                .map(|x| "-I".to_string() + &x)
                .collect::<Vec<_>>();

            let s = Command::new("gcc")
                .arg(dest_path.into_os_string().to_str().unwrap())
                .args(&includepaths)
                .args(&libs)
                .args(&libpaths)
                .arg("-o")
                .arg(&format!("{}/main.o", out_dir))
                .status()
                .unwrap();
            assert!(s.success(), "\n\n".to_string() + msg);
            (libpaths, libs, includepaths)
        }
    }
}


fn main() {
    let (lp, libs, ip) = try_gcc("MagickWand",
        "MagickWand not found. On Ubuntu try 'sudo apt-get install libmagickwand-dev' before continuing."
    );

    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["magickwand/image.c", "-c", "-fPIC"])
        .args(&ip)
        .arg("-o")
        .arg(&format!("{}/image.o", out_dir))
        .status()
        .unwrap();

    Command::new("ar")
        .args(&["crs", "libimage.a", "image.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    let _lib = lp.iter().chain(libs.iter()).fold(String::new(), |x, i| x + &i + " ");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=image");

    // TODO: how to do this?
    //println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    //println!("cargo:rustc-link-lib=MagickWand-6.Q16");
    //println!("cargo:rustc-link-lib=MagickCore-6.Q16");

    //println!("cargo:rustc-flags={}", "-l MagickWand-6.Q16 -l MagickCore-6.Q16 -L /usr/lib/x86_64-linux-gnu");
}
