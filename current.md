# Current Decisions
All keywords, operators, and types currently decided to exist in the Sela programming language. These must currently be used somewhere in example code even if the feature's use is highly likely to be adapted, unless its use is guaranteed (like the modulus operator). Decisions may—and often will—be overturned on this document. Not every potential feature used will be documented here as currently definite.

## Keywords
|Name|Keyword|Name|Keyword|Name|Keyword|
|-|-|-|-|-|-|
|Let|`let`|Constant|`const`|Mutable|`mut`|
|Type|`type`|Enumerable|`enum`|Class|`class`|
|Idea|`idea`|Function|`func`|Module|`mod`|
|If|`if`|Else|`else`|Loop|`loop`|
|While|`while`|For|`for`|Match|`match`|
|Break|`break`|Continue|`continue`|Return|`return`|
|Self Value|`self`|Self Type|`Self`|Macro|`macro`|
|As|`as`|True|`true`|False|`false`|
|Charm|`charm`|And|`and`|Or|`or`|
|Use|`use`|

## Operators
|Name|Operator|Name|Operator|
|-|:-:|-|:-:|
|Add|`+`|Subtract/Negate|`-`|
|Multiply/Dereference|`*`|Divide|`/`|
|Modulus|`%`|Not|`!`|
|Bitwise And/Reference|`&`|Bitwise Or|`\|`|
|Bitwise Xor|`^`|Bitwise Right Shift|`>>`|
|Bitwise Left Shift|`<<`|Assignment|`=`|
|Equal To|`==`|Not Equal To|`!=`|
|Less Than|`<`|Greater Than|`>`|
|Less Than Or Equal To|`<=`|Greater Than or Equal To|`>=`|
|Inclusive Range|`..=`|Exclusive Range|`..<`|
|Full Range|`..`|Member Access|`.`|
|Interpolation|`$()`|Macro Call|`@`|

## Types
|Name|Type|Name|Type|Name|Type|
|-|-|-|-|-|-|
|Integer|`Int`|Unsigned Integer|`UInt`|Boolean|`Bool`|
|String|`String`|Character|`Char`|Floating-Point Number|`Float`|
|Array|`Array`|List|`List`|Map|`Map`|
|Option|`Option`|Result|`Result`|Box|`Box`|