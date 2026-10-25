#![no_std]

//! Kuberve ABI: общие типы и константы между kernel, libc, userspace.
//!
//! Этот crate НЕ содержит логики, только структуры и константы.
//! Меняется только при изменении бинарного интерфейса syscall.

/// Сообщение IPC. Фиксированный размер, 64 байта.
///
/// Семантика `data`:
/// - `tag=1 (WRITE)`: `data[0] = len`, `data[1..7] = байты` (48 байт payload)
/// - `tag=2 (IRQ)`:   `data[0] = intid`, остальное 0
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Message {
    pub tag: u64,
    pub data: [u64; 7],
}

/// Номера syscall. ARM64 ABI: `x8` = номер, `x0..x5` = аргументы, `x0` = возврат.
pub mod sys {
    pub const YIELD:    u64 = 0;
    pub const WRITE:    u64 = 1;
    pub const EXIT:     u64 = 2;
    pub const SLEEP:    u64 = 3;
    pub const GETPID:   u64 = 4;
    pub const SEND:     u64 = 5;
    pub const RECV:     u64 = 6;
    pub const IRQ_ACK:  u64 = 7;
    pub const TRY_SEND: u64 = 8;
    pub const TRY_RECV: u64 = 9;
    pub const SBRK:     u64 = 10;
    pub const OPEN:  u64 = 11;
    pub const READ:  u64 = 12;
    pub const CLOSE: u64 = 13;
    pub const WRITE_FILE: u64 = 14;
    pub const SOCKET:   u64 = 15;
    pub const BIND:     u64 = 16;
    pub const SENDTO:   u64 = 17;
    pub const RECVFROM: u64 = 18;
    pub const CONNECT:  u64 = 19;
    pub const TCP_SEND: u64 = 20;
    pub const TCP_RECV: u64 = 21;
    pub const SPAWN:    u64 = 22;
}

/// Теги IPC-сообщений (для `Message::tag`).
pub mod tags {
    /// WRITE: data[0] = ptr, data[1] = len
    pub const WRITE: u64 = 1;
    /// IRQ: data[0] = intid
    pub const IRQ:   u64 = 2;
}

pub mod flags {
    pub const O_RDONLY: u64 = 0;
    pub const O_CREAT:  u64 = 1;
    pub const O_TRUNC:  u64 = 2;
    pub const O_APPEND: u64 = 4;
}

pub mod socket_const {
    pub const AF_INET: u64 = 2;
    pub const SOCK_DGRAM: u64 = 2;
}