// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub(crate) trait Read: CpuStep {
    fn reader(register: &mut Register, value: u8);

    fn entry_opcode(
        &mut self,
        _core: &mut Core,
        _ppu: &mut Ppu,
        _cartridge: &mut Cartridge,
        _controller: &mut Controller,
        _apu: &mut Apu,
    ) {
        self.set_step(0);
    }

    fn exec_opcode(
        &mut self,
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut Cartridge,
        controller: &mut Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        let step = self.get_step() + 1;
        self.set_step(step);
        match step {
            1 => {
                let data = core.memory.read(
                    core.register.get_opaddr(),
                    ppu,
                    cartridge,
                    controller,
                    apu,
                    &mut core.interrupt,
                );
                Self::reader(&mut core.register, data);
            }
            _ => {
                return CpuStepStateEnum::Exit;
            }
        }
        CpuStepStateEnum::Continue
    }
}

macro_rules! read {
    ($name:ident, $reader:expr) => {
        pub(crate) struct $name {
            step: usize,
        }

        impl $name {
            pub fn new() -> Self {
                Self { step: 0 }
            }
        }

        impl CpuStep for $name {
            fn get_step(&self) -> usize {
                self.step
            }

            fn set_step(&mut self, value: usize) {
                self.step = value;
            }
        }

        impl Read for $name {
            fn reader(register: &mut Register, value: u8) {
                $reader(register, value);
            }
        }

        cpu_step_state_impl!($name);
    };
}

pub(crate) trait Write: CpuStep {
    fn writer(register: &mut Register) -> (u8, usize);

    fn entry_opcode(
        &mut self,
        _core: &mut Core,
        _ppu: &mut Ppu,
        _cartridge: &mut Cartridge,
        _controller: &mut Controller,
        _apu: &mut Apu,
    ) {
        self.set_step(0);
    }

    fn exec_opcode(
        &mut self,
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut Cartridge,
        controller: &mut Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        let step = self.get_step() + 1;
        self.set_step(step);
        match step {
            1 => {
                let (data, address) = Self::writer(&mut core.register);
                core.memory.write(
                    address,
                    data,
                    ppu,
                    cartridge,
                    controller,
                    apu,
                    &mut core.interrupt,
                );
            }
            _ => {
                return CpuStepStateEnum::Exit;
            }
        }
        CpuStepStateEnum::Continue
    }
}

macro_rules! write {
    ($name:ident, $writer:expr) => {
        pub(crate) struct $name {
            step: usize,
        }

        impl $name {
            pub fn new() -> Self {
                Self { step: 0 }
            }
        }

        impl CpuStep for $name {
            fn get_step(&self) -> usize {
                self.step
            }

            fn set_step(&mut self, value: usize) {
                self.step = value;
            }
        }

        impl Write for $name {
            fn writer(register: &mut Register) -> (u8, usize) {
                $writer(register)
            }
        }

        cpu_step_state_impl!($name);
    };
}

read!(Lax, |r: &mut Register, v: u8| {
    r.set_a(v);
    r.set_x(v);
    r.set_nz_from_value(v);
});

read!(Anc, |r: &mut Register, v: u8| {
    let result = r.get_a() & v;
    r.set_nz_from_value(result);
    r.set_c(result & 0x80 != 0);
    r.set_a(result);
});

read!(Alr, |r: &mut Register, v: u8| {
    let result = r.get_a() & v;
    r.set_c(result & 0x01 != 0);
    let value = result >> 1;
    r.set_nz_from_value(value);
    r.set_a(value);
});

read!(Arr, |r: &mut Register, v: u8| {
    let result = r.get_a() & v;
    let value = result >> 1 | if r.get_c() { 0x80 } else { 0 };
    r.set_c(result & 0x80 != 0);
    r.set_v((((value >> 6) & 1) != 0) ^ (((value >> 5) & 1) != 0));
    r.set_nz_from_value(value);
    r.set_a(value);
});

read!(Xaa, |r: &mut Register, v: u8| {
    let result = r.get_x() & v;
    r.set_nz_from_value(result);
    r.set_a(result);
});

read!(Las, |r: &mut Register, v: u8| {
    let result = r.get_sp() & v;
    r.set_nz_from_value(result);
    r.set_a(result);
    r.set_x(result);
    r.set_sp(result);
});

read!(Axs, |r: &mut Register, v: u8| {
    let a = u16::from(r.get_a() & r.get_x());
    let b = u16::from(v);
    let d = a.wrapping_sub(b);

    let result = (d & 0xFF) as u8;
    r.set_nz_from_value(result);
    r.set_x(result);
    r.set_c(d <= 0xFF);
});

write!(Sax, |r: &mut Register| (
    r.get_a() & r.get_x(),
    r.get_opaddr()
));

write!(Tas, |r: &mut Register| {
    let sp = r.get_a() & r.get_x();
    r.set_sp(sp);
    (
        sp & ((r.get_pc() >> 8) as u8).wrapping_add(1),
        r.get_opaddr(),
    )
});

write!(Ahx, |r: &mut Register| {
    let address = r.get_opaddr();
    let high = ((address >> 8) as u8).wrapping_add(1);
    (r.get_a() & r.get_x() & high, address)
});

write!(Shx, |r: &mut Register| {
    let address = r.get_opaddr();
    let high = ((address >> 8) as u8).wrapping_add(1);
    let low = address & 0xFF;
    let value = r.get_x() & high;
    let new_addr = (usize::from(value) << 8) | low;
    (value, new_addr)
});

write!(Shy, |r: &mut Register| {
    let address = r.get_opaddr();
    let high = ((address >> 8) as u8).wrapping_add(1);
    let low = address & 0xFF;
    let value = r.get_y() & high;
    let new_addr = (usize::from(value) << 8) | low;
    (value, new_addr)
});
