// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code,missing_docs)]

use attributes::BasicAttributes;

pub trait Element {
    fn basic_attrs(&self) -> &BasicAttributes;
    fn basic_attrs_mut(&mut self) -> &mut BasicAttributes;

    fn classes(&self) -> &Vec<String> {
        &self.basic_attrs().classes
    }

    fn classes_mut(&mut self) -> &mut Vec<String> {
        &mut self.basic_attrs_mut().classes
    }

    fn ids(&self) -> &Vec<String> {
        &self.basic_attrs().ids
    }

    fn ids_mut(&mut self) -> &mut Vec<String> {
        &mut self.basic_attrs_mut().ids
    }

    fn names(&self) -> &Vec<String> {
        &self.basic_attrs().names
    }

    fn names_mut(&mut self) -> &mut Vec<String> {
        &mut self.basic_attrs_mut().names
    }

    fn source(&self) -> &String {
        &self.basic_attrs().source
    }

    fn source_mut(&mut self) -> &mut String {
        &mut self.basic_attrs_mut().source
    }
}


pub trait BibliographicElement {}
pub trait BodyElement {}
pub trait InlineElement {}
pub trait SectionElement {}

macro_rules! impl_element {
   ($element:ident) => {
       impl Element for $element {
           fn basic_attrs(&self) -> &BasicAttributes { &self.basic_attributes }
           fn basic_attrs_mut(&mut self) -> &mut BasicAttributes { &mut self.basic_attributes }
       }
   }
}

pub struct Abbreviation {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Abbreviation);
impl InlineElement for Abbreviation {}

pub struct Acronym {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Acronym);
impl InlineElement for Acronym {}

pub struct Address {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Address);
impl BibliographicElement for Address {}

pub struct Admonition {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Admonition);
impl BodyElement for Admonition {}

pub struct Attention {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Attention);
impl BodyElement for Attention {}

pub struct Author {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Author);
impl BibliographicElement for Author {}

pub struct Authors {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Authors);
impl BibliographicElement for Authors {}

pub struct BlockQuote {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(BlockQuote);
impl BodyElement for BlockQuote {}

pub struct BulletList {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(BulletList);
impl BodyElement for BulletList {}

pub struct Caution {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Caution);
impl BodyElement for Caution {}

pub struct Citation {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Citation);
impl BodyElement for Citation {}

pub struct CitationReference {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(CitationReference);
impl InlineElement for CitationReference {}

pub struct Comment {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Comment);
impl BodyElement for Comment {}

pub struct Compound {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Compound);
impl BodyElement for Compound {}

pub struct Contact {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Contact);
impl BibliographicElement for Contact {}

pub struct Container {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Container);
impl BodyElement for Container {}

pub struct Copyright {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Copyright);
impl BibliographicElement for Copyright {}

pub struct Date {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Date);
impl BibliographicElement for Date {}

pub struct DoctestBlock {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(DoctestBlock);
impl BodyElement for DoctestBlock {}

pub struct Emphasis {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Emphasis);
impl InlineElement for Emphasis {}

pub struct Figure {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Figure);
impl BodyElement for Figure {}

pub struct Footnote {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Footnote);
impl BodyElement for Footnote {}

pub struct Danger {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Danger);
impl BodyElement for Danger {}

pub struct DefinitionList {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(DefinitionList);
impl BodyElement for DefinitionList {}

pub struct EnumeratedList {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(EnumeratedList);
impl BodyElement for EnumeratedList {}

pub struct Error {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Error);
impl BodyElement for Error {}

pub struct Field {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Field);
impl BibliographicElement for Field {}

pub struct FieldList {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(FieldList);
impl BodyElement for FieldList {}

pub struct FootnoteReference {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(FootnoteReference);
impl InlineElement for FootnoteReference {}

pub struct Generated {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Generated);
impl InlineElement for Generated {}

pub struct Hint {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Hint);
impl BodyElement for Hint {}

pub struct Image {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Image);
impl BodyElement for Image {}
impl InlineElement for Image {}

pub struct Important {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Important);
impl BodyElement for Important {}

pub struct Inline {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Inline);
impl InlineElement for Inline {}

pub struct LiteralBlock {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(LiteralBlock);
impl BodyElement for LiteralBlock {}

pub struct LineBlock {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(LineBlock);
impl BodyElement for LineBlock {}

pub struct ListItem {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(ListItem);

pub struct Literal {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Literal);
impl InlineElement for Literal {}

pub struct Note {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Note);
impl BodyElement for Note {}

pub struct OptionList {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(OptionList);
impl BodyElement for OptionList {}

pub struct Organization {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Organization);
impl BibliographicElement for Organization {}

pub struct Paragraph {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Paragraph);
impl BodyElement for Paragraph {}

pub struct Pending {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Pending);
impl BodyElement for Pending {}

pub struct Problematic {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Problematic);
impl InlineElement for Problematic {}

pub struct Raw {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Raw);
impl BodyElement for Raw {}
impl InlineElement for Raw {}

pub struct Reference {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Reference);
impl BodyElement for Reference {}
impl InlineElement for Reference {}

pub struct Revision {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Revision);
impl BibliographicElement for Revision {}

pub struct Rubric {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Rubric);
impl BodyElement for Rubric {}

pub struct Section {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Section);
impl SectionElement for Section {}

pub struct Sidebar {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Sidebar);

pub struct Status {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Status);
impl BibliographicElement for Status {}

pub struct Strong {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Strong);
impl InlineElement for Strong {}

pub struct Subscript {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Subscript);
impl InlineElement for Subscript {}

pub struct SubstitutionDefinition {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(SubstitutionDefinition);
impl BodyElement for SubstitutionDefinition {}

pub struct SubstitutionReference {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(SubstitutionReference);
impl InlineElement for SubstitutionReference {}

pub struct Superscript {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Superscript);
impl InlineElement for Superscript {}

pub struct Subtitle {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Subtitle);

pub struct SystemMessage {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(SystemMessage);
impl BodyElement for SystemMessage {}

pub struct Table {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Table);
impl BodyElement for Table {}

pub struct Target {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Target);
impl BodyElement for Target {}
impl InlineElement for Target {}

pub struct Tip {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Tip);
impl BodyElement for Tip {}

pub struct Title {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Title);

pub struct TitleReference {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(TitleReference);
impl InlineElement for TitleReference {}

pub struct Topic {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Topic);

pub struct Version {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Version);
impl BibliographicElement for Version {}

pub struct Warning {
    /// Attributes common to all elements.
    pub basic_attributes: BasicAttributes,
}
impl_element!(Warning);
impl BodyElement for Warning {}
