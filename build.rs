fn main() {
    // Rebuild when embedded frontend assets change.
    println!("cargo:rerun-if-changed=web/dist");
    // Keep builtin skills embedding in sync as well.
    println!("cargo:rerun-if-changed=skills/built-in");
    
    // Link against the Go static library
    println!("cargo:rustc-link-search=native=services");
    println!("cargo:rustc-link-lib=static=servico");
    println!("cargo:rustc-link-lib=dylib=pthread");
    
    // Only link dl on non-Windows platforms
    if cfg!(not(target_os = "windows")) {
        println!("cargo:rustc-link-lib=dylib=dl");
    }
    
    // Rebuild when the static library changes
    println!("cargo:rerun-if-changed=services/libservico.a");
    println!("cargo:rerun-if-changed=services/libservico.h");
}
