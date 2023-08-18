// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . The DeepCausality Authors. All Rights Reserved.

use deep_causality::prelude::{Causaloid, CausaloidGraph, Dataoid, Spaceoid, SpaceTempoid, Tempoid};
use deep_causality::protocols::causable_graph::graph::CausableGraph;

use crate::benchmarks::utils_shared;

const SMALL: usize = 9;
const MEDIUM: usize = 1_00;
const LARGE: usize = 1_000;

type CausalGraph<'l> = CausaloidGraph<Causaloid<'l, Dataoid, Spaceoid, Tempoid, SpaceTempoid>>;

pub fn get_small_linear_graph_and_data<'l>()
    -> (CausalGraph<'l>, [f64; SMALL + 1])
{ // Builds a linear graph: root -> a -> b -> c
    (build_linear_graph(SMALL), generate_sample_data())
}

pub fn get_medium_linear_graph_and_data<'l>()
    -> (CausalGraph<'l>, [f64; MEDIUM + 1])
{ // Builds a linear graph: root -> a -> b -> c ...
    (build_linear_graph(MEDIUM), generate_sample_data())
}

pub fn get_large_linear_graph_and_data<'l>()
    -> (CausalGraph<'l>, [f64; LARGE + 1])
{ // Builds a linear graph: root -> a -> b -> c ...
    (build_linear_graph(LARGE), generate_sample_data())
}

pub fn build_linear_graph<'l>(
    k: usize
)
    -> CausalGraph<'l>
{   // Builds a linear graph: root -> a -> b -> c
    let mut g = CausaloidGraph::new();

    let root_causaloid = utils_shared::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    let mut previous_idx = root_index;

    for _ in 0..k {
        // add a new causaloid and set current idx to it
        let causaloid = utils_shared::get_test_causaloid();
        let current_idx = g.add_causaloid(causaloid);

        // link current causaloid to previos causaloid
        g.add_edge(previous_idx, current_idx).expect("Failed to add edge");

        previous_idx = current_idx;
    }

    g
}

pub fn get_small_multi_cause_graph_and_data<'l>()
    -> (CausalGraph<'l>, [f64; 4 + 1])
{   // Builds a multi-layer cause graph:
    (build_multi_cause_graph(), generate_sample_data())
}

fn build_multi_cause_graph<'l>()
    -> CausalGraph<'l>
{
    // Builds a multi cause graph:
    //  root
    //  / \
    //  A B
    //  \ /
    //   C

    let mut g = CausaloidGraph::new();

    // Add root causaloid
    let root_causaloid = utils_shared::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    // Add causaloid A
    let causaloid = utils_shared::get_test_causaloid();
    let idx_a = g.add_causaloid(causaloid);

    // Link causaloid A to root causaloid
    g.add_edge(root_index, idx_a).expect("Failed to add edge between root and A");

    // Add causaloid B
    let causaloid = utils_shared::get_test_causaloid();
    let idx_b = g.add_causaloid(causaloid);

    // Link causaloid B to root causaloid
    g.add_edge(root_index, idx_b).expect("Failed to add edge between root and B");

    // Add causaloid C
    let causaloid = utils_shared::get_test_causaloid();
    let idx_c = g.add_causaloid(causaloid);

    // Link causaloid C to A
    g.add_edge(idx_a, idx_c).expect("Failed to add edge between A and C");

    // Link causaloid C to B
    g.add_edge(idx_b, idx_c).expect("Failed to add edge between C and B");

    g
}


// Generates a fixed sized array with sample data
fn generate_sample_data<const N: usize>()
    -> [f64; N]
{
    [0.99; N]
}
