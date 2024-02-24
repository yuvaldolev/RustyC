use rustyc_span::Span;

use crate::item_kind::ItemKind;

pub struct Item {
    kind: ItemKind,
    span: Span,
}

impl Item {
    pub fn new(kind: ItemKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn get_kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }
}
