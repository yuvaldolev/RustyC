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
}
