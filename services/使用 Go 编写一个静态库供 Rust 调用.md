要使用 Go 编写一个静态库供 Rust 调用，你需要通过 C 语言作为桥梁（FFI），因为 Go 可以编译为 C 兼容的静态库，而 Rust 可以链接 C 库。以下是逐步指导：

---

## 1. 编写可导出的 Go 代码

首先创建一个 Go 源文件，例如 `mylib.go`。必须使用 `import "C"` 并利用 `//export` 注释导出你想要 Rust 调用的函数。注意导出的函数必须满足 C 调用约定，只能使用与 C 兼容的类型（如整数、浮点数、指针等）。

```go
package main

import "C"
import "fmt"

// 导出一个简单的加法函数
//export add
func add(a, b C.int) C.int {
    return a + b
}

// 导出一个打印字符串的函数，演示如何处理 Go 字符串
//export greet
func greet(name *C.char) {
    // 将 C 字符串转换为 Go 字符串
    goName := C.GoString(name)
    fmt.Printf("Hello, %s!\n", goName)
}

// 注意：必须包含 main 函数，即使为空
func main() {}
```

- `import "C"` 是必须的，它激活了 cgo 功能。
- `//export` 注释告诉 cgo 将该函数导出到生成的 C 头文件中。
- 函数参数和返回值必须是 C 兼容类型。
- 即使库不包含可执行入口，也必须保留 `main` 函数（可以空）。

---

## 2. 编译为静态库

使用 Go 工具链生成静态库（`.a` 文件）和对应的 C 头文件（`.h`）。在终端中执行：

```bash
go build -buildmode=c-archive -o libmylib.a mylib.go
```

执行成功后，你会得到两个文件：
- `libmylib.a`：静态库文件
- `libmylib.h`：C 头文件，包含了导出函数的声明

你可以检查头文件内容，它会包含类似：

```c
#ifdef __cplusplus
extern "C" {
#endif

extern int add(int a, int b);
extern void greet(char* name);

#ifdef __cplusplus
}
#endif
```

---

## 3. 准备 Rust 项目

创建一个新的 Rust 项目（如果还没有）：

```bash
cargo new rust_caller --bin
cd rust_caller
```

将刚才生成的 `libmylib.a` 和 `libmylib.h` 复制到 Rust 项目中的某个位置，比如项目根目录下的 `lib/` 文件夹。

```
rust_caller/
├── Cargo.toml
├── src/
│   └── main.rs
└── lib/
    ├── libmylib.a
    └── libmylib.h
```

---

## 4. 配置 Rust 构建脚本 (`build.rs`)

为了让 Rust 正确链接 Go 静态库，你需要编写一个构建脚本，告诉编译器在哪里找到库文件以及链接哪些库。

在项目根目录创建 `build.rs`：

```rust
fn main() {
    // 告诉 Rust 在编译时重新运行此脚本（如果 lib/ 目录变化）
    println!("cargo:rerun-if-changed=lib");

    // 指定库搜索路径（相对路径，从项目的根目录出发）
    println!("cargo:rustc-link-search=native=lib");

    // 指定要链接的静态库（不包含前面的 lib 和后面的 .a）
    println!("cargo:rustc-link-lib=static=mylib");

    // 如果你的 Go 代码使用了 pthread、dl 等，可能需要添加以下链接
    // println!("cargo:rustc-link-lib=dylib=pthread");
    // println!("cargo:rustc-link-lib=dylib=dl");
}
```

> **注意**：Go 运行时可能依赖 `pthread` 和 `dl`，如果链接时出现未定义符号错误，可以尝试添加上述注释掉的两行。另外，如果你在 macOS 上使用，可能还需要链接 `System` 框架或其他库，具体取决于你的 Go 版本和平台。

---

## 5. 在 Rust 中声明外部函数

你可以手动声明外部函数（基于头文件），或者使用 `bindgen` 自动生成。为了简单，这里手动声明。

编辑 `src/main.rs`：

```rust
// 声明从静态库导入的函数
extern "C" {
    fn add(a: i32, b: i32) -> i32;
    fn greet(name: *const std::os::raw::c_char);
}

fn main() {
    // 调用 add
    let result = unsafe { add(5, 3) };
    println!("5 + 3 = {}", result);

    // 调用 greet，需要将 Rust 字符串转换为 C 字符串
    let name = std::ffi::CString::new("Rust").expect("CString::new failed");
    unsafe {
        greet(name.as_ptr());
    }
}
```

- 由于调用外部函数是不安全的，必须放在 `unsafe` 块中。
- `add` 返回 `i32`（对应 C 的 `int`），与 Go 的 `C.int` 一致。
- `greet` 接受一个 `*const c_char`，我们需要用 `CString` 将 Rust 字符串转换为 C 兼容的字符串。

---

## 6. 编译和运行

现在可以编译并运行 Rust 程序了：

```bash
cargo run
```

你应该看到输出：

```
5 + 3 = 8
Hello, Rust!
```

---

## 重要注意事项

### 1. Go 运行时初始化
静态库 `libmylib.a` 包含了 Go 运行时的初始化代码。当 Rust 程序启动时，Go 运行时会被自动初始化（通过链接器引入的构造函数）。大多数情况下无需额外处理。但如果你遇到崩溃或未初始化错误，可以尝试在 Rust 的 `main` 之前调用 Go 的初始化函数（通常名为 `_cgo_init` 或其他），但通常链接器会处理。

### 2. 内存管理
- **从 Go 分配的内存**：由 Go 的垃圾回收器管理。当你从 Rust 调用 Go 函数返回一个指针时，该指针指向的内存由 Go 管理，Rust 不应释放它。
- **从 Rust 传递内存给 Go**：如果 Go 函数需要长期持有 Rust 分配的指针（例如在回调中），你需要非常小心，因为 Rust 可能在没有通知 Go 的情况下释放内存。最佳实践是在 FFI 边界上复制数据，而不是共享指针。

### 3. 线程安全
Go 运行时可能会创建新的操作系统线程（例如当使用 goroutine 时）。Rust 代码在多线程环境下调用 Go 函数通常是安全的，但 Go 本身并不是完全线程安全的（例如，在多个线程同时调用 Go 函数时，Go 运行时会处理）。通常可以正常使用。

### 4. 跨平台编译
如果你需要在不同平台上编译，需要为每个目标平台单独编译 Go 静态库。例如，为 Linux 编译 `linux/amd64` 版本，为 macOS 编译 `darwin/amd64` 或 `darwin/arm64` 版本。可以使用 Go 的 `GOOS` 和 `GOARCH` 环境变量：

```bash
GOOS=linux GOARCH=amd64 go build -buildmode=c-archive -o libmylib.a mylib.go
```

然后将对应平台的库放在 Rust 项目相应位置，并在 `build.rs` 中动态选择路径（可以使用 `std::env::var("TARGET")` 判断目标）。

### 5. 调试符号
如果遇到链接错误，可以使用 `cargo build -vv` 查看详细的链接命令，检查是否找到了库。

---

## 完整示例项目结构

```
go-rust-ffi/
├── go/
│   ├── mylib.go
│   └── build.sh (编译脚本)
└── rust_caller/
    ├── Cargo.toml
    ├── build.rs
    ├── lib/
    │   ├── libmylib.a
    │   └── libmylib.h
    └── src/
        └── main.rs
```

编译 Go 库后，运行 Rust 程序即可。

---

通过以上步骤，你应该能够成功创建一个 Go 静态库并在 Rust 中使用它。如果在实践中遇到问题，可以检查是否遗漏了必要的链接库，或者查看 Go 与 Rust 的 FFI 边界数据类型是否匹配。