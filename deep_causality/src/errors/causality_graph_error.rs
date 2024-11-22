// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

use std::error::Error;
use std::fmt;
use deep_causality_macros::Constructor;

#[derive(Constructor, Debug)]
pub struct CausalityGraphError(pub String);

impl Error for CausalityGraphError {}

impl fmt::Display for CausalityGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CausalityGraphError: {}", self.0)
    }
}
