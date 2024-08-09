#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    Negate,
    AddressOf,
    Dereference,
}
