# `ffiapi`

This repository contains codes for learning to build FFI APIs in Rust for C++ and C\#.
There is only one memory leak in this library.
The cause of this leak is the `println` macro, which is called in the function `print_rust`.
More details in the following link: https://github.com/rust-lang/rust/issues/19776.


## How to compile the library

```bash
cargo build
```


## C++

### Generating C++ header

```bash
cbindgen --config cbindgen.toml --crate ffiapi --output include/ffiapi.h
```

### Compiling C++ code

#### Add the library in the `LD_LIBRARY_PATH`

```bash
export LD_LIBRARY_PATH=/home/breno/Documents/Workspace/Projects/rust_for_cpp/target/debug:$LD_LIBRARY_PATH
```

#### Using the library in C++ code

```bash
g++ --std=c++11 -o target/test examples/test.cpp -Ltarget/debug/ -lffiapi
```

or

```bash
g++ --std=c++11 -o target/test examples/test.cpp target/debug/libffiapi.so
```

#### Compile and test

```bash
reset && cargo build && cbindgen --config cbindgen.toml --crate ffiapi --output include/ffiapi.h && g++ --std=c++11 -o target/test examples/test.cpp -Ltarget/debug/ -lffiapi && valgrind ./target/test
```


## C\#

### Compile and test

```bash
reset && cargo build && dotnet run
```


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
