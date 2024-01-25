use std::hash::Hash;

// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.
use crate::prelude::Identifiable;
use crate::types::context_types::node_types::data::Data;

impl<T> Identifiable for Data<T>
where
    T: Default + Copy + Clone + Hash + Eq + PartialEq,
{
    fn id(&self) -> u64 {
        self.id
    }
}
