// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

macro_rules! condition_jump {
    ($name:ident, $cond:expr) => {
        pub(crate) struct $name;

        impl ConditionJump for $name {
            fn condition(register: &Register) -> bool {
                $cond(register)
            }
        }

        cpu_step_state_impl!($name);
    };
}

condition_jump!(Bcc, |r: &Register| !r.get_c());
condition_jump!(Bcs, Register::get_c);
condition_jump!(Beq, Register::get_z);
condition_jump!(Bmi, Register::get_n);
condition_jump!(Bne, |r: &Register| !r.get_z());
condition_jump!(Bpl, |r: &Register| !r.get_n());
condition_jump!(Bvc, |r: &Register| !r.get_v());
condition_jump!(Bvs, Register::get_v);

pub(crate) trait ConditionJump {
    fn condition(register: &Register) -> bool;

    fn exec_opcode(
        &mut self,
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut Cartridge,
        controller: &mut Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        match core.register.get_opstep() {
            1 => {
                core.register.set_crossed(true);
                core.register.set_interrupt(core.interrupt.executing);
                if !Self::condition(&core.register) {
                    return exit_opcode(core);
                }
                // dummy read
                read_dummy_current(core, ppu, cartridge, controller, apu);
                let pc = core.register.get_pc() as usize;
                core.register
                    .set_crossed(page_crossed(core.register.get_opaddr(), pc));
            }
            2 => {
                if !core.register.get_crossed() {
                    core.register.set_pc(core.register.get_opaddr() as u16);
                    if !core.register.get_interrupt() {
                        core.interrupt.executing = false;
                    }
                    return exit_opcode(core);
                }
                // dummy read
                read_dummy_current(core, ppu, cartridge, controller, apu);

                core.register.set_pc(core.register.get_opaddr() as u16);
            }
            _ => {
                return exit_opcode(core);
            }
        }
        CpuStepStateEnum::Continue
    }
}
