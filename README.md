# wasm-secp256k1-test

This is test showing that we can compile WASM code to RISC-V directly, and run the resulting binary directly on CKB VM.

# How to use this:

```bash
$ git clone --recursive https://github.com/WebAssembly/wabt
$ cd wabt
$ export WABT_PATH=`pwd`
$ mkdir build
$ cd build
$ cmake ..
$ cmake --build .
$ cd ../..
$ git clone https://github.com/nervosnetwork/wasm-secp256k1-test
$ cd wasm-secp256k1-test
$ cargo build --release --target=wasm32-unknown-unknown
$ $WABT_PATH/bin/wasm2c target/wasm32-unknown-unknown/release/wasm-secp256k1-test.wasm -o secp.c
$ cd ..
$ sudo docker run --rm -it -v `pwd`:/code nervos/ckb-riscv-gnu-toolchain:xenial bash
(docker) $ cd /code/wasm-secp256k1-test
(docker) $ riscv64-unknown-elf-gcc -o secp_riscv64 -O3 -g main.c secp.c /code/wabt/wasm2c/wasm-rt-impl.c -I /code/wabt/wasm2c
(docker) $ exit
```

Now you will have a secp_riscv64 binary to play with in CKB VM. One way to run it, is to take advantage of CKB VM binaries used in CKB VM test suite:

```bash
$ git clone https://github.com/nervosnetwork/ckb-vm-test-suite
$ cd ckb-vm-test-suite
$ git submodule update --init --recursive
$ git clone https://github.com/nervosnetwork/ckb-vm
$ cd binary
$ cargo build --release
$ ./target/release/asm64 ../../wasm-secp256k1-test/secp_riscv64 1
```
