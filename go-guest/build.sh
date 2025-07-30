#! /usr/bin/env bash

wkg wit build -d ../wit -o ../ohim:dom.wasm
go tool wit-bindgen-go generate --world ohim:dom/imports-go --out internal ../ohim:dom.wasm
tinygo build -target=wasip2 -o test.wasm --wit-package ../ohim:dom.wasm --wit-world ohim:dom/imports-go main.go