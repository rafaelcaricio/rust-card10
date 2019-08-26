use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out = env::var_os("OUT_DIR")
        .map(|path| PathBuf::from(path))
        .ok_or("OUT_DIR definition missing")?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor/l0dable.ld");
    println!("cargo:rerun-if-changed=vendor/client.c");
    println!("cargo:rerun-if-changed=vendor/wrapper.h");
    println!("cargo:rustc-link-search={}", out.display());

    // Linker script
    fs::copy("vendor/crt.s", out.join("crt.s"))?;
    fs::copy("vendor/l0dable.ld", out.join("l0dable.ld"))?;

    // Link against C code
    cc::Build::new()
        .target("thumbv7em-none-eabi")
        .compiler("arm-none-eabi-gcc")
        .opt_level_str("s")
        .debug(true)
        .define("TARGET", "MAX32665")
        .define("TARGET_UC", "MAX32665")
        .define("TARGET_LC", "max32665")
        .define("TARGET_REV", "0x4131")
        .define("BOARD", "card10")
        .flag("-fPIE")
        .flag("-pie")
        .include("firmware/epicardium")
        .include("firmware/lib/sdk/Libraries/CMSIS/Device/Maxim/MAX32665/Include")
        .include("firmware/lib/sdk/Libraries/CMSIS/Include")
        .include("firmware/lib/sdk/Libraries/MAX32665PeriphDriver/Include")
        .file("firmware/lib/sdk/Libraries/MAX32665PeriphDriver/Source/sema.c")
        .file("firmware/lib/sdk/Libraries/MAX32665PeriphDriver/Source/mxc_assert.c")
        .file("firmware/l0dables/lib/hardware.c")
        .file("firmware/epicardium/api/caller.c")
        .file("firmware/lib/gfx/Fonts/font12.c")
        .file("firmware/lib/gfx/Fonts/font16.c")
        .file("firmware/lib/gfx/Fonts/font20.c")
        .file("firmware/lib/gfx/Fonts/font24.c")
        .file("firmware/lib/gfx/Fonts/font8.c")
        .file("vendor/client.c")
        .compile("card10");

    // Generate bindings to C code
    let bindings = bindgen::Builder::default()
        .use_core()
        .clang_arg("-Isrc")
        .clang_arg("-Ifirmware/epicardium")
        .clang_arg("-Ifirmware/lib/sdk/Libraries/CMSIS/Device/Maxim/MAX32665/Include")
        .clang_arg("-Ifirmware/lib/sdk/Libraries/CMSIS/Include")
        .clang_arg("-Ifirmware/lib/sdk/Libraries/MAX32665PeriphDriver/Include")
        .header("vendor/wrapper.h")
        .ctypes_prefix("ctypes")
        .generate()
        .map_err(|_| "Couldn't generate bindings")?;

    bindings.write_to_file(out.join("bindings.rs"))?;

    Ok(())
}
