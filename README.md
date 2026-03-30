# HolyLang
![Unit Tests](https://github.com/chadsec1/HolyLang/actions/workflows/unit_tests_with_coverage.yml/badge.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/73ffcbec8b7849a7a90f2003efe101af)](https://app.codacy.com/gh/chadsec1/HolyLang/dashboard)
[![Codacy Badge](https://app.codacy.com/project/badge/Coverage/73ffcbec8b7849a7a90f2003efe101af)](https://app.codacy.com/gh/chadsec1/HolyLang/coverage)


HolyLang programming language: A mix between Go's readability, C's simplicity, and Rust's compile-time safety.


# Work-in-progress
This only implements parser, and semantics.
It still lacks: unsafe blocks, and actual binary generation phase.



# Example syntax
```
# This is a comment

func main() {
    # 1 is evaluated as an expression, and given appropriate type depending on its literal value
    # since x has no explicit value, the type of the right hand literal become the type of x
    own x = 1


    # Shadowing a declared variable is allowed
    own x = 2

    # 2 is evaluated as an expression, with type hint of int32
    # if 2 evaulated type does not match type hint, the compiler will error.
    own x int32 = 2


    # Arrays are supported too
    own x int32[] = int32[1, 2, 3, 4, 5]

    # They can be type inferred as well
    own x = int32[1, 2, 3, 4, 5]


    # Array access example (array accessing is always a copy)
    own y = x[0] # This is equal to 1st element in array x, which is 1

    # Array slicing example (array slicing is always a copy)
    own y = x[1:3] # this creates new array starting from x's 2nd element up to 4th element


    # Nested arrays example
    own x int32[][] = int32[][int32[1,2], int32[3,4], int32[5,6]]


    # Example of move-or-copy safety model, only one owner of a variable exists,
    # no aliasing, no references, no borrowing.

    own x = 1
    own y = x

    # This is invalid, it would not compile if I uncomment it.
    # x = 2

    own z = copy(y)
    # This is valid, because z did not move y, it only copied it.
    y = 3



    # All basic primitive types have a default value 
    # (0 for integers, 0.0 for floats, false for booleans, empty arrays for arrays, "" for string)
    #
    own x int32
    # x has value of 0


    # 1 and 2 are integer literals and evaluated as expressions to determine their 
    # type, with infer hint of the function arguments list
    own x = add(1, 2)



    # This is multi declaration
    own x, y, z = give_3_numbers()

    # You can also do multi assignment
    x, y, z = give_3_numbers()


    # Strings example
    own name = "John"

    # Format takes one string argument, placeholders are directly in string
    # To escape a placeholder use {{}}
    own greeting_str = format("Hello, {name}! How are you ?")



    own x = 1

    # Variable locking prevents overshadowing it, and assigning to it.
    # You can still move or copy it though.
    lock x

    # x = 2 # If I uncomment this, it will be compiler error
    # own x = 3 # Same thing here.


    # You can unlock variables.
    unlock x

    # Now these work fine! 
    x = 2
    own x = 3


    # If statements example
    own x = 1
    own y = 2

    if y > x {
        x = 2

        # Shadowing variables declared outside scope of if statement is an error
        # this wont compile if i uncomment this
        # own x = 3
    }


    # While loops
    while y > x {
        if x == 3 {
            break
        } else {
            x = x + 1
        }
    }



    # For loops

    own x = string["john", "jane", "jeffrey", "epstein"]
    for s in x {
        if s == "epstein" {
            break
        }
    }


    for i in range(1, 10) {
        if i == 7 {
            continue
        }
    }



    # Infinite loops
    own x int32
    infinite {
        x = add(x + 1, x + 2)


        if x >= 1000 {
            break
        }
    }

}


# Function that adds 2 numbers together and returns result
func add(a int32, b int32) int32 {
    own result = a + b
    return result
}


# Function that returns 3 numbers
func give_3_numbers() (int32, int32, int32) {
    own a int32 = 1
    own b int32 = 2
    own c int32 = 3

    return a, b, c
}

```
