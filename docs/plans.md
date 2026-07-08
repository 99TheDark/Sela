# High-Stage Plans and Ideas
These are ideas that are in the late stages of development. They may be reworked but are likely, as a core, to remain in the language.

## Operator Disambiguity
All binary operators (this does not _currently_ include `as`, `.`, `..=`, etc) are split into three domains:
- Standard (`+`, `/`, `%`, `<<`, `^`, `-`)
- Comparative (`>=`, `<`, `!=`, `is`, `==`)
- Logical (`and`, `or`)

The precedence between domains is always set as Standard > Comparative > Logical. However, _within_ all domains, operator precedence does not exist. It is not left-to-right; you must disambiguate with parentheses. Again, this is only for operator use _within the same domain_, not between domains. There is one exception, however: `+`, `-`, `*`, and `/` have set precedences, so when not paired with other operators of the same domain but only themselves, they do not require parentheses. For example, you write `x == 5 and y + 1 > 3` and it works fine. But `x << 1 + 2` fails, as you must write `x << (1 + 2)`. However, in `x ^ 3 - 2 * 4`, you can write this as `x ^ (3 - 2 * 4)` rather than `x ^ (3 - (2 * 4))`. The final exception is when using the same operator multiple times, like `x >> y >> z`; since all binary operators have left-to-right precedence (no `**`, and assignment is not an operator), it is clear what order the expression is running.

## Subsets
An enum can be subset via `.()`. For example, `read_file()` may return `Result[String, FileError.(Read, Permission)]` and no other `FileError` needs to be matched. Similarly, you can have granular/partial borrows via something like `&mut x.(a, b)`, `&x.(b, mut c)`, `&mut x.d`. This allows for greater flexibility. This is similarly used for multiselection in something like `use charm.ship.(Ship, Direction)`.

## Implicit Coercion
- Widening (`UInt32` -> `UInt64`, `Int16` -> `UInt32`, `Int32` -> `Float64`)
- String interpolation (`"Hello, I am $(age) years old."`)