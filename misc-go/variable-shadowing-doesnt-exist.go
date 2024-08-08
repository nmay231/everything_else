package main

import "fmt"

func main() {
	x := 1

	// Doesn't work in go ):
	// x := "My type changed!"

	// Minor detail but you can only repeat a variable declaration
	// with := if there is a new variable being clared, y in this case
	x, y := 2, "testing"
	fmt.Printf("asdf %v %v\n", x, y)
}
