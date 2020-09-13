# `rust_for_cpp`

Codes for learning to build FFI APIs in Rust for C++.
There are some memory leaks in the library.
I do not know if I did something wrong, probably I did, but there are also some leaks are not my fault: https://github.com/rust-lang/rust/issues/19776.

## How to compile

```bash
cargo build
```


## Generating `.h` file

```bash
cbindgen --config cbindgen.toml --crate ffiapi --output examples/ffiapi.h
```


## Using the library in C++ code

```bash
g++ --std=c++11 -o target/test examples/test.cpp -Ltarget/debug/ -lffiapi
```

or

```bash
g++ --std=c++11 -o target/test examples/test.cpp target/debug/libffiapi.so
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
