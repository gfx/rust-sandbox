# Rust Sandbox

This is just what I leaned on Rust.

## Build

```
cargo run echo -- foo bar
cargo run http2bin
```

## CAVEAT

rust-openssl requires some configuration if you build it on OSX.

https://github.com/sfackler/rust-openssl

Configure homebrew openssl before building dependencies:

```
brew install openssl
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
```

