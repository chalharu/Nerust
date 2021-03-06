// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub(crate) struct Nop;

impl CpuStepState for Nop {
    fn exec(
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut dyn Cartridge,
        controller: &mut dyn Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        match core.internal_stat.get_step() {
            1 => {
                let pc = core.register.get_pc() as usize;
                let _ = core
                    .memory
                    .read(pc, ppu, cartridge, controller, apu, &mut core.interrupt);
            }
            _ => {
                return exit_opcode(core);;
            }
        }
        CpuStepStateEnum::Continue
    }
}

pub(crate) struct Kil;

impl CpuStepState for Kil {
    fn exec(
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut dyn Cartridge,
        controller: &mut dyn Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        match core.internal_stat.get_step() {
            1 | 2 => {
                let pc = core.register.get_pc() as usize;
                let _ = core
                    .memory
                    .read(pc, ppu, cartridge, controller, apu, &mut core.interrupt);
            }
            _ => {
                return exit_opcode(core);
            }
        }
        CpuStepStateEnum::Continue
    }
}
