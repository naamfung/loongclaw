fn main() {
    // 编译Go代码为静态库
    let output = std::process::Command::new("go")
        .args(&["build", "-buildmode=c-archive", "-o", "services/libservicor.a", "services/servicor_c.go"])
        .output()
        .expect("Failed to compile Go code");
    
    if !output.status.success() {
        panic!("Failed to compile Go code: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    // 告诉Rust编译器在哪里找到静态库
    println!("cargo:rustc-link-search=native=services");
    println!("cargo:rustc-link-lib=static=servicor");
    println!("cargo:rustc-link-lib=pthread");
    
    // 告诉Cargo当Go代码变化时重新构建
    println!("cargo:rerun-if-changed=services/servicor.go");
    println!("cargo:rerun-if-changed=services/servicor_c.go");
}
