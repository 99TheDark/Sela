#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,     //
    Assign,   // Assignment
    Arrow,    //
    Range,    //
    ShortOr,  // Short-Circuiting Or
    ShortAnd, // Short-Circuiting And
    Equal,    // Equality
    Relat,    // Relational
    EagerOr,  //
    EagerXor, //
    EagerAnd, //
    Shift,    // Bit Shifting
    Addive,   // Additive
    Mulive,   // Multiplicative
    Cast,     // Casting
    Unary,    //
    Bind,     // Binding
    Pair,     // Pairing
    Prop,     // Property
}
