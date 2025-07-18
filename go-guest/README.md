# gyu-dom
DOM made by WebAssembly component and Go lang.

- Documents
  - https://component-model.bytecodealliance.org/language-support/go.html

### init project
```
go mod init [module-path]
```

### update go mod dependency
```
go mod tidy
```

### Resolve wit import and build package -> .wasm
```
wkg wit build -d ../wit -o ../ohim:dom.wasm
```

### Generate bindings wasm <-> go from ohim WIT definition
```
go tool wit-bindgen-go generate --world ohim:dom/imports --out internal ../ohim:dom.wasm
```

### Build host component
```
tinygo build -target=wasip2 -o test.wasm --wit-package ../ohim:dom.wasm --wit-world ohim:dom/imports main.go
```

### Exam wasm component exports
```
wasm-tools component wit test.wasm
```
