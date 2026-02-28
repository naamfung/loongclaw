# 以下命令会自动生成.h头文件，故此无须手动编写.h头文件

# 检测操作系统
case "$(uname)" in
    Linux)
        echo "Building for Linux (musl)"
        CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build \
            -buildmode=c-archive \
            -ldflags '-extldflags "-static"' \
            -o libservico.a
        ;;
    *)
        echo "Building for native platform"
        go build -buildmode=c-archive -o libservico.a main.go
        ;;
esac
