fn main() {
    // Rebuild when embedded frontend assets change.
    println!("cargo:rerun-if-changed=web/dist");
    // Keep builtin skills embedding in sync as well.
    println!("cargo:rerun-if-changed=skills/built-in");

    // Link against the Go static library
    println!("cargo:rustc-link-search=native=services");
    println!("cargo:rustc-link-lib=static=servico");

    // On non-Windows platforms (Linux, etc.), we may need to link pthread and dl statically
    // because the Go static library might depend on them.
    #[cfg(not(target_os = "windows"))]
    {
        // Link pthread and dl statically. These are provided by musl-dev on Alpine.
        // If your Go library was compiled with CGO_ENABLED=0, these may not be needed,
        // but it's safe to include them as they will be ignored if not required.
        println!("cargo:rustc-link-lib=static=pthread");
        println!("cargo:rustc-link-lib=static=dl");

        // Note: For musl targets, Rust already links musl libc statically, so no need for -lc.
        // If you still encounter undefined references to other C symbols (e.g., malloc, free),
        // it indicates that your Go library was not compiled with CGO_ENABLED=0.
        // In that case, you should recompile the Go library with CGO_ENABLED=0.
    }

    // Rebuild when the static library changes
    println!("cargo:rerun-if-changed=services/libservico.a");
    println!("cargo:rerun-if-changed=services/libservico.h");
}