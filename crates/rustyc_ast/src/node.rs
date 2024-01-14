use rustyc_span::Span;

use crate::node_kind::NodeKind;

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    span: Span,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(kind: NodeKind, span: Span) -> Self {
        Self {
            kind,
            span,
            left: None,
            right: None,
        }
    }

    pub fn new_binary(kind: NodeKind, span: Span, left: Box<Node>, right: Box<Node>) -> Self {
        Self {
            kind,
            span,
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn get_kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }

    pub fn get_left(&self) -> Option<&Node> {
        self.left.as_deref()
    }

    pub fn get_right(&self) -> Option<&Node> {
        self.right.as_deref()
    }
}
