# Lorebook Language Definition

## Syntax

```ebnf
Script              ::= Statement*
Statement           ::= AssignmentStatement | LinkStatement

AssignmentStatement ::= 'let' Identifier '=' CreateExpression ';'
CreateExpression    ::= 'new' ElementType PropertyBlock?

LinkStatement       ::= 'add' ValidLink ';'
ValidLink           ::= 'Tome'      Identifier 'to' 'Library' Identifier
                      | 'Story'     Identifier 'to' 'Tome'    Identifier
                      | 'Setting'   Identifier 'to' ('Tome' | 'Story') Identifier
                      | 'Character' Identifier 'to' ('Tome' | 'Setting' | 'Story') Identifier

PropertyBlock       ::= '{' PropertyList? '}'
PropertyList        ::= Property (',' Property)* ','?
Property            ::= Identifier '=' LiteralValue
LiteralValue        ::= StringLiteral | NumberLiteral | BooleanLiteral

ElementType         ::= 'Library' | 'Tome' | 'Story' | 'Setting' | 'Character' 
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
let main_lib = new Library { name = "Grand Archive" };
let lore_tome = new Tome { name = "Second Book of Lore" };
let hero = new Character { name = "Aladdin" };

// Adding elements to other elements:
add Tome lore_tome to Library main_lib;
add Character hero to Tome lore_tome;
```
