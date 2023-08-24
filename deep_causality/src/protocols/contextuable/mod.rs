// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

use crate::errors::ContextIndexError;
use crate::prelude::{Contextoid, ContextoidType, Identifiable, RelationKind, TimeScale};

pub trait Datable: Identifiable {}

// Make Spatial and Temporable trait generic
// Issue #42
// https://github.com/deepcausality-rs/deep_causality/issues/42

// See also:
// https://stackoverflow.com/questions/72523459/how-can-i-specify-trait-bounds-with-or-logic

pub trait Temporable: Identifiable //  where T: Default + Add<T, Output=T> + Mul<T, Output=T> // ...
{
    fn time_scale(&self) -> TimeScale;
    fn time_unit(&self) -> u32;
}

pub trait Spatial: Identifiable //  where T: Default + Add<T, Output=T> + Mul<T, Output=T> // ...
{
    fn x(&self) -> i64;
    fn y(&self) -> i64;
    fn z(&self) -> i64;
}

pub trait SpaceTemporal: Identifiable + Spatial + Temporable
//  where T: Default + Add<T, Output=T> + Mul<T, Output=T> // ...
{
    fn t(&self) -> u64; // returns 4th dimension, t
}

pub trait Contextuable<D, S, T, ST>: Identifiable
where
    D: Datable,
    S: Spatial,
    ST: SpaceTemporal,
    T: Temporable,
{
    fn vertex_type(&self) -> &ContextoidType<D, S, T, ST>;
}

pub trait ContextuableGraph<'l, D, S, T, ST>
where
    D: Datable,
    S: Spatial,
    ST: SpaceTemporal,
    T: Temporable,
{
    fn add_node(&mut self, value: Contextoid<D, S, T, ST>) -> usize;
    fn contains_node(&self, index: usize) -> bool;
    fn get_node(&self, index: usize) -> Option<&Contextoid<D, S, T, ST>>;
    fn remove_node(&mut self, index: usize) -> Result<(), ContextIndexError>;
    fn add_edge(
        &mut self,
        a: usize,
        b: usize,
        weight: RelationKind,
    ) -> Result<(), ContextIndexError>;
    fn contains_edge(&self, a: usize, b: usize) -> bool;
    fn remove_edge(&mut self, a: usize, b: usize) -> Result<(), ContextIndexError>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
}
