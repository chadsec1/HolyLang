# HolyLang
![Unit Tests](https://github.com/chadsec1/HolyLang/actions/workflows/unit_tests.yml/badge.svg)

HolyLang programming language: A mix between Go's readability, C's simplicity, and Rust's compile-time safety.


# Work-in-progress
This only implements parser, and semantics.
It still lacks: if conditions, for loops, infinte loops, and unsafe blocks.



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
    own x = int32[1, 2, 3, 4, 55]


    # Nested arrays example
    own x int32[][] = int32[][int32[1,2], int32[3,4], int32[5,6]]


    # Example of move-or-copy safety model, only one owner of a variable exists. no aliasing, no references, no borrowing.

    own x = 1
    own y = x

    # This is invalid, it would not compile if I uncomment it.
    # x = 2

    own z = copy(y)
    # This is valid, because z did not move y, it only copied it.
    y = 3



    # 1 and 2 are integer literals and evaluated as expressions to determine their type, with infer hint of the function arguments list
    own x = add(1, 2)



    own x, y, z = give_3_numbers()


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
