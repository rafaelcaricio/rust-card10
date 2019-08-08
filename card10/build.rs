use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

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
        .pic(false)
        .include("../c/lib/card10")
        .include("../c/lib/gfx")
        .include("../c/lib/gfx/GUI_DEV")
        .include("../c/lib/gfx/LCD")
        .include("../c/lib/gfx/Fonts")
        .include("../c/lib/vendor/Bosch/BHy1/driver/inc")
        .include("../c/lib/vendor/Bosch/BHy1/examples/firmware")
        .include("../c/lib/vendor/Bosch/BME680")
        .include("../c/lib/vendor/Bosch/BMA400")
        .include("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Include")
        .include("../c/lib/sdk/Libraries/CMSIS/Device/Maxim/MAX32665/Include")
        .include("../c/lib/sdk/Libraries/CMSIS/Include")
        .include("../c/lib/sdk/Libraries/Boards/card10/Include")
        .include("../c/lib/vendor/Maxim/MAX77650")
        .include("../c/lib/sdk/Libraries/Boards/Include")
        .file("../c/lib/card10/card10.c")
        .file("../c/lib/card10/pb.c")
        .file("../c/lib/card10/pmic.c")
        .file("../c/lib/card10/portexpander.c")
        .file("../c/lib/card10/leds.c")
        .file("../c/lib/card10/bosch.c")
        .file("../c/lib/card10/display.c")
        .file("../c/lib/gfx/LCD/LCD_Driver.c")
        .file("../c/lib/gfx/GUI_DEV/DEV_Config.c")
        .file("../c/lib/gfx/GUI_DEV/GUI_Paint.c")
        .file("../c/lib/sdk/Libraries/CMSIS/Device/Maxim/MAX32665/Source/system_max32665.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/i2c.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/gpio.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/tmr.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/tmr_utils.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/rtc.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/spi.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/spi17y.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/mxc_delay.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/mxc_lock.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/mxc_sys.c")
        .file("../c/lib/sdk/Libraries/MAX32665PeriphDriver/Source/mxc_pins.c")
        .file("../c/lib/sdk/Libraries/Boards/card10/Source/board.c")
        // .file("../c/lib/sdk/Libraries/Boards/Source/stdio.c")
        .file("../c/lib/vendor/Bosch/BMA400/bma400.c")
        .file("../c/lib/vendor/Bosch/BME680/bme680.c")
        .file("../c/lib/vendor/Bosch/BHy1/driver/src/bhy.c")
        .file("../c/lib/vendor/Bosch/BHy1/driver/src/bhy_uc_driver.c")
        .file("../c/lib/vendor/Bosch/BHy1/driver/src/bhy_support.c")
        .file("../c/lib/vendor/Maxim/MAX77650/MAX77650-Arduino-Library.c")
        .compile("card10");
}
