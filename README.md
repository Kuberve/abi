[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Status: pre-alpha](https://img.shields.io/badge/status-pre--alpha-orange.svg)]()
[![Platform: aarch64](https://img.shields.io/badge/platform-aarch64-lightgrey.svg)]()

# Kuberve ABI

Shared types and constants for the **Kuberve** binary interface: syscall
numbers, the `Message` struct, tags, flags, and `repr(C)` types used across
the kernel / libc / userspace boundary.

> **Pre-alpha.** This is the contract between EL1 and EL0. Any change here
> is a breaking ABI change. Treat it like a public API from day one.

## Why MIT (not AGPL)

This crate contains only constants and struct definitions — no logic. It is
meant to be freely embedded in any program (proprietary, MIT, GPL, whatever)
that talks to the Kuberve kernel. MIT keeps that frictionless. The actual
implementation lives in the kernel (AGPLv3) and `libc` (LGPLv3); programs
that use those still must respect those licenses, but the header is free.

## What's inside

### `Message` (64 bytes, `repr(C)`)

```rust
pub struct Message {
    pub tag: u64,        // tag determines how to read `data`
    pub data: [u64; 7],  // 56 bytes payload
}
```

- Half a cache line, fits in `stp/ldp` register pair.
- Large payloads go through shared memory (`v0.7+`), not `Message`.

### Tags

| Tag | Name | `data[0]` | `data[1]` | `data[2..7]` |
|---|---|---|---|---|
| 1 | WRITE | `ptr` | `len` | — |
| 2 | IRQ   | `intid` | — | — |

### Syscall numbers

ABI: `x8` = number, `x0..x5` = args, `x0` = return. `svc #0` to enter.
`u64::MAX` return means error.

| # | Name | Args | Returns |
|---|---|---|---|
| 0 | YIELD | — | — |
| 1 | WRITE | `fd`, `ptr`, `len` | `n` or -1 |
| 2 | EXIT | `code` | never |
| 3 | SLEEP | `ticks` | — |
| 4 | GETPID | — | `pid` |
| 5 | SEND | `target_pid`, `*Message` | 0 or -1 |
| 6 | RECV | `*Message` | 0 |
| 7 | IRQ_ACK | `intid` | — |
| 8 | TRY_SEND | `target_pid`, `*Message` | 0 or -1 |
| 9 | TRY_RECV | `*Message` | 0 or -1 |
| 10 | SBRK | `delta` | `old_brk` |
| 11 | OPEN | `path_ptr`, `path_len`, `flags` | fd or -1 |
| 12 | READ | `fd`, `buf`, `len` | `n` or -1 |
| 13 | CLOSE | `fd` | 0 or -1 |
| 14 | WRITE_FILE | `fd`, `buf`, `len` | `n` or -1 |
| 15 | SOCKET | `domain`, `kind` | fd or -1 |
| 16 | BIND | `fd`, `port` | 0 or -1 |
| 17 | SENDTO | `fd`, `buf`, `len`, `ip`, `port` | `n` or -1 |
| 18 | RECVFROM | `fd`, `buf`, `len`, `ip_out`, `port_out` | `n` or -1 |
| 19 | CONNECT | `fd`, `ip`, `port` | 0 or -1 |
| 20 | TCP_SEND | `fd`, `buf`, `len` | `n` or -1 |
| 21 | TCP_RECV | `fd`, `buf`, `len` | `n` or -1 |
| 22 | SPAWN | `path_ptr`, `path_len`, `priority` | pid or -1 |
| 23 | KILL | `pid` | 0 or -1 |
| 24 | WAIT | `pid` | exit code |
| 25 | STAT | `path_ptr`, `path_len`, `*Stat` | 0 or -1 |
| 26 | MKDIR | `path_ptr`, `path_len` | 0 or -1 |
| 27 | UNLINK | `path_ptr`, `path_len` | 0 or -1 |
| 28 | RMDIR | `path_ptr`, `path_len` | 0 or -1 |
| 29 | READDIR | `path_ptr`, `path_len`, `buf`, `max` | `n` or -1 |
| 30 | GUI_CREATE_WINDOW | `w`, `h`, `title_ptr`, `title_len` | `id` or -1 |

Full spec: [Kuberve/spec → syscalls](https://github.com/Kuberve/spec/blob/main/syscalls/README.md).

## Usage

```rust
use kuberve_abi::{Message, sys, tags};
```

This crate contains **no unsafe code and no logic**. If you find yourself
wanting to add helper functions here, they belong in `kuberve-libc` instead.

## Prerequisites

None. This crate has no dependencies.

## Build

```bash
git clone https://github.com/Kuberve/abi.git
cd abi
cargo build
```

## Related repos (Kuberve Org)

- [kuberve-libc](https://github.com/Kuberve/libc) — syscall wrappers, heap, UART (LGPLv3)
- [kuberve-user](https://github.com/Kuberve/user) — userspace programs (AGPLv3)
- [kuberve-kernel](https://github.com/Kuberve/kernel) — the kernel (AGPLv3)
- [kuberve-spec](https://github.com/Kuberve/spec) — formal ABI specs

## Contributing

Pull requests are accepted, but may take time to review. Please read
[CONTRIBUTING.md](CONTRIBUTING.md) and sign the [CLA](CLA.md) before opening
a PR.

## License

- **Header / constants:** MIT. See [LICENSE](LICENSE).
- **Trademark:** "Kuberve" is a trademark. See [TRADEMARK.md](TRADEMARK.md).

---

**Kuberve** is a trademark of the project author.