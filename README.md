```
Module = "module" Id ";" { Decl } "end" "."
Decl = Proc ";"
Proc = "procedure" Id ["*"] ";" "end"
```
