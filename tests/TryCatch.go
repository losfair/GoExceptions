package main

import (
    "fmt"
    "errors"
)

func doSomething() {
    panic(errors.New("Test panic"))
}

func main() {
	try {
		doSomething()
	} catch(e error) {
		fmt.Println(e.Error())
	}
}
