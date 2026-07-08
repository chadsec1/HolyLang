# HolyLang
![Unit Tests](https://github.com/HolyLang/HolyLang-Bootstrap/actions/workflows/unit_tests_with_coverage.yml/badge.svg)
![Clippy](https://github.com/HolyLang/HolyLang-Bootstrap/actions/workflows/clippy.yml/badge.svg)
![CodeQL](https://github.com/HolyLang/HolyLang-Bootstrap/actions/workflows/codeql.yml/badge.svg)
[![Coverage](https://sonarcloud.io/api/project_badges/measure?project=HolyLang_HolyLang-Bootstrap&metric=coverage)](https://sonarcloud.io/summary/new_code?id=HolyLang_HolyLang-Bootstrap)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=HolyLang_HolyLang-Bootstrap&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=HolyLang_HolyLang-Bootstrap)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=HolyLang_HolyLang-Bootstrap&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=HolyLang_HolyLang-Bootstrap)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=HolyLang_HolyLang-Bootstrap&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=HolyLang_HolyLang-Bootstrap)
[![Maintainability Rating](https://sonarcloud.io/api/project_badges/measure?project=HolyLang_HolyLang-Bootstrap&metric=sqale_rating)](https://sonarcloud.io/summary/new_code?id=HolyLang_HolyLang-Bootstrap)
[![Duplicated Lines (%)](https://sonarcloud.io/api/project_badges/measure?project=HolyLang_HolyLang-Bootstrap&metric=duplicated_lines_density)](https://sonarcloud.io/summary/new_code?id=HolyLang_HolyLang-Bootstrap)


**HolyLang** programming language: Rust's compile-time safety, with Golang's readability.

**HolyLang** is more secure than Rust, but not as performant as Rust. The purpose of HolyLang is memory and logic safety, not speed.

**IMPORTANT NOTE**: The name `HolyLang` is just a name, it does not mean it's holy, it is just a nod to Terry Davis's "HolyC". Language design and feature-wise, the only common thing with HolyLang and HolyC, is the fact we don't have a float32 type.
If you want real holiness (God's word), read the [Quran](https://quran.com/al-fatihah).

**NOTE**: **This project is just a hobby language** with design objective of being the safest and most readable systems programming language!
**There is no guarantees that comes with using this project.**


## HolyLang features:
- Readable syntax, and semantics.
- No borrow checker, and yet is still more secure than Rust. programs can be easily reasoned about due to the simple binary safey model of "move, or copy". 
- Arithmetic is always checked, including bitwise. Floating point arithmetic is also always checked (against inf/nan).
- No optional warnings, only errors. Unreachable code? Error. Unused variable? Error. Etc.
- Documentation is forced for functions, structs, and constants.
- No type inference, everything must be explicilty stated (this prevents common errors that stems especially from numeric types and assumptions.).
- No overshadowing allowed. Making codebases easier to audit, and reducing likelyhood of logic bugs.
- `lock` and `unlock` statements allow you to declare "zones" where variables behave as constants.


**HolyLang**'s bootstrap compiler transpiles down to pure Rust for a mathematical guarantee of safety: `"If Rust is safe, then HolyLang must also be at least as safe as Rust"`.

.. and a lot more! This is just the bootstrap compiler, the actual compiler will have even more security features, such as static stack analyzes that guarantee (at compile-time!) a program cannot overflow the stack, allowing for even stricter security than Rust, and even other formally verified languages like Ada SPARK, whom have weak stack overflow protection without an OS's help.


# Work-in-progress
This bootstrap compiler implements parser, semantic analysis and enforcement, and the transpiler.
It still lacks: structs and methods, enums, sin (unsafe) blocks.

# Compiling the bootstrap compiler.
**Note**: The latest commit in main branch is always the latest stable release.
1. Clone the repository:
   ```bash
   git clone https://github.com/chadsec1/HolyLang.git
   ```
2. Compile the compiler:
   ```bash
   cargo build --release
   ```

The compiler binary will be located in `target/release/holylang`. Feel free to move it wherever you like.

# Compiling a HolyLang program
```bash
./holylang HOLY_SOURCE_CODE_PATH.holy TARGET_BINARY_PATH
```
That will compile a HolyLang file, and produce a binary at `TARGET_BINARY_PATH`.


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

    # Variables declared with an explicit value are `locked` by default (aka immutable)
    # if you want to assign to them, you must unlock them first
    #
    unlock x

    # Then you can assign.
    x = 2

    # Almost all types have a default value:
    # 0 for integers, 0.0 for floats, false for booleans, 
    # empty arrays for arrays, "" for strings.
    # 
    # The only exception to this rule are constants and fixed arrays.
    #
    
    own h int32 # h has value of 0

    # Variables declared without explicit value are `unlocked` by default
    # you don't need to unlock them before assignment.
    #
    h = 2

    # Variable overshadowing is not allowed.
    # own x int64 = 2 # This would've been a compile-time error if I uncomment it.

    # Dynamic heap-allocated arrays example
    own arr []int32 = [1, 2, 3, 4, 5]

    # Fixed-size stack-allocated arrays example
    own fixed_arr [5]int32 = [1, 2, 3, 4, 5]

    # Nested dynamic arrays example
    own nested_arr [][]int32 = [[1,2], [3,4], [5,6]]


    # Nested fixed arrays example
    own nested_fixed [3][2]int32 = [[1,2], [3,4], [5,6]]


    # Array access example (array accessing is always a copy)
    own first_element int32 = arr[0] # This is equal to 1st element in array `arr`, which is 1

    # Array slicing example (array slicing is always a copy)
    own arr_slice []int32 = arr[1:3] # this creates new array starting from `arr`s 2nd element up to 4th element


    # Example of the move-or-copy safety model, where there is only one owner of a variable,
    # Holylang does not support references, borrowing, aliases, etc.
    # You either move a variable, or copy it.
    #

    own a int32 = 1
    own b int32 = a

    # This is invalid, it would not compile if I uncomment it.
    # a = 2

    own c int32 = copy(b)

    # This is valid, because `c` did not move `b`, it only copied it.
    #
    unlock b
    b = 3


    # Function calls example
    own res int32 = add(1, 2)


    # This is multi declaration
    own n1 int32, n2 int32, n3 int32 = give_3_numbers()


    unlock n1, n2, n3

    # You can also do multi assignment
    n1, n2, n3 = give_3_numbers()


    # Strings example
    own name string = "John"

    # Format takes one string argument, placeholders are directly in string
    # To escape a placeholder use {{}}
    own greeting_str string = format("Hello, {name}! How are you ?")



    own v int32

    # Variable locking prevents assigning to it.
    # You can still move or copy it though.
    lock v

    # v = 2 # If I uncomment this, it will be compiler error


    # You can unlock variables.
    unlock v

    # Now these work fine!
    v = 2


    # If statements example
    own one int32
    own two int32 


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
