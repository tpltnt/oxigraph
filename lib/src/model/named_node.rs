use crate::Result;
use rio_api::iri::Iri;
use rio_api::model as rio;
use std::fmt;

/// A RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri)
///
/// The default string formatter is returning a N-Triples, Turtle and SPARQL compatible representation:
/// ```
/// use oxigraph::model::NamedNode;
///
/// assert_eq!(
///     "<http://example.com/foo>",
///     NamedNode::parse("http://example.com/foo").unwrap().to_string()
/// )
/// ```
///
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Hash)]
pub struct NamedNode {
    iri: String,
}

impl NamedNode {
    /// Builds and validate a RDF [IRI](https://www.w3.org/TR/rdf11-concepts/#dfn-iri)
    pub fn parse(iri: impl Into<String>) -> Result<Self> {
        Ok(Self::new_from_iri(Iri::parse(iri.into())?))
    }

    pub(crate) fn new_from_iri(iri: Iri<String>) -> Self {
        Self::new_from_string(iri.into_inner())
    }

    pub(crate) fn new_from_string(iri: impl Into<String>) -> Self {
        Self { iri: iri.into() }
    }

    pub fn as_str(&self) -> &str {
        self.iri.as_str()
    }

    pub fn into_string(self) -> String {
        self.iri
    }
}

impl fmt::Display for NamedNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        rio::NamedNode::from(self).fmt(f)
    }
}

impl<'a> From<&'a NamedNode> for rio::NamedNode<'a> {
    fn from(node: &'a NamedNode) -> Self {
        rio::NamedNode { iri: node.as_str() }
    }
}
