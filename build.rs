use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put linker scripts in the build directory
    fs::copy("ld/tc37xA_memory.ld", out_dir.join("tc37xA_memory.ld"))?;
    fs::copy(
        "ld/tc37x_bsp_example_llvm.ld",
        out_dir.join("tc37x_bsp_example_llvm.ld"),
    )?;

    Ok(())
}
