// Generate bindings: bindgen --ctypes-prefix raw --no-doc-comments --raw-line "#![allow(non_upper_case_globals)]" --raw-line "#![allow(non_camel_case_types)]" --raw-line "#![allow(non_snake_case)]" --raw-line "#![allow(dead_code)]"  --raw-line "use std::os::raw;" --rustfmt-bindings -o genned_bindings.rs nvml.h
// Generate lib file (from VS dev console): `dumpbin /EXPORTS nvml.dll > nvml.exports`, paste function names into nvml.def with `EXPORTS` at top, `lib /def:nvml.def /out:nvml.lib /machine:X64`

#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rustc-link-lib=nvml");
    println!("cargo:rustc-link-search={}", env!("CARGO_MANIFEST_DIR"));
}

#[cfg(target_os = "linux")]
fn main() {
    let paths = std::fs::read_dir("/usr/lib").unwrap();

    for path in paths {
        let entry = path.unwrap().path();
        if entry.is_dir() {
            let entry_string = entry.to_string_lossy();
            if entry_string.contains("nvidia") && !entry_string.ends_with("prime") {
                println!("cargo:rustc-link-search=native={}", entry_string);
                break;
            }
        }
    }

    println!("cargo:rustc-link-lib=nvidia-ml");
}

#[cfg(target_os = "macos")]
fn main() {
    compile_error!("NVML is not supported on macOS and therefore this crate cannot compile.");
}
