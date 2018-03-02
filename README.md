# Bitmap4Rust

![Status](https://img.shields.io/badge/status-active-brightgreen.svg?style=flat)
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)]()
[![License](https://img.shields.io/badge/license-mit-brightgreen.svg?style=flat)](https://github.com/aditisrinivas97/Bitmap4Rust/blob/master/LICENSE)
[![](http://meritbadge.herokuapp.com/bitmap4rust)](https://crates.io/crates/bitmap4rust)

A Rust library for creating and manipulating Bitmaps.

## API


The bitmap is maintained in a byte vector of appropriate size. Hence, it's type should be `Vec<u8>`.

___

    pub fn bitmap_create(bitmap_size: &mut u64) -> Vec<u8>

Used to create the bitmap. Since the bitmap is maintained as a byte vector, it must be of type `Vec<u8>`.


---

    pub fn clear_bit(bitmap: &mut Vec<u8>, bitno: u64) -> i32

Used to clear the bit at `bitno`, i.e, set it to 0. If `bitno` is larger than the bitmap's size, -1 is returned.

---

    pub fn set_bit(bitmap: &mut Vec<u8>, bitno: u64) -> i32

Used to set the bit at `bitno`, i.e, set it to 1. If `bitno` is larger than the bitmap's size, -1 is returned.

---

    pub fn get_first_set_bit(bitmap: &mut Vec<u8>) -> i64

Used to get the first bit of `bitmap` that is set to 1. Returns -1 if none are set.

---

    pub fn get_first_unset_bit(bitmap: &mut Vec<u8>) -> i64 

Used to get the first bit of `bitmap` that is set to 0. Returns -1 if all bits are set.

---

    pub fn check_bit(bitmap: &mut Vec<u8>, bitno: u64) -> i32

Used to check the value of the bit corresponding to `bitno`.

---

## Usage

* Add the dependency `bitmap4rust` in `Cargo.toml`

```rust

[dependencies]
bitmap4rust = "1.0.0"

```

* Include the crate `bitmap4rust`

```rust

extern crate bitmap4rust;

```