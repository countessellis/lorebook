# Lorebook Language Definition

## Syntax

```ebnf
Script              ::= Statement*
Statement           ::= AssignmentStatement | LinkStatement

AssignmentStatement ::= 'let' Identifier '=' CreateExpression ';'
CreateExpression    ::= 'new' ElementType PropertyBlock?

LinkStatement       ::= 'add' ValidLink ';'
ValidLink           ::= 'tome'      Identifier 'to' 'library' Identifier
                      | 'story'     Identifier 'to' 'tome'    Identifier
                      | 'setting'   Identifier 'to' ('tome' | 'story') Identifier
                      | 'character' Identifier 'to' ('tome' | 'setting' | 'story') Identifier

PropertyBlock       ::= '{' PropertyList? '}'
PropertyList        ::= Property (',' Property)* ','?
Property            ::= Identifier '=' LiteralValue
LiteralValue        ::= StringLiteral | NumberLiteral | BooleanLiteral

ElementType         ::= 'library' | 'tome' | 'story' | 'setting' | 'character' 
Identifier          ::= [a-zA-Z_][a-zA-Z0-9_]*
StringLiteral       ::= '"' ('\\"' | [^"])* '"'
NumberLiteral       ::= [0-9]+
BooleanLiteral      ::= 'true' | 'false'

## Rules

* Whitespace is insignificant except to separate tokens.
* Identifiers in `add` statements must reference variables initialized with the matching `ElementType`.

## Examples

```text
// Creating elements:
let main_lib = new library { name = "Grand Archive" };
let lore_tome = new tome { name = "Second Book of Lore" };
let hero = new character { name = "Aladdin" };

// Adding elements to other elements:
add tome lore_tome to library main_lib;
add character hero to tome lore_tome;
```
