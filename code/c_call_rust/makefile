GCC_BIN ?= $(shell which gcc)
CARGO_BIN ?= $(shell which cargo)

# 动态链接 libcallrust.so
share: clean cargo
	mkdir cbin
	$(GCC_BIN) -o ./cbin/main ./c_src/main.c -I./src -L./target/debug -lcallrust

	# 注意动态链接再运行时也需要再次指定 `.so` 文件所在目录，否则会报错找不到！
	LD_LIBRARY_PATH=./target/debug ./cbin/main

# 静态链接 libcallrust.a
static: clean cargo
	mkdir cbin

	# libcallrust.a 缺少了一些pthread, dl类函数，需要链接进来
	$(GCC_BIN) -o ./cbin/main ./c_src/main.c -I./src ./target/debug/libcallrust.a -lpthread -ldl
	./cbin/main

clean:
	$(CARGO_BIN) clean
	rm -rf ./cbin

cargo:
	$(CARGO_BIN) build
