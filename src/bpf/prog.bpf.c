// SPDX-License-Identifier: GPL-2
//
// Copyright (C) 2021  William Findlay
//
// Jan. 19, 2021  William Findlay  Created this.

// This must be first
#include <vmlinux.h>

// These must be below vmlinux.h
#include <bpf/bpf_core_read.h> /* for BPF CO-RE helpers */
#include <bpf/bpf_helpers.h> /* most used helpers: SEC, __always_inline, etc */
#include <bpf/bpf_tracing.h> /* for getting kprobe arguments */

/* ========================================================================= *
 * BPF CO-RE Globals                                                         *
 * ========================================================================= */

// TODO: Your global variables here

/* ========================================================================= *
 * BPF Maps                                                                  *
 * ========================================================================= */

// TODO: Your maps here

/* ========================================================================= *
 * Kernel-Dependent Helpers                                                  *
 * ========================================================================= */

// TODO: Your version-dependent helpers here

/* ========================================================================= *
 * Helpers Functions                                                         *
 * ========================================================================= */

// TODO: Your helpers here

/* ========================================================================= *
 * BPF Programs                                                              *
 * ========================================================================= */

// TODO: Your programs here

/* ========================================================================= *
 * License String                                                            *
 * ========================================================================= */

char LICENSE[] SEC("license") = "GPL";
