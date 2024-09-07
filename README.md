# RustyC
A minimalist C compiler written in Rust

# TODO
- [ ] Name resolution during AST lowering to HIR - resolve each name to an ID
- [ ] Design & Implement MIR
- [ ] Intern all data structures (AST, HIR, MIR, ...)
- [ ] Design & Implement LIR
- [ ] Rename AST lowerer to HIR builder and use visitor pattern (same as with
  HIR to MIR lowering) for lowering
- [ ] Remove Rc from structures that don't need it anymore (relevant to
  structures for which the visitor pattern has been implemented)
- [ ] Pretty print compiler data structures (AST, HIR, MIR, LIR, ...) when a
  command line flag is specified (will be enabled for debug builds only)
