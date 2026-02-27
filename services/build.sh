# 以下命令会自动生成.h头文件，故此无须手动编写.h头文件
go build -buildmode=c-archive -o libservico.a main.go
