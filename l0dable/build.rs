use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("l0dable.ld"))
        .unwrap()
        .write_all(include_bytes!("l0dable.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=l0dable.ld");

    cc::Build::new()
        .target("thumbv7em-none-eabihf")
        .compiler("arm-none-eabihf-gcc")
        .define("TARGET", "MAX32665")
        .define("TARGET_UC", "MAX32665")
        .define("TARGET_LC", "max32665")
        .define("TARGET_REV", "0x4131")
        .define("BOARD", "card10")
        .opt_level_str("s")
        .debug(true)
        .flag("-fPIE").flag("-pie")
        .include("../c/epicardium")
        .include("../c/lib/sdk/Libraries/CMSIS/Device/Maxim/MAX32665/Include")
        .include("../c/lib/sdk/Libraries/CMSIS/Include")
        .include("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Include")
        .file("../c/l0dables/lib/hardware.c")
        .compile("card10");
}
