// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

use crate::ring_buffer::prelude::{AtomicSequence, Sequence};
use std::borrow::Borrow;

pub fn get_min_cursor_sequence<S, A>(sequences: &[S]) -> Sequence
where
    S: Borrow<A>,
    A: AtomicSequence,
{
    sequences
        .iter()
        .map(|s| s.borrow().get())
        .min()
        .unwrap_or_default()
}
