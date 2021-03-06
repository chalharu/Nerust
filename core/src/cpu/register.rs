// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub(crate) struct Register {
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,

    c: bool, // 0x01
    z: bool, // 0x02
    i: bool, // 0x04
    d: bool, // 0x08
    b: bool, // 0x10
    r: bool, // 0x20
    v: bool, // 0x40
    n: bool, // 0x80
}

bitflags::bitflags! {
    pub struct RegisterP: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT = 0b0000_0100;
        const DECIMAL = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const RESERVED = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

impl Register {
    pub(crate) fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,

            c: false, // 0x01
            z: false, // 0x02
            i: false, // 0x04
            d: false, // 0x08
            b: false, // 0x10
            r: true,  // 0x20
            v: false, // 0x40
            n: false, // 0x80
        }
    }

    pub(crate) fn get_pc(&self) -> u16 {
        self.pc
    }
    pub(crate) fn get_sp(&self) -> u8 {
        self.sp
    }
    pub(crate) fn get_a(&self) -> u8 {
        self.a
    }
    pub(crate) fn get_x(&self) -> u8 {
        self.x
    }
    pub(crate) fn get_y(&self) -> u8 {
        self.y
    }
    pub(crate) fn get_c(&self) -> bool {
        self.c
    }
    pub(crate) fn get_z(&self) -> bool {
        self.z
    }
    pub(crate) fn get_i(&self) -> bool {
        self.i
    }
    pub(crate) fn get_v(&self) -> bool {
        self.v
    }
    // pub fn get_b(&self) -> bool {
    //     self.b
    // }
    pub(crate) fn get_n(&self) -> bool {
        self.n
    }
    pub(crate) fn get_p(&self) -> u8 {
        ((if self.c {
            RegisterP::CARRY
        } else {
            RegisterP::empty()
        }) | (if self.z {
            RegisterP::ZERO
        } else {
            RegisterP::empty()
        }) | (if self.i {
            RegisterP::INTERRUPT
        } else {
            RegisterP::empty()
        }) | (if self.d {
            RegisterP::DECIMAL
        } else {
            RegisterP::empty()
        }) | (if self.b {
            RegisterP::BREAK
        } else {
            RegisterP::empty()
        }) | (if self.r {
            RegisterP::RESERVED
        } else {
            RegisterP::empty()
        }) | (if self.v {
            RegisterP::OVERFLOW
        } else {
            RegisterP::empty()
        }) | (if self.n {
            RegisterP::NEGATIVE
        } else {
            RegisterP::empty()
        }))
        .bits()
    }

    pub(crate) fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }
    pub(crate) fn set_sp(&mut self, value: u8) {
        self.sp = value;
    }
    pub(crate) fn set_a(&mut self, value: u8) {
        self.a = value;
    }
    pub(crate) fn set_x(&mut self, value: u8) {
        self.x = value;
    }
    pub(crate) fn set_y(&mut self, value: u8) {
        self.y = value;
    }
    pub(crate) fn set_c(&mut self, value: bool) {
        self.c = value;
    }
    pub(crate) fn set_i(&mut self, value: bool) {
        self.i = value;
    }
    pub(crate) fn set_d(&mut self, value: bool) {
        self.d = value;
    }
    // pub fn set_b(&mut self, value: bool) {
    //     self.b = value;
    // }
    pub(crate) fn set_v(&mut self, value: bool) {
        self.v = value;
    }
    pub(crate) fn set_p(&mut self, value: u8) {
        let reg_p = RegisterP::from_bits(value).unwrap();
        self.c = reg_p.contains(RegisterP::CARRY);
        self.z = reg_p.contains(RegisterP::ZERO);
        self.i = reg_p.contains(RegisterP::INTERRUPT);
        self.d = reg_p.contains(RegisterP::DECIMAL);
        self.b = reg_p.contains(RegisterP::BREAK);
        self.r = reg_p.contains(RegisterP::RESERVED);
        self.v = reg_p.contains(RegisterP::OVERFLOW);
        self.n = reg_p.contains(RegisterP::NEGATIVE);
    }

    pub(crate) fn set_z_from_value(&mut self, value: u8) {
        self.z = value == 0;
    }

    pub(crate) fn set_n_from_value(&mut self, value: u8) {
        self.n = value & 0x80 != 0;
    }

    pub(crate) fn set_nz_from_value(&mut self, value: u8) {
        self.set_z_from_value(value);
        self.set_n_from_value(value);
    }
}
