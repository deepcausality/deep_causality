// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

use std::error::Error;
use deep_causality::prelude::ContextIndexError;

#[test]
fn test_context_index_error_creation() {
    let error_msg = "test error message";
    let error = ContextIndexError::new(error_msg.to_string());
    assert_eq!(error.0, error_msg);
}

#[test]
fn test_context_index_error_display() {
    let error_msg = "test error message";
    let error = ContextIndexError::new(error_msg.to_string());
    assert_eq!(format!("{}", error), format!("ContextIndexError: {}", error_msg));
}

#[test]
fn test_context_index_error_debug() {
    let error_msg = "test error message";
    let error = ContextIndexError::new(error_msg.to_string());
    assert_eq!(format!("{:?}", error), format!("ContextIndexError({:?})", error_msg));
}

#[test]
fn test_context_index_error_is_error() {
    let error = ContextIndexError::new("test".to_string());
    let is_error: &dyn Error = &error;
    assert!(is_error.is::<ContextIndexError>());
}