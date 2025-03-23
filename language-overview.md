# Wittgenlang overview

### Comments

```wittgenlang
! Exclamation point starts a single line comment
! This is a back-to-back single line comment

!doc """
This is a multiline comment.
Write whatever you want here.
It will only be interpreted by your brain.
"""
````

### Values: Constants & Variables

```wittgenlang
!doc """
Values are immutable by default. To allow a value to be changed, use the keyword `forNow`.
To change a variable, use the keyword `change`.
Types of the value are specified after the name using `#`.
All values need types specified.
Kebab case is the one true path.
"""
one #Number is 1
ten #Number is 10
forNow eleven #Number is 0

change eleven to ten + one

message #Text is "What in the world?"
we-live-like-this #Decision is yes
```

### Control flow

```wittgenlang
!doc """
`of` is the swtich statement.
`if`, `for`, and `while`, you already know and love.
`unless` is the opposite of `if`.
"""
! Conditional statements with if
temperature #Number is 22
if temperature > 30 {
  write ("It's hot outside!")
} else if temperature > 20 {
  write ("It's a pleasant day.")
} else {
  write ("It's cool today.")
}

! Unless is the opposite of if
unless temperature < 0 {
  write ("At least it's not freezing.")
}

! Switch statement using 'of'
day #Text is "Monday"
of day {
  "Monday" -> write ("Start of work week")
  "Friday" -> write ("Almost the weekend!")
  "Saturday", "Sunday" -> write ("Weekend!")
  otherwise -> write ("Just another day")
}

! For loops for iteration
for number in [1, 2, 3, 4, 5] {
  write (number * 2)
}

! Looping with range
for i in 1..5 {
  write ("Iteration " & i)
}

! While loops for conditional iteration
forNow counter #Number is 0
while counter < 5 {
  write ("Counter: " & counter)
  change counter to counter + 1
}

! Break and continue
for i in 1..10 {
  if i % 2 is 0 {
    continue  ! Skip even numbers
  }
  
  if i > 7 {
    break  ! Exit loop if i > 7
  }
  
  write ("Odd number: " & i)
}

! Ternary-like conditional expressions
is-adult #Decision is age >= 18 ? yes : no

! Pattern matching in conditions
shape #Shape is Circle(5)
if shape is Circle(radius) {
  write ("This is a circle with radius " & radius)
} else if shape is Rectangle(width, height) {
  write ("This is a rectangle " & width & "x" & height)
}
```

### Functions

```wittgenlang
!doc """
Functions are declared using `by`.
The return type is specified after the name using `#`.
`#Bliss` is the better version of `void` or `unit` and is used to signify functions that return nothing.
Parameters to the function are declared in the function body using the `@` keyword.
There is always a new line after the parameter declarations unless there are no params.
There must always a new line before the last statment of the function.
Functions are called Kotlin style. Can be listed out or hash-style using colon.
There must always be a space after the name of the function when calling it.
Functions with no params can be called using `.`
Functions implicitly return their last expression, but can use `produce` to exit early with a value.
"""

add #Number by {
  @x #Number
  @y #Number
  
  x + y
}

fourtyTwo is add (12, 30)
twenty is add (x: 1, y: 23)

log-happiness #Bliss by {
  write ("If you can't be happy now, then you'll never be happy :D")
}

log-happiness.

calculate-understanding #Text by {
  !doc """
    Calculates total understanding, based upon power level.
  """
  @power-level #Number

  if power-level > 9000 {
    "Supreme knowledge"
  } else {
    "Ignorance"
  }
}

calculate-understanding (power-level: 8999)

! Using produce for early exit from a function
safe-divide #Number by {
  @numerator #Number
  @denominator #Number
  
  if denominator is 0 {
    produce 0  ! Exit early with default value
  }
  
  numerator / denominator
}

validate-age #Result by {
  @age #Number
  
  if age < 0 {
    produce Error("Age cannot be negative")
  }
  
  if age > 150 {
    produce Error("Age is unrealistic")
  }
  
  Success(age)
}
```

### Math operations

```wittgenlang
!doc """
Standard math operations include addition (+), subtraction (-), multiplication (*), division (/), 
modulo (%), exponentiation (^), and integer division (//). 
"""

number-a #Number is 5
number-b #Number is 3

sum #Number is number-a + number-b        ! addition: 8
difference #Number is number-a - number-b  ! subtraction: 2
product #Number is number-a * number-b    ! multiplication: 15
quotient #Number is number-a / number-b   ! division: 1.6666...
remainder #Number is number-a % number-b  ! modulo: 2
power #Number is number-a ^ number-b      ! exponentiation: 125
int-div #Number is number-a // number-b   ! integer division: 1

forNow counter #Number is 0
```

### Data structures

```wittgenlang
!doc """
Wittgenlang supports various data structures such as Lists, Maps, Records, and Tuples.
Lists and Maps are mutable by default, while Records and Tuples are immutable.
Type functions use a module-like syntax and must be imported to be used.
The notation `value'function` is shorthand for `Type'function(value)`.
"""

! Lists
numbers #List(Number) is [1, 2, 3, 4, 5]
words #List(Text) is ["hello", "world"]
mixed #List(Any) is [1, "two", yes]

! Accessing list elements (zero-indexed)
first-number #Number is numbers [0]    ! 1
third-word #Text is words [2]          ! Causes runtime error (out of bounds)

! Maps (key-value pairs)
ages #Map(Text, Number) is {
  "alice": 30,
  "bob": 25,
  "charlie": 35
}

alice-age #Number is ages ["alice"]    ! 30

! Records (named fields)
Person is record {
  name #Text
  age #Number
  is-active #Decision
}

alice #Person is Person {
  name: "Alice",
  age: 30,
  is-active: yes
}

! Type functions use module-like syntax
! alice'name is shorthand for Person'name(alice)
import Person
alice-name #Text is alice'name        ! "Alice" 
! Alternatively: alice-name #Text is Person'name(alice)

! Tuples (fixed-size collections of different types)
coordinate #Tuple(Number, Number) is (10.5, 20.3)
name-and-age #Tuple(Text, Number) is ("Dave", 28)

x-coord #Number is coordinate [0]      ! 10.5
```

### Error handling

TODO

### Types

```wittgenlang
!doc """
Wittgenlang has several built-in types and supports custom type definitions.
Type annotations are required for all variables and function returns.
Only structs and modules are available for types, and their functions must
be imported before use.
"""

! Built-in primitive types
number #Number is 42                ! Floating point numbers (default)
integer #Integer is 42               ! Integer
text #Text is "Hello"             ! Text strings
decision #Decision is yes             ! Boolean (yes/no)
nothing #Nothing is nothing          ! Similar to null/nil/None

! Type aliases
see #Age is #Number                  ! Age is now an alias for Number
see #CountryCode is #Text            ! CountryCode is an alias for Text

! Union types
see #Result is #Number | #Text        ! Can be either a Number or Text
maybe-value #Result is "error" ! Holds a Text now
change maybe-value to 42       ! Now holds a Number

! Custom types with variants (sum types)
see #Shape is variant {
  Circle(radius #Number)
  Rectangle(width #Number, height #Number)
  Triangle(base #Number, height #Number)
}

my-shape #Shape is Circle(5)
another-shape #Shape is Rectangle(10, 20)

! Type checking
if my-shape is Circle {
  ! Do something with Circle
}
```

### Custom type definitions

```wittgenlang
!doc """
Custom types are defined using the `#` prefix. The definition starts with the type name,
followed by `is` and then the type definition. This allows for creating expressive, domain-specific
types that improve code readability and maintainability.
"""

! Simple custom type definition
see #AccountId is #Text                 ! AccountId is just a specialized Text type

! Record type definition
see #Person is #{
  name #Text
  age #Number
  email #Text
}

! Variant type definition (sum type)
see #Result is #{
  Success(value #Any)
  Error(message #Text)
}

! Parametric type definition
see #Pair(A, B) is #{
  first #A
  second #B
}

! Using custom types
create-account #AccountId by {
  @name #Text
  
  name & "-" & random-digits(6)  ! Combines the name with random digits
}

process-result #Nothing by {
  @result #Result
  
  ! Import Result module to access its functions
  import Result
  
  if result is Success {
    ! result'value is shorthand for Result'value(result)
    value #Any is result'value
    write ("Success: " & value)
  } else {
    ! result'message is shorthand for Result'message(result)
    error-msg #Text is result'message
    write ("Error: " & error-msg)
  }
}

create-pair #Pair(Number, Text) by {
  @number #Number
  @text #Text
  
  #Pair(Number, Text) {
    first: number,
    second: text
  }
}

! Example usage
account-id #AccountId is create-account ("alice")  ! Returns "alice-123456"
number-text-pair #Pair(Number, Text) is create-pair (42, "answer")
```

### Modules and imports

```wittgenlang
!doc """
Modules in Wittgenlang are inspired by Elixir. They provide namespaces for functions
and can be nested. All code belongs to a module, and functions defined within a module
can be called directly within that module.

Types behave like modules for their functions. To access type functions, you must first
import the type module. The syntax `value'function` is shorthand for `Type'function(value)`.
"""

! Defining a module
module Math {
  ! Module constant
  pi #Number is 3.14159

  ! Public function (exported by default)
  add #Number by {
    @x #Number
    @y #Number
    
    x + y
  }
  
  ! Private function
  square #Number by {
    @x #Number
    
    x * x
  }
  
  ! Function that uses another function in the module
  square-sum #Number by {
    @x #Number
    @y #Number
    
    _square(add(x, y))
  }
}

! Nested modules
module Geometry {
  module Circle {
    area #Number by {
      @radius #Number
      
      Math'pi * radius * radius
    }
    
    circumference #Number by {
      @radius #Number
      
      2 * Math'pi * radius
    }
  }
  
  module Rectangle {
    area #Number by {
      @width #Number
      @height #Number
      
      width * height
    }
  }
}

! Using modules
import Math
sum #Number is Math'add (5, 10)  ! 15

! Import specific functions
import { add } from Math
another-sum #Number is add (7, 3)  ! 10

! Import nested modules
import Geometry.Circle
circle-area #Number is Circle'area (5)  ! 78.53975

! Use alias for imported modules
import Math as M
import Geometry.Rectangle as Rect
result #Number is M'add (Rect'area(3, 4), 5)  ! 17
```

### Standard library

```wittgenlang
!doc """
Wittgenlang comes with a standard library that provides common functionality.
These are some of the core modules and functions available.
Type functions must be imported before use.
"""

! Text operations
greeting #Text is "Hello, World!"
import Text  ! Import Text module to use its functions
length #Number is greeting'length            ! 13, shorthand for Text'length(greeting)
uppercase #Text is greeting'to-upper         ! "HELLO, WORLD!"
contains-hello #Decision is greeting'contains ("Hello") ! yes

! Math utilities
import Math
square-root #Number is Math'sqrt (16)                    ! 4
sine-value #Number is Math'sin (3.14159 / 2)      ! 1.0
cosine-value #Number is Math'cos (0)              ! 1.0

! IO operations
import IO.File
content #Text is File'read ("example.txt")
File'write ("output.txt", "Hello from Wittgenlang!")

! Date and time
import Time
current-time #DateTime is Time'now.
formatted-date #Text is Time'format (current-time, "YYYY-MM-DD")

! Collections utilities
numbers #List(Number) is [1, 2, 3, 4, 5]
import List  ! Import List module to use its functions
sum #Number is numbers'sum                   ! 15, shorthand for List'sum(numbers)
doubled #List(Number) is numbers'map (number => number * 2) ! [2, 4, 6, 8, 10]
evens #List(Number) is numbers'filter (number => number % 2 is 0) ! [2, 4]
```
