# GoExceptions

This is a tool for transpiling Go code with try/catch statements into native Go source.

Note that GoExceptions is still in initial development and may include a lot of bugs. Don't use it in production for now.

An example:

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

will be translated into:

    package main

    import (
        "errors"
        "fmt"
    )

    func doSomething() {
        panic(errors.New("Test panic"))
    }

    func main() {

        func() {
            defer func() {
                if err := recover(); err != nil {
                    switch err.(type) {
                    case error:
                        func(e error) {

                            fmt.Println(e.Error())

                        }(err.(error))
                    default:
                        panic(err)
                    }
                }
            }()

            doSomething()

        }()

    }


## Build

You need `rustc` and `cargo` installed to build this tool:

    cd transpiler
    cargo build --release

The binary will be located at `transpiler/target/release/transpiler`.

## Contribute

Issues and pull requests are all welcomed.
