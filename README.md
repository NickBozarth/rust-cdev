# Rust Cdev

Rust Cdev is aimed at being a proof-of-concept for developing character devices in rust for FreeBSD platforms. It is meant to be rough around the edges at the moment and will likely become a standalone library that can assist rust character device development in the future. Currently only 64 bit FreeBSD 15.0-RELEASE for the Raspberry Pi 3 - Model B is tested but more support is to come.

## Usage
- Run make
- Load the resulting .a file with [rust-cdev-init](https://github.com/NickBozarth/rust-cdev-init) project


