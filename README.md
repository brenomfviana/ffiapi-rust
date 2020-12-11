# `rust_for_cpp`

This repository contains codes for learning to build FFI APIs in Rust for C++.
There is only one memory leak in this library.
The cause of this leak is the `println` macro, which is called in the function `print_rust`.
More details in the following link: https://github.com/rust-lang/rust/issues/19776.


## How to compile

```bash
cargo build
```


## Generating `.h` file

```bash
cbindgen --config cbindgen.toml --crate ffiapi --output include/ffiapi.h
```


## Add the library in the `LD_LIBRARY_PATH`

```bash
export LD_LIBRARY_PATH=/home/breno/Documents/Workspace/Projects/rust_for_cpp/target/debug:$LD_LIBRARY_PATH
```


## Using the library in C++ code

```bash
g++ --std=c++11 -o target/test examples/test.cpp -Ltarget/debug/ -lffiapi
```

or

```bash
g++ --std=c++11 -o target/test examples/test.cpp target/debug/libffiapi.so
```


## Compile and test

```bash
reset && cargo build && cbindgen --config cbindgen.toml --crate ffiapi --output include/ffiapi.h && g++ --std=c++11 -o target/test examples/test.cpp -Ltarget/debug/ -lffiapi && valgrind ./target/test
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
