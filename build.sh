cd services
go mod tidy
chmod +x build.sh
./build.sh
cd ..
cargo build --release
#cargo build --release --features sqlite-vec

cp target/release/loongclaw /usr/local/bin/