fn main() {
    // Rebuild when embedded frontend assets change.
    println!("cargo:rerun-if-changed=web/dist");
    // Keep builtin skills embedding in sync as well.
    println!("cargo:rerun-if-changed=skills/built-in");
    
    // Link against the Go static library
    println!("cargo:rustc-link-search=native=services");
    println!("cargo:rustc-link-lib=static=servico");
    
    // 在非 Windows 平台上，我们可能需要静态链接 pthread 和 dl，
    // 但更好的方式是启用静态 C 运行时并让链接器自动处理。
    // 这里通过 rustc-link-arg 传递 target-feature，仅对非 Windows 目标生效。
    #[cfg(not(target_os = "windows"))]
    {
        // 启用静态 C 运行时（对 musl 目标有效，也会影响 glibc 目标但可能不必要）
        println!("cargo:rustc-link-arg=-Ctarget-feature=+crt-static");
        
        // 如果 Go 库仍然依赖 pthread 和 dl（即使启用了 +crt-static，有时仍需显式链接），
        // 可以尝试静态链接它们（需要系统中有静态库，Alpine 的 musl-dev 提供）。
        // 但通常 +crt-static 会自动将 libc.a 链接进来，其中已包含 pthread 和 dl 符号。
        // 如果链接阶段出现 undefined reference，再取消下面两行的注释：
        // println!("cargo:rustc-link-lib=static=pthread");
        // println!("cargo:rustc-link-lib=static=dl");
    }
    
    // 注意：Windows 不需要上述设置，且 +crt-static 在 Windows 上含义不同，
    // 因此我们用条件编译隔离，确保 Windows 编译正常。
    
    // Rebuild when the static library changes
    println!("cargo:rerun-if-changed=services/libservico.a");
    println!("cargo:rerun-if-changed=services/libservico.h");
}