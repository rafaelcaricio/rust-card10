use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Linker script
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("l0dable.ld"))
        .unwrap()
        .write_all(include_bytes!("l0dable.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=l0dable.ld");

    // Link against C code
    cc::Build::new()
        .target("thumbv7em-none-eabi")
        .compiler("arm-none-eabi-gcc")
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
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/sema.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/mxc_assert.c")
        .file("../c/l0dables/lib/hardware.c")
        .file("../c/epicardium/api/caller.c")
        .file("src/client.c")
        .file("../c/lib/gfx/Fonts/font12.c")
        .file("../c/lib/gfx/Fonts/font16.c")
        .file("../c/lib/gfx/Fonts/font20.c")
        .file("../c/lib/gfx/Fonts/font24.c")
        .file("../c/lib/gfx/Fonts/font8.c")
        .compile("card10");
    println!("cargo:rerun-if-changed=src/client.c");

    // Generate bindings to C code
    let bindings = bindgen::Builder::default()
        .clang_args(&[
            "-Isrc",
            "-I../c/epicardium",
            "-I../c/lib/sdk/Libraries/CMSIS/Device/Maxim/MAX32665/Include",
            "-I../c/lib/sdk/Libraries/CMSIS/Include",
            "-I../c/lib/sdk/Libraries/MAX32665PeriphDriver/Include",
        ])
        .header("src/bindings.h")
        .use_core()
        .ctypes_prefix("super::ctypes")
        .generate()
        .expect("Unable to generate bindings");
    println!("cargo:rerun-if-changed=src/bindings.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
