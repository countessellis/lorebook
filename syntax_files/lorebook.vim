" Vim syntax file
" Language: Lorebook
" Maintainer: Bethany Davis

if exists("b:current_syntax")
  finish
endif

" 1. Keywords & Types (from EBNF literals)
syntax keyword lorebookKeyword let new add to
syntax keyword lorebookType    library tome story setting character
syntax keyword lorebookBoolean true false

" 2. Comments (from Examples)
syntax match   lorebookComment "//.*$"

" 3. Literals (from EBNF regex rules)
syntax region  lorebookString  start=/"/ skip=/\\"/ end=/"/
syntax match   lorebookNumber  "\<\d\+\>"

" 4. Operators & Delimiters
syntax match   lorebookOperator "="
syntax match   lorebookDelimiter "[{};,]"

" 5. Map syntax groups to standard Vim highlight groups
highlight default link lorebookKeyword   Keyword
highlight default link lorebookType      Type
highlight default link lorebookBoolean   Boolean
highlight default link lorebookComment   Comment
highlight default link lorebookString    String
highlight default link lorebookNumber    Number
highlight default link lorebookOperator  Operator
highlight default link lorebookDelimiter Delimiter

let b:current_syntax = "lorebook"

