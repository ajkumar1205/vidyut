
### The Grammar of the language is roughly given below


`ProgramFile` => `ImportDeclList` `DeclList` `MainFn` `DeclList`

`DeclList` => `Decl` `DeclList` | `null`

`Decl` => `StructDeclList` | `EnumDeclList` | `ImplDeclList` | `FnDeclList`

`StructDeclList` => `StructDecl` `StructDeclList` | `null`

`EnumDeclList` => `EnumDecl` `EnumDeclList` | `null`

`ImplDeclList` => `ImplDecl` `ImplDeclList` | `null`

`FuncDeclList` => `FuncDecl` `FuncDeclList` | `null`

`ImportDeclList` => `ImportDecl` `ImportDeclList` | `null`

`StructDecl` => `AccessDecl` struct `ID` { `StructTypeDeclList` }

`StructTypeDeclList` => `AccessDecl` `Type` `ID`, `StructTypeDeclList` | `null`

`AccessDecl` => pub | `null`

`EnumDecl` =>  `AccessDecl` enum `ID` { `EnumTypeDeclList` }

`EnumTypeDeclList` => `ID`, `EnumTypeDeclList` | `null`

`ImplDecl` => impl `ID` { `FnDeclList` }

`FnDeclList` => `FnDecl` `FnDeclList` | `null`

`FnDecl` => `Type` `ID` (`FnTypeDeclList`){ `CompoundDeclList` }

`FnTypeDeclList` => `FnTypeDecl`, `FnTypeDeclList` | `null`

`MainFn` => `AccessDecl` void main(){  `CompoundDeclList` }

`CompoundDeclList` => `VarConLoopDecl` `CompoundDeclList` | `null`

`VarConLoopDecl` => `VariableDeclList` | `VariableInitList` |  `ConditionalStm` | `LoopStm` | 

`VariableInitList` => `VariableInit` `VariableInitList` | `null`

`VariableDeclList` => `VariableDecl` `VariableDeclList` | `null`

`VariableDecl` => let `MutDecl` `ID`: `Type`;

`MutDecl` => mut | `null`

`VariableInit` => let `MutDecl` `ArrowAsign` `Expression`;

`ArrowAsign` => <- | `null`