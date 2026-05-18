# HolyLang
![Unit Tests](https://github.com/chadsec1/HolyLang/actions/workflows/unit_tests_with_coverage.yml/badge.svg)
![CodeQL](https://github.com/chadsec1/HolyLang/actions/workflows/codeql.yml/badge.svg)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/73ffcbec8b7849a7a90f2003efe101af)](https://app.codacy.com/gh/chadsec1/HolyLang/dashboard)
[![Codacy Badge](https://app.codacy.com/project/badge/Coverage/73ffcbec8b7849a7a90f2003efe101af)](https://app.codacy.com/gh/chadsec1/HolyLang/coverage)


HolyLang programming language: Rust's compile-time safety, with Golang's readability.

In some specific aspects such as "logic-bug reducing features", HolyLang is superior to Rust.


# Work-in-progress
This only implements parser, and semantics.
It still lacks: structs, enums, unsafe blocks, char type, and actual binary generation phase.



# Example syntax
```
# This is a comment


# `const` is the constant declaration keyword
# Constant declaration syntax is:
# const CONST_NAME CONST_TYPE = EXPRESSION
#
const hi int32 = 123

# A constant can have complex expressions in it, as long as it consists of literals 
# .. and or other constants.
#
const hey int32 = hi + 1 # That would be 124


func main() {
    # You can also declare constants within a scope
    # Like, this constant scope makes it only available within `main` function
    const idk int32 = hi * hey

    # `own` is the variable declaration keyword
    # Declaration syntax is:
    # own VAR_NAME VAR_TYPE = EXPRESSION
    #

    own x int32 = 1

    # Assignment example
    x = 2


    # Variable overshadowing is not allowed.
    # own x int64 = 2 # This would've been a compile-time error if I uncomment it.

    # Array literals example
    own arr []int32 = [1, 2, 3, 4, 5]


    # Array access example (array accessing is always a copy)
    own first_element int32 = arr[0] # This is equal to 1st element in array `arr`, which is 1

    # Array slicing example (array slicing is always a copy)
    own arr_slice []int32 = arr[1:3] # this creates new array starting from `arr`s 2nd element up to 4th element


    # Nested arrays example
    own nested_arr [][]int32 = [ [1,2], [3,4], [5,6]]


    # Example of the move-or-copy safety model, where there is only one owner of a variable,
    # Holylang does not support references, borrowing, aliases, etc.
    # You either move a variable, or copy it.
    #

    own a int32 = 1
    own b int32 = x

    # This is invalid, it would not compile if I uncomment it.
    # a = 2

    own c int32 = copy(b)
    # This is valid, because `c` did not move `b`, it only copied it.
    b = 3



    # All basic primitive types have a default value
    # (0 for integers, 0.0 for floats, false for booleans, empty arrays for arrays, "" for string)
    #
    own h int32
    # h has value of 0


    # Function calls example
    own res int32 = add(1, 2)



    # This is multi declaration
    own n1 int32, n2 int32, n3 int32 = give_3_numbers()

    # You can also do multi assignment
    n1, n2, n3 = give_3_numbers()


    # Strings example
    own name string = "John"

    # Format takes one string argument, placeholders are directly in string
    # To escape a placeholder use {{}}
    own greeting_str string = format("Hello, {name}! How are you ?")



    own v int32 = 1

    # Variable locking prevents assigning to it.
    # You can still move or copy it though.
    lock v

    # v = 2 # If I uncomment this, it will be compiler error


    # You can unlock variables.
    unlock v

    # Now these work fine!
    v = 2


    # If statements example
    own one int32 = 1
    own two int32 = 2

    if two > one {
        one = 2
    }

    if (one >= two) and (two <= one) {
        two = 6
    } elif (two > one) or (one < two) {
        one = 6
    } else {
        one = two * 2
    }


    # While loops
    while true {
        if one >= 100 {
            break
        } else {
            one = one + 1
        }
    }



    # For loops

    own names []string = ["john", "jane", "jack", "jeffrey", "epstein"]
    for s in names {
        if s == "jack" {
            break
        }
    }


    for i in range(1, 10) {
        if i == 7 {
            continue
        }
    }



    # Infinite loops
    own num int32
    infinite {
        # When you use variables in binary expressions, they are copied automatically, you dont need copy().
        num = add(num + 1, num + 2)


        if num >= 1000 {
            break
        }
    }

}


# Function that adds 2 numbers together and returns result
func add(a int32, b int32) int32 {
    own result int32 = a + b
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
