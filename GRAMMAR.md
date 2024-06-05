
### The Grammar of the language is roughly given below


`ProgramFile` => `(ImportDecl)*` `(Decl)*` `MainFn` `(Decl)*`

`Decl` => `(StructDecl)*` | `(EnumDecl)*` | `(ImplDecl)*` | `(FnDecl)*`

`ImportDecl` => |Not yet Decided|

`StructDecl` => `AccessDecl` struct `ID` { `(StructTypeDecl)*` }

`StructTypeDecl` => `(AccessDecl Type ID,)*`

`AccessDecl` => pub | `null`

`EnumDecl` =>  `AccessDecl` enum `ID` { `(ID,)*` }

`ImplDecl` => impl `ID` { `(FnDecl)*` }

`FnDecl` => `Type` `ID` (`(FnTypeDecl),*`){`(CompoundDecl)*`}

`MainFn` => void main(){ | `(CompoundDecl)*` | }

`CompoundDecl` => `(VariableDecl)*` |  `ConditionalStm` | `LoopStm` 


