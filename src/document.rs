// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(missing_docs)]

use attributes::BasicAttributes;
use elements;

pub enum StructureContentModel {
    BodyElements(Vec<Box<elements::BodyElement>>),
    Topic(elements::Topic),
    Sidebar(elements::Sidebar),
    Section(elements::SectionElement),
}

pub struct Document {
    pub common: BasicAttributes,
    pub title: Option<elements::Title>,
    pub subtitle: Option<elements::Subtitle>,
    pub header: Vec<Box<elements::BodyElement>>,
    pub footer: Vec<Box<elements::BodyElement>>,
    pub info: Vec<Box<elements::BibliographicElement>>,
    pub content: Vec<Box<StructureContentModel>>,
}
