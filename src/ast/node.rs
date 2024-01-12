use crate::span::Span;

use super::node_kind::NodeKind;

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
}
