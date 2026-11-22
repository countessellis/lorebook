# Lorebook Language Definition

## Syntax

```ebnf
Script              ::= IncludeStatement* | (TomeHeader Statement*)
FileHeader          ::= ElementType PropertyBlock? ';'

Statement           ::= IncludeStatement | AssignmentStatement | LinkStatement
IncludeStatement    ::= 'include' StringLiteral ';'
AssignmentStatement ::= ('let' | 'has') Identifier '=' CreateExpression ';'
CreateExpression    ::= 'new' ElementType PropertyBlock?
LinkStatement       ::= 'add' Identifier 'to' Identifier ('in' Identifier)? ';'

ElementType         ::= 'Library' | 'Tome' | 'Story' | 'Setting' | 'Character' 
PropertyBlock       ::= '{' PropertyList? '}'
PropertyList        ::= Property (',' Property)* ','?
Property            ::= Identifier '=' LiteralValue
LiteralValue        ::= StringLiteral | NumberLiteral | BooleanLiteral

Identifier          ::= [a-zA-Z_][a-zA-Z0-9_]*
StringLiteral       ::= '"' ('\\"' | [^"])* '"'
NumberLiteral       ::= [0-9]+
BooleanLiteral      ::= 'true' | 'false'

## Rules

* Whitespace is insignificant except to separate tokens.
* Every `.lore` file begins with a `<ElementType> { ... };` header defining the root element of that file.
* `let` binds a local variable in the current file without adding it to the file's root element.
* `has` binds a local variable **and** immediately registers the element inside the file's root element (must obey valid containment rules).
* `include "<path>";` parses the target `.lore` file (resolved relative to the current file's directory) and directly adds its root element to the current file's root element.
* Elements created via `let` are bound to a local variable and automatically registered in the current file's root element.
* Valid containment relationships (both for `include` and `add`):
  * `Library`: top-level only; accepts `Tome`.
  * `Tome`: accepts `Story`, `Setting`, and `Character`.
  * `Story`: accepts `Setting` and `Character`.
  * `Setting`: accepts `Character`.
  * `Character`: leaf element (cannot contain child elements).
* The optional `in <Identifier>` clause scopes the target container inside a parent element (e.g., `add <Character> to <Setting> in <Story>;`).

## Examples

```text
// library.lore
Library;
include "book_of_lore.lore";

// book_of_lore.lore
Tome { name = "Book of Lore" };
include "city_of_brass.lore";

// city_of_brass.lore
Story { name = "City of Brass" };
include "aladdin.lore";

let cave_of_wonders = new Setting { name = "Cave of Wonders" };
add aladdin to cave_of_wonders;

// aladdin.lore
Character { name = "Aladdin" };

// Single-file Library example
Library;

has book_one = new Tome { name = "Book of Lore" };

let aladdin = new Character { name = "Aladdin" };
let cave_of_wonders = new Setting { name = "Cave of Wonders" };
let wonderous_lamp = new Story { name = "The Wonderous Lamp" };

add aladdin to wonderous_lamp;
add cave_of_wonders to wonderous_lamp;
add aladdin to cave_of_wonders in wonderous_lamp;
add wonderous_lamp to book_one;
```
