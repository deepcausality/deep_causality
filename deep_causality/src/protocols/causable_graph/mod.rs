use std::collections::HashMap;
use crate::errors::CausalityGraphError;
use crate::prelude::{Causable, IdentificationValue, NodeIndex, NumericalValue};

pub trait CausableGraph<T>
    where
        T: Causable + PartialEq,
{
    // Root Node
    fn add_root_causaloid(&mut self, value: T) -> NodeIndex;
    fn contains_root_causaloid(&self) -> bool;
    fn get_root_causaloid(&self) -> Option<&T>;
    fn get_root_index(&self) -> Option<NodeIndex>;

    // Nodes
    fn add_causaloid(&mut self, value: T) -> NodeIndex;
    fn contains_causaloid(&self, index: NodeIndex) -> bool;
    fn get_causaloid(&self, index: NodeIndex) -> Option<&T>;
    fn remove_causaloid(&mut self, index: NodeIndex);

    // Edges
    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex);
    fn add_edg_with_weight(&mut self, a: NodeIndex, b: NodeIndex, weight: u64);

    fn contains_edge(&self, a: NodeIndex, b: NodeIndex) -> bool;
    fn remove_edge(&mut self, a: NodeIndex, b: NodeIndex);

    // Utils
    fn all_active(&self) -> bool;
    fn number_active(&self) -> NumericalValue;
    fn percent_active(&self) -> NumericalValue;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
    fn count_edges(&self) -> usize;
    fn count_nodes(&self) -> usize;
}

/// Describes signatures for causal reasoning and explaining
/// in causality hyper graph.
pub trait CausableGraphReasoning<T>
    where
        T: Causable + PartialEq,
{
    /// Explains the line of reasoning across the entire graph.
    ///
    /// Returns: String representing the explanation or an error
    fn explain_all_causes(
        &self
    )
        -> Result<String, CausalityGraphError>;

    /// Explains the line of reasoning across a subgraph starting from a given node index until
    /// the end of the graph.
    ///
    /// index: NodeIndex - index of the starting node
    ///
    /// Returns: String representing the explanation or an error
    fn explain_subgraph_from_cause(
        &self,
        start_index: NodeIndex,
    )
        -> Result<String, CausalityGraphError>;


    /// Explains the line of reasoning of the shortest sub-graph
    /// between a start and stop cause.
    ///
    /// start_index: NodeIndex - index of the start cause
    /// stop_index: NodeIndex - index of the stop cause
    ///
    /// Returns: String representing the explanation or an error
    fn explain_shortest_path_between_causes(
        &self,
        start_index: NodeIndex,
        stop_index: NodeIndex,
    )
        -> Result<String, CausalityGraphError>;

    /// Reason over the entire graph.
    ///
    /// data: &[NumericalValue] - data applied to the subgraph
    /// Optional: data_index - provide when the data have a different index sorting than
    /// the causaloids.
    ///
    /// Conventionally, the index of the causaloid is matched to the
    /// index of the data so that data at index i get applied to causaloid i.
    /// If, for any reason, the data use a different index, the the optional data_index
    /// is used to match a causaloid i to its data at a (different) index n.
    ///
    /// Returns Result either true or false in case of successful reasoning or
    /// a CausalityGraphError in case of failure.
    fn reason_all_causes(
        &self,
        data: &[NumericalValue],
        data_index: Option<&HashMap<IdentificationValue, IdentificationValue>>,
    )
        -> Result<bool, CausalityGraphError>;

    /// Reason over a subgraph starting from a given node index.
    ///
    /// start_index: NodeIndex - index of the starting node
    /// data: &[NumericalValue] - data applied to the subgraph
    /// Optional: data_index - provide when the data have a different index sorting than
    /// the causaloids.
    ///
    /// Conventionally, the index of the causaloid is matched to the
    /// index of the data so that data at index i get applied to causaloid i.
    /// If, for any reason, the data use a different index, the the optional data_index
    /// is used to match a causaloid i to its data at a (different) index n.
    ///
    /// Returns Result either true or false in case of successful reasoning or
    /// a CausalityGraphError in case of failure.
    fn reason_subgraph_from_cause(
        &self,
        start_index: NodeIndex,
        data: &[NumericalValue],
        data_index: Option<&HashMap<IdentificationValue, IdentificationValue>>,
    )
        -> Result<bool, CausalityGraphError>;

    /// Reason over the shortest subgraph spanning between a start and stop cause.
    ///
    /// start_index: NodeIndex - index of the start cause
    /// stop_index: NodeIndex - index of the stop cause
    /// data: &[NumericalValue] - data applied to the subgraph
    /// Optional: data_index - provide when the data have a different index sorting than
    /// the causaloids.
    ///
    /// Conventionally, the index of the causaloid is matched to the
    /// index of the data so that data at index i get applied to causaloid i.
    /// If, for any reason, the data use a different index, the the optional data_index
    /// is used to match a causaloid i to its data at a (different) index n.
    ///
    /// Returns Result either true or false in case of successful reasoning or
    /// a CausalityGraphError in case of failure.
    fn reason_shortest_path_between_causes(
        &self,
        start_index: NodeIndex,
        stop_index: NodeIndex,
        data: &[NumericalValue],
        data_index: Option<&HashMap<IdentificationValue, IdentificationValue>>,
    )
        -> Result<bool, CausalityGraphError>;

    /// Reason over single node given by its index
    ///
    /// index: NodeIndex - index of the node
    ///
    /// Returns Result either true or false in case of successful reasoning or
    /// a CausalityGraphError in case of failure.
    fn reason_single_cause(
        &self,
        index: NodeIndex,
        data: &[NumericalValue],
    )
        -> Result<bool, CausalityGraphError>;
}