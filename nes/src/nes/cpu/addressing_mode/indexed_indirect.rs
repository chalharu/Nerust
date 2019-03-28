// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub(crate) struct IndexedIndirect {
    ind_address: usize,
    address_low: u8,
    step: usize,
}

impl IndexedIndirect {
    pub fn new() -> Self {
        Self {
            ind_address: 0,
            address_low: 0,
            step: 0,
        }
    }
}

impl CpuStepState for IndexedIndirect {
    fn entry(
        &mut self,
        _core: &mut Core,
        _ppu: &mut Ppu,
        _cartridge: &mut Cartridge,
        _controller: &mut Controller,
        _apu: &mut Apu,
    ) {
        self.step = 0;
    }

    fn exec(
        &mut self,
        core: &mut Core,
        ppu: &mut Ppu,
        cartridge: &mut Cartridge,
        controller: &mut Controller,
        apu: &mut Apu,
    ) -> CpuStepStateEnum {
        self.step += 1;
        match self.step {
            1 => {
                let pc = core.register.get_pc() as usize;
                self.address_low =
                    core.memory
                        .read(pc, ppu, cartridge, controller, apu, &mut core.interrupt);
            }
            2 => {
                let _ = core.memory.read_next(
                    &mut core.register,
                    ppu,
                    cartridge,
                    controller,
                    apu,
                    &mut core.interrupt,
                );
                self.ind_address =
                    usize::from(self.address_low.wrapping_add(core.register.get_x()));
            }
            3 => {
                self.address_low = core.memory.read(
                    self.ind_address,
                    ppu,
                    cartridge,
                    controller,
                    apu,
                    &mut core.interrupt,
                );
            }
            4 => {
                let address_high = usize::from(core.memory.read(
                    self.ind_address.wrapping_add(1) & 0xFF,
                    ppu,
                    cartridge,
                    controller,
                    apu,
                    &mut core.interrupt,
                ));
                core.register
                    .set_opaddr((address_high << 8) | usize::from(self.address_low));
            }
            _ => {
                return CpuStepStateEnum::Exit;
            }
        }
        CpuStepStateEnum::Continue
    }

    fn exit(
        &mut self,
        core: &mut Core,
        _ppu: &mut Ppu,
        _cartridge: &mut Cartridge,
        _controller: &mut Controller,
        _apu: &mut Apu,
    ) -> CpuStatesEnum {
        core.opcode_tables.get(core.register.get_opcode())
    }
}
