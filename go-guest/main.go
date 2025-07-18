package main

import imports "go-guest/internal/ohim/dom/imports"

func init() {
	imports.Exports.Test = func() string {
		return "Hello from Go!"
	}
}

// main is required for the `wasi` target, even if it isn't used.
func main() {}
