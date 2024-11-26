# windows-kernel-driver-rust-hello-world

## Intro
This project is based on the presentation by Matthias Heiden
https://www.youtube.com/watch?v=NfBXDEgm6VY


### Pre-requisites

1. Windows (or VM)
2. [WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)  (Windows Driver Kit, essential tools and headers)



3. [LLVM](https://github.com/llvm/llvm-project/releases/tag/llvmorg-19.1.0)  (Rust bindgen from C headers, add to PATH)

4. [OSR Driver Loader](https://www.osronline.com/article.cfm%5earticle=157.htm)

5. [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and add MSVC target for building drivers
```
rustup install nightly
rustup default nightly
rustup target add x86_64-pc-windows-msvc
```
