cd services
go mod tidy
chmod +x build.sh
./build.sh
cd ..

# 检测操作系统
case "$(uname)" in
    Linux)
        echo "Building for Linux (musl)"
        cargo build --release --target x86_64-unknown-linux-musl
        cp target/x86_64-unknown-linux-musl/release/loongclaw /usr/local/bin/
        cp target/x86_64-unknown-linux-musl/release/loongclaw ./
        ;;
    *)
        echo "Building for native platform"
        cargo build --release
        cp target/release/loongclaw ./
        ;;
esac
