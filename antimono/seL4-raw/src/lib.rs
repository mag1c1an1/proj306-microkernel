#![no_std]
#![allow(non_camel_case_types)]

//! use c style to fork sel4

struct ApicBaseMsr(u32);

impl ApicBaseMsr {
    fn base_addr(&self) -> u32 {
        todo!()
    }
    fn enabled(&self) -> u32 {
        todo!()
    }
    fn x2apic(&self) -> u32 {
        todo!()
    }
}

struct ApicIcr1(u32);
struct ApicIcr2(u32);
struct ApicLvt(u32);

struct EndPoint([u64; 2]);

struct MdbNode([u64; 2]);
struct Notification([u64; 4]);
