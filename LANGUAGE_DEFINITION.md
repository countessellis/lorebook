# Lorebook Language Definition

## Syntax

```ebnf
Script              ::= IncludeStatement* | (TomeHeader Statement*)

IncludeStatement    ::= 'include' StringLiteral ';'
TomeHeader          ::= 'Tome' PropertyBlock ';'

Statement           ::= IncludeStatement | AssignmentStatement | LinkStatement
AssignmentStatement ::= 'let' Identifier '=' CreateExpression ';'
CreateExpression    ::= 'new' ElementType PropertyBlock?

LinkStatement       ::= 'add' Identifier 'to' Identifier ('in' Identifier)? ';'

ElementType         ::= 'Story' | 'Setting' | 'Character' 
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
* Every `.lore` tome file begins with a `Tome { ... };` header defining that file's `Tome`.
* All elements created with `let` in a tome file are automatically registered in that file's `Tome`.
* Valid `add` relationships:
  * `Story` -> `Tome` (implicit)
  * `Setting` -> `Tome` (implicit) or `Story`
  * `Character` -> `Tome` (implicit), `Setting`, or `Story`
* The optional `in <Identifier>` clause scopes the target container inside a parent element (e.g., `add <Character> to <Setting> in <Story>;`).

## Examples

```text
// library.lore
include "book_of_lore.lore";
include "second_book_of_lore.lore";

// book_of_lore.lore
Tome { name = "Book of Lore" };

let aladdin = new Character { name = "Aladdin" };
let cave_of_wonders = new Setting { name = "Cave of Wonders" };
let wonderous_lamp = new Story { name = "The Wonderous Lamp" };

add aladdin to wonderous_lamp;
add cave_of_wonders to wonderous_lamp;
add aladdin to cave_of_wonders in wonderous_lamp;
```
