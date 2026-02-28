cd services
go mod tidy
./build.sh
cd ..
cargo build --release
#cargo build --release --features sqlite-vec