// SPDX-License-Identifier: MIT
// Copyright (c) "2024" . The DeepCausality Authors. All Rights Reserved.

pub trait Indexable {
    /// Gets the index for the provided key from either the current or previous
    /// index map, depending on the value of `current`.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to look up in the index map
    /// * `current` - Whether to check the current or previous index map
    ///
    /// # Returns
    ///
    /// Returns the index for the provided key if it exists, otherwise returns None.
    ///
    fn get_index(&self, key: &usize, current: bool) -> Option<&usize>;

    /// Sets the index for the provided key in either the current or previous
    /// index map, depending on the value of `current`.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to insert into the index map
    /// * `index` - The index value to associate with the key
    /// * `current` - Whether to insert into the current or previous index map
    ///
    /// If the key already exists in the chosen index map, the existing value
    /// will be overwritten with the provided `index` value.
    ///
    fn set_index(&mut self, key: usize, index: usize, current: bool);
}
