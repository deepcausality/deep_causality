/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use crate::prelude::{CausableGraph, Causaloid, CausaloidGraph};
use crate::utils::{bench_utils_shared, test_utils};

const SMALL: usize = 100;
const MEDIUM: usize = 1_000;
const LARGE: usize = 10_000;

pub fn get_small_linear_graph_and_data()
    -> (CausaloidGraph<Causaloid>, [f64; SMALL + 1])
{ // Builds a linear graph: root -> a -> b -> c
    let k = SMALL;
    (build_linear_graph(k), bench_utils_shared::generate_sample_data(k))
}

pub fn get_medium_linear_graph_and_data()
    -> (CausaloidGraph<Causaloid>, [f64; MEDIUM + 1])
{ // Builds a linear graph: root -> a -> b -> c ...
    let k = MEDIUM;
    (build_linear_graph(k), bench_utils_shared::generate_sample_data(k))
}

pub fn get_large_linear_graph_and_data()
    -> (CausaloidGraph<Causaloid>, [f64; LARGE + 1])
{ // Builds a linear graph: root -> a -> b -> c ...
    let k = LARGE;
    (build_linear_graph(k), bench_utils_shared::generate_sample_data(k))
}

fn build_linear_graph(
    k: usize
)
    -> CausaloidGraph<Causaloid>
{   // Builds a linear graph: root -> a -> b -> c
    let mut g = CausaloidGraph::new();

    let root_causaloid = test_utils::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    let mut previous_idx = root_index;

    for _ in 0..k {
        // add a new causaloid and set current idx to it
        let causaloid = test_utils::get_test_causaloid();
        let current_idx = g.add_causaloid(causaloid);

        // link current causaloid to previos causaloid
        g.add_edge(previous_idx, current_idx);

        previous_idx = current_idx;
    }

    g
}

pub fn get_small_multi_cause_graph_and_data()
    -> (CausaloidGraph<Causaloid>, [f64; 4 + 1])
{   // Builds a multi-layer cause graph:
    let k = 4;
    (build_multi_cause_graph(), bench_utils_shared::generate_sample_data(k))
}

fn build_multi_cause_graph()
    -> CausaloidGraph<Causaloid>
{
    // Builds a multi cause graph:
    //  root
    //  / \
    //  A B
    //  \ /
    //   C

    let mut g = CausaloidGraph::new();

    // Add root causaloid
    let root_causaloid = test_utils::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    // Add causaloid A
    let causaloid = test_utils::get_test_causaloid();
    let idx_a = g.add_causaloid(causaloid);

    // Link causaloid A to root causaloid
    g.add_edge(root_index, idx_a);

    // Add causaloid B
    let causaloid = test_utils::get_test_causaloid();
    let idx_b = g.add_causaloid(causaloid);

    // Link causaloid B to root causaloid
    g.add_edge(root_index, idx_b);

    // Add causaloid C
    let causaloid = test_utils::get_test_causaloid();
    let idx_c = g.add_causaloid(causaloid);

    // Link causaloid C to A
    g.add_edge(idx_a, idx_c);

    // Link causaloid C to B
    g.add_edge(idx_b, idx_c);

    g
}

pub fn get_small_multi_layer_cause_graph_and_data()
    -> (CausaloidGraph<Causaloid>, [f64; 8 + 1])
{   // Builds a multi-layer cause graph:
    let k = 8;
    (build_multi_layer_cause_graph(), bench_utils_shared::generate_sample_data(k))
}

fn build_multi_layer_cause_graph()
    -> CausaloidGraph<Causaloid>
{
    // Builds a multi-layer cause graph:
    //    root
    //  /   |  \
    //  A   B   C
    // /\  /\  /\
    //D   E   F  G

    let mut g = CausaloidGraph::new();

    // Add root causaloid
    let root_causaloid = test_utils::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    // ### First layer ### //

    // Add causaloid A
    let causaloid = test_utils::get_test_causaloid();
    let idx_a = g.add_causaloid(causaloid);
    // Link causaloid A to root causaloid
    g.add_edge(root_index, idx_a);

    // Add causaloid B
    let causaloid = test_utils::get_test_causaloid();
    let idx_b = g.add_causaloid(causaloid);
    // Link causaloid B to root causaloid
    g.add_edge(root_index, idx_b);

    // Add causaloid C
    let causaloid = test_utils::get_test_causaloid();
    let idx_c = g.add_causaloid(causaloid);
    // Link causaloid C  to root causaloid
    g.add_edge(root_index, idx_c);

    // ### Second layer ### //

    // Add causaloid D
    let causaloid = test_utils::get_test_causaloid();
    let idx_d = g.add_causaloid(causaloid);
    // Link causaloid D  to A
    g.add_edge(idx_a, idx_d);

    // Add causaloid E
    let causaloid = test_utils::get_test_causaloid();
    let idx_e = g.add_causaloid(causaloid);
    // Link causaloid E  to A
    g.add_edge(idx_a, idx_e);
    // Link causaloid E  to B
    g.add_edge(idx_b, idx_e);

    // Add causaloid F
    let causaloid = test_utils::get_test_causaloid();
    let idx_f = g.add_causaloid(causaloid);
    // Link causaloid F  to B
    g.add_edge(idx_b, idx_f);
    // Link causaloid F  to C
    g.add_edge(idx_c, idx_f);

    // Add causaloid G
    let causaloid = test_utils::get_test_causaloid();
    let idx_g = g.add_causaloid(causaloid);
    // Link causaloid G  to C
    g.add_edge(idx_c, idx_g);

    g
}

pub fn get_left_imbalanced_cause_graph()
    -> (CausaloidGraph<Causaloid>, [f64; 6 + 1])
{   // Builds a multi-layer cause graph:
    let k = 6;
    (build_left_imbalanced_cause_graph(), bench_utils_shared::generate_sample_data(k))
}

fn build_left_imbalanced_cause_graph()
    -> CausaloidGraph<Causaloid>
{
    // Builds a multi-layer cause graph:
    //    root
    //  /   |  \
    //  A   B   C
    // /\
    //D  E

    let mut g = CausaloidGraph::new();

    // Add root causaloid
    let root_causaloid = test_utils::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    // ### First layer ### //

    // Add causaloid A
    let causaloid = test_utils::get_test_causaloid();
    let idx_a = g.add_causaloid(causaloid);
    // Link causaloid A to root causaloid
    g.add_edge(root_index, idx_a);

    // Add causaloid B
    let causaloid = test_utils::get_test_causaloid();
    let idx_b = g.add_causaloid(causaloid);
    // Link causaloid B to root causaloid
    g.add_edge(root_index, idx_b);

    // Add causaloid C
    let causaloid = test_utils::get_test_causaloid();
    let idx_c = g.add_causaloid(causaloid);
    // Link causaloid C  to root causaloid
    g.add_edge(root_index, idx_c);

    // ### Second layer ### //

    // Add causaloid D
    let causaloid = test_utils::get_test_causaloid();
    let idx_d = g.add_causaloid(causaloid);
    // Link causaloid D  to A
    g.add_edge(idx_a, idx_d);

    // Add causaloid E
    let causaloid = test_utils::get_test_causaloid();
    let idx_e = g.add_causaloid(causaloid);
    // Link causaloid E  to A
    g.add_edge(idx_a, idx_e);

    g
}

pub fn get_right_imbalanced_cause_graph()
    -> (CausaloidGraph<Causaloid>, [f64; 6 + 1])
{   // Builds a multi-layer cause graph:
    let k = 6;
    (build_right_imbalanced_cause_graph(), bench_utils_shared::generate_sample_data(k))
}

fn build_right_imbalanced_cause_graph()
    -> CausaloidGraph<Causaloid>
{
    // Builds a multi-layer cause graph:
    //    root
    //  /   |  \
    //  A   B   C
    //          /\
    //         D  E

    let mut g = CausaloidGraph::new();

    // Add root causaloid
    let root_causaloid = test_utils::get_test_causaloid();
    let root_index = g.add_root_causaloid(root_causaloid);

    // ### First layer ### //

    // Add causaloid A
    let causaloid = test_utils::get_test_causaloid();
    let idx_a = g.add_causaloid(causaloid);
    // Link causaloid A to root causaloid
    g.add_edge(root_index, idx_a);

    // Add causaloid B
    let causaloid = test_utils::get_test_causaloid();
    let idx_b = g.add_causaloid(causaloid);
    // Link causaloid B to root causaloid
    g.add_edge(root_index, idx_b);

    // Add causaloid C
    let causaloid = test_utils::get_test_causaloid();
    let idx_c = g.add_causaloid(causaloid);
    // Link causaloid C  to root causaloid
    g.add_edge(root_index, idx_c);

    // ### Second layer ### //

    // Add causaloid D
    let causaloid = test_utils::get_test_causaloid();
    let idx_d = g.add_causaloid(causaloid);
    // Link causaloid D  to C
    g.add_edge(idx_c, idx_d);

    // Add causaloid E
    let causaloid = test_utils::get_test_causaloid();
    let idx_e = g.add_causaloid(causaloid);
    // Link causaloid E  to C
    g.add_edge(idx_c, idx_e);

    g
}