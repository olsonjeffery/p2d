// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

pub struct TextInputDialog<TFont, TBox> {
    input_state: ~str,
    bg_color: (u8, u8, u8),
    cursor: ~str,
    coords: (int, int),
    box_size: (uint, uint),
    text_gap: uint
}

