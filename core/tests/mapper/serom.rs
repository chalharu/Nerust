// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

#[test]
fn serom() {
    run_test!(
        "mapper/serom/serom.nes",
        ScenarioLeaf::check_screen(20, 0x66F3_C603_B111_9162)
    );
}
