// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod screen_buffer;
mod screen_buffer_unit;

pub use screen_buffer::ScreenBuffer;

fn allocate<T: Default + Clone>(size: usize) -> Box<[T]> {
    // let mut buffer = Vec::with_capacity(size);
    // unsafe {
    //     buffer.set_len(size);
    // }
    let buffer = vec![T::default(); size];
    buffer.into_boxed_slice()
}
