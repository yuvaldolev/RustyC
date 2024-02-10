pub struct Function {
    body: Vec<Box<Node>>,
    locals: Vec<Variable>,
}
