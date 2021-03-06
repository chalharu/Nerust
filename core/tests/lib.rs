// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod macros;

mod apu;
mod cpu;
mod input;
mod mapper;
mod ppu;

use self::ButtonCode::*;
use self::PadState::{Pressed, Released};
use self::StandardControllerButtonCode::Pad1;
use crc::crc64;
use nerust_core::controller::standard_controller::{Buttons, StandardController};
use nerust_core::Core;
use nerust_screen_buffer::ScreenBuffer;
use nerust_screen_filter::FilterType;
use nerust_screen_traits::LogicalSize;
use nerust_sound_traits::MixerInput;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

struct TestMixer;

impl MixerInput for TestMixer {
    fn push(&mut self, _data: f32) {}
}

struct ScenarioRunner {
    screen_buffer: ScreenBuffer,
    core: Core,
    controller: StandardController,
    mixer: TestMixer,
    frame_counter: u64,
    pad1: Buttons,
    pad2: Buttons,
}

impl ScenarioRunner {
    fn new<I: Iterator<Item = u8>>(input: &mut I) -> Self {
        Self {
            screen_buffer: ScreenBuffer::new(
                FilterType::None,
                LogicalSize {
                    width: 256,
                    height: 240,
                },
            ),
            core: Core::new(input).unwrap(),
            controller: StandardController::new(),
            mixer: TestMixer,
            frame_counter: 0,
            pad1: Buttons::empty(),
            pad2: Buttons::empty(),
        }
    }

    fn run(&mut self, scenario: Scenario) {
        let mut tmpscenario = scenario.0.clone();
        tmpscenario.sort_by(|a, b| a.frame_number.cmp(&b.frame_number));
        let mut scenario = VecDeque::from(tmpscenario);

        while !scenario.is_empty() {
            self.on_update();
            while !scenario.is_empty() && scenario[0].frame_number == self.frame_counter {
                match scenario.pop_front().unwrap().operation {
                    ScenarioOperation::CheckScreen { hash } => {
                        let mut hasher = crc64::Digest::new(crc64::ECMA);
                        self.screen_buffer.hash(&mut hasher);
                        if hasher.finish() != hash {
                            panic!(format!(
                                "assertion failed: `(left == right)` \
                                 (left: `0x{:016X}`, right: `0x{:016X}` frame: {})",
                                hasher.finish(),
                                hash,
                                self.frame_counter
                            ));
                        };
                    }
                    ScenarioOperation::Reset => {
                        self.core.reset();
                    }
                    ScenarioOperation::StandardControllerInput { code, state } => match code {
                        Pad1(code) => {
                            self.pad1 = match state {
                                Pressed => self.pad1 | Buttons::from(code),
                                Released => self.pad1 & !(Buttons::from(code)),
                            };
                            self.controller.set_pad1(self.pad1);
                        }
                        StandardControllerButtonCode::Pad2(code) => {
                            self.pad2 = match state {
                                Pressed => self.pad2 | Buttons::from(code),
                                Released => self.pad2 & !(Buttons::from(code)),
                            };
                            self.controller.set_pad2(self.pad2);
                        }
                    },
                }
            }
        }
    }

    fn on_update(&mut self) {
        while !self.core.step(
            &mut self.screen_buffer,
            &mut self.controller,
            &mut self.mixer,
        ) {}
        self.frame_counter += 1;
    }
}

#[derive(Debug, Copy, Clone)]
enum ButtonCode {
    A,
    B,
    SELECT,
    START,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<ButtonCode> for Buttons {
    fn from(v: ButtonCode) -> Self {
        match v {
            A => Buttons::A,
            B => Buttons::B,
            SELECT => Buttons::SELECT,
            START => Buttons::START,
            UP => Buttons::UP,
            DOWN => Buttons::DOWN,
            LEFT => Buttons::LEFT,
            RIGHT => Buttons::RIGHT,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum StandardControllerButtonCode {
    Pad1(ButtonCode),
    #[allow(dead_code)]
    Pad2(ButtonCode),
}

#[derive(Debug, Copy, Clone)]
enum PadState {
    Pressed,
    Released,
}

#[derive(Debug, Copy, Clone)]
enum ScenarioOperation {
    CheckScreen {
        hash: u64,
    },
    Reset,
    StandardControllerInput {
        code: StandardControllerButtonCode,
        state: PadState,
    },
}
impl ScenarioOperation {
    pub(crate) fn check_screen(hash: u64) -> Self {
        ScenarioOperation::CheckScreen { hash }
    }
    pub(crate) fn standard_controller(code: StandardControllerButtonCode, state: PadState) -> Self {
        ScenarioOperation::StandardControllerInput { code, state }
    }
    pub(crate) fn reset() -> Self {
        ScenarioOperation::Reset
    }
}

#[derive(Debug, Copy, Clone)]
struct ScenarioLeaf {
    frame_number: u64,
    operation: ScenarioOperation,
}

impl ScenarioLeaf {
    pub(crate) fn new(frame_number: u64, operation: ScenarioOperation) -> Self {
        Self {
            frame_number,
            operation,
        }
    }
    pub(crate) fn check_screen(frame_number: u64, hash: u64) -> Self {
        Self::new(frame_number, ScenarioOperation::check_screen(hash))
    }
    pub(crate) fn standard_controller(
        frame_number: u64,
        code: StandardControllerButtonCode,
        state: PadState,
    ) -> Self {
        Self::new(
            frame_number,
            ScenarioOperation::standard_controller(code, state),
        )
    }
    pub(crate) fn reset(frame_number: u64) -> Self {
        Self::new(frame_number, ScenarioOperation::reset())
    }
}

struct Scenario(Vec<ScenarioLeaf>);

impl Scenario {
    pub(crate) fn new(senarios: &[ScenarioLeaf]) -> Self {
        Scenario(senarios.to_vec())
    }
}

// mod full_palette {
//     use super::*;

//     #[test]
//     fn flowing_palette() {
//         run_test!(
//             "full_palette/flowing_palette.nes",
//             ScenarioLeaf::check_screen(30, 0xE31E_B517_2247_2E30)
//         );
//     }

//     #[test]
//     fn full_palette_smooth() {
//         run_test!(
//             "full_palette/full_palette_smooth.nes",
//             ScenarioLeaf::check_screen(30, 0xE31E_B517_2247_2E30)
//         );
//     }

//     #[test]
//     fn full_palette() {
//         run_test!(
//             "full_palette/full_palette.nes",
//             ScenarioLeaf::check_screen(30, 0xE31E_B517_2247_2E30)
//         );
//     }
// }

// mod nmi_sync {
//     use super::*;

//     #[test]
//     fn demo_ntsc() {
//         run_test!(
//             "nmi_sync/demo_ntsc.nes",
//             ScenarioLeaf::check_screen(30, 0xE31E_B517_2247_2E30)
//         );
//     }

//     #[test]
//     fn demo_pal() {
//         run_test!(
//             "nmi_sync/demo_pal.nes",
//             ScenarioLeaf::check_screen(30, 0xE31E_B517_2247_2E30)
//         );
//     }
// }
