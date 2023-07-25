// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use deep_causality::prelude::*;
use deep_causality::utils::test_utils::get_test_causality_array;

#[test]
fn test_all_active()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
}

#[test]
fn test_number_active()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
    assert_eq!(10.0, col.number_active());
}

#[test]
fn test_percent_active()
{
    let col = get_test_causality_array();
    assert!(!col.get_all_causes_true());

    let obs = 0.99;
    for cause in &col {
        cause.verify_single_cause(&obs).expect("verify failed");
    }
    assert!(col.get_all_causes_true());
    assert_eq!(10.0, col.number_active());
    assert_eq!(100.0, col.percent_active());
}

#[test]
fn test_get_all_items() {
    let col = get_test_causality_array();
    let all_items = col.get_all_items();

    let exp_len = col.len();
    let actual_len = all_items.len();
    assert_eq!(exp_len, actual_len);
}

#[test]
fn test_len()
{
    let col = get_test_causality_array();
    assert_eq!(10, col.len());
}

#[test]
fn test_is_empty()
{
    let col = get_test_causality_array();
    assert!(!col.is_empty());
}

#[test]
fn test_to_vec()
{
    let col = get_test_causality_array();
    assert_eq!(10, col.to_vec().len());
}