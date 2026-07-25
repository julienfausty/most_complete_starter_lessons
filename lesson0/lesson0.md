# Lesson 0 - Set up environment

The zero lesson is supposed to be about setting up the environment. What follows goes through the steps to set up my rust environment on an Archlinux OS.

## Step 1 - Install rust toolchain

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Step 2 - Install avr-gcc toolchain for AVR microcontrollers

```shell
pacman -S avr-gcc avr-libc avrdude
```

## Step 3 - Setup rust toolchain

```shell
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo install cargo-binutils
rustup component add llvm-tools-preview
```
