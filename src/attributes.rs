// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Attributes shared by all elements.
pub struct BasicAttributes {
    /// Unique identifiers, typically assigned by the system.
    pub ids: Vec<String>,
    /// An identifier assigned in the markup.
    pub names: Vec<String>,
    /// The source of this document or fragment.
    pub source: String,
    /// Used to transmit individuality information forward.
    pub classes: Vec<String>,
}

/// Types of references.
pub enum ReferenceTarget {
    /// External reference to a URI / URL.
    URI(String),
    /// Internal reference to the `id` attribute of an element.
    ID(String),
    /// Internal reference to the `name` attribute of an element. On
    /// a `target` element, this indicates an indirect target which
    /// may resolve to either an internal or an external reference.
    Name(String),
}

/// Should whitespace be preserved or not?
pub enum FixedSpace {
    /// Collapse whitespace.
    Default,
    /// Preserve whitespace.
    Preserve,
}
