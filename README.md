# windows-kernel-driver-rust-hello-world

## Intro
#### This project is based on the presentation by Matthias Heiden
https://www.youtube.com/watch?v=NfBXDEgm6VY

#### Examples on how to write Windows kernel drivers in Rust
https://github.com/StephanvanSchaik/windows-kernel-rs


### [Pre-requisites](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-prerequisites/)

- [Git](https://git-scm.com/downloads)

- Windows (or VM) in Testing Mode (to load self-signed Windows drivers)

- [WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)  Windows Driver Kit (essential tools and headers)

- [LLVM](https://github.com/llvm/llvm-project/releases/tag/llvmorg-19.1.0)  for Rust bindgen from C headers (add to PATH)

- [OSR Driver Loader](https://www.osronline.com/article.cfm%5earticle=157.htm)

- [DebugViewhttps](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview)

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) (nightly) and add MSVC target for building drivers
```
rustup install nightly
rustup default nightly
rustup target add x86_64-pc-windows-msvc
```
- [Cargo-make](https://github.com/sagiegurari/cargo-make) to automatically generate a certificate and sign the driver 
```
cargo install cargo-make
```
