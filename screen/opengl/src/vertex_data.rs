// Copyright (c) 2018 Mitsuharu Seki
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Vec2D;

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct VertexData {
    pub(crate) position: Vec2D,
    pub(crate) uv: Vec2D,
}

impl VertexData {
    pub(crate) fn new(position: Vec2D, uv: Vec2D) -> Self {
        Self { position, uv }
    }
}
