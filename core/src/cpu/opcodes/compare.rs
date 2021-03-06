// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub(crate) trait Compare {
    fn comparer(register: &Register) -> u8;

    fn exec_opcode(
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut dyn Cartridge,
        controller: &mut dyn Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        match core.internal_stat.get_step() {
            1 => {
                let a = Self::comparer(&core.register);
                let b = core.memory.read(
                    core.internal_stat.get_address(),
                    ppu,
                    cartridge,
                    controller,
                    apu,
                    &mut core.interrupt,
                );

                core.register.set_nz_from_value(a.wrapping_sub(b));
                core.register.set_c(a >= b);
            }
            _ => {
                return exit_opcode(core);
            }
        }
        CpuStepStateEnum::Continue
    }
}

macro_rules! compare {
    ($name:ident, $comparer:expr) => {
        pub(crate) struct $name;

        impl Compare for $name {
            fn comparer(register: &Register) -> u8 {
                $comparer(register)
            }
        }

        cpu_step_state_impl!($name);
    };
}

compare!(Cmp, Register::get_a);
compare!(Cpx, Register::get_x);
compare!(Cpy, Register::get_y);
