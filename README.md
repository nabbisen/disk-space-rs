# disk-space-rs

[![crates.io](https://img.shields.io/crates/v/disk-space?label=rust)](https://crates.io/crates/disk-space)
[![License](https://img.shields.io/github/license/nabbisen/disk-space-rs)](https://github.com/nabbisen/disk-space-rs/blob/main/LICENSE)
[![Rust Documentation](https://docs.rs/disk-space/badge.svg?version=latest)](https://docs.rs/disk-space)
[![Dependency Status](https://deps.rs/crate/disk-space/latest/status.svg)](https://deps.rs/crate/disk-space)

Tiny lib for disk space info of os file system (cross-platform supported)

```rust
use std::path::PathBuf;
use disk_space::DiskSpace;

const MIN_DISKSPACE_MB: f64 = 512.0;

let path = PathBuf::from("/");
let disk_space = DiskSpace::new(&path).expect("failed to get file system info");

let disk_space_mb_available = disk_space.as_mb().available;
let disk_space_ok = (MIN_DISKSPACE_MB) < disk_space_mb_available;
```
