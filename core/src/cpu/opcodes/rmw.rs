// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

accumulate_memory!(Isc, |r: &mut Register, v: u8| {
    let result = v.wrapping_add(1);
    let a = u16::from(r.get_a());
    let b = u16::from(result);
    let c = if r.get_c() { 0 } else { 1 };
    let d = a.wrapping_sub(b).wrapping_sub(c);
    let result2 = (d & 0xFF) as u8;
    r.set_nz_from_value(result2);
    r.set_a(result2);
    r.set_c(d <= 0xFF);
    r.set_v((a ^ b) & 0x80 != 0 && (a ^ d) & 0x80 != 0);
    result
});
accumulate_memory!(Dcp, |r: &mut Register, v: u8| {
    let data = v.wrapping_sub(1);
    let a = r.get_a();
    r.set_nz_from_value(a.wrapping_sub(data));
    r.set_c(a >= data);
    data
});
accumulate_memory!(Slo, |r: &mut Register, v: u8| {
    r.set_c(v & 0x80 == 0x80);
    let a = r.get_a();
    let result = v << 1;
    r.set_a(a | result);
    r.set_nz_from_value(a | result);
    result
});
accumulate_memory!(Rla, |r: &mut Register, v: u8| {
    let c = if r.get_c() { 1 } else { 0 };
    r.set_c(v & 0x80 != 0);
    let wd = (v << 1) | c;
    let value = wd & r.get_a();
    r.set_a(value);
    r.set_nz_from_value(value);
    wd
});
accumulate_memory!(Sre, |r: &mut Register, v: u8| {
    r.set_c(v & 0x01 != 0);
    let data = v >> 1;
    let value = r.get_a() ^ data;
    r.set_a(value);
    r.set_nz_from_value(value);
    data
});
accumulate_memory!(Rra, |r: &mut Register, v: u8| {
    let value = v >> 1 | if r.get_c() { 0x80 } else { 0 };

    let a = usize::from(r.get_a());
    let b = usize::from(value);
    let c = usize::from(v & 0x01);
    let d = a + b + c;
    let result = (d & 0xFF) as u8;
    r.set_nz_from_value(result);
    r.set_a(result);
    r.set_c(d > 0xFF);
    r.set_v((a ^ b) & 0x80 == 0 && (a ^ d) & 0x80 != 0);
    value
});
