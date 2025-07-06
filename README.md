```
Module = "MODULE" Id ";" { Decl } "END" "."
Decl = Proc ";"
Proc = "PROCEDURE" Id ["*"] [":" Id] ";" ["BEGIN" StmtSeq] "END"
StmtSeq = Stmt { ";" Stmt }
Stmt = "RETURN" [ Expr ]
Expr = Number
Number = Integer
Integer = Digit { Digit }
Id = Letter { Letter | Digit }
Letter = "a".."z" | "A".."Z"
Digit = "0".."9"
```
