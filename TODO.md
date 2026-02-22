# To do
- Eat the entire line if the compiler gets really confused
- Implement forced parens on operators
- Multi-line comments
- Add complexity to errors (contextual regions, like where borrows occur, the start and end of a function, etc)
- Better error escaping for incorrect tokens; e.g. `{ let a = }` should properly exit the block
- Dot operator