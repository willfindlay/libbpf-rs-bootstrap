// SPDX-License-Identifier: GPL-2
//
// Copyright (C) 2021  William Findlay
//
// Jan. 19, 2021  William Findlay  Created this.

use std::process::Command;

fn main() {
    // Re-run build if our header file(s) has changed
    println!("cargo:rerun-if-changed=bindings.h");

    // Generate bindings
    let bindings = bindgen::builder()
        .header("bindings.h")
        .derive_default(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .generate()
        .expect("Failed to generate bindings");

    // Save bindings
    bindings
        .write_to_file("src/bindings/bindings.rs")
        .expect("Failed to save bindings");

    // Make vmlinux if we don't have a good enough version
    // TODO: This command can be allowed to fail if we already have an existing vmlinux.h
    let status = Command::new("make")
        .arg("vmlinux")
        .current_dir("src/bpf")
        .status()
        .expect("Failed to run make");
    assert!(status.success(), "Failed to update vmlinux.h");

    // Run cargo-libbpf-build
    let status = Command::new("cargo")
        .arg("libbpf")
        .arg("build")
        .status()
        .expect("Failed to run cargo libbpf build");
    assert!(status.success(), "Failed to run cargo libbpf build");

    // Run cargo-libbpf-gen
    let status = Command::new("cargo")
        .arg("libbpf")
        .arg("gen")
        .status()
        .expect("Failed to run cargo libbpf gen");
    assert!(status.success(), "Failed to run cargo libbpf gen");
}
