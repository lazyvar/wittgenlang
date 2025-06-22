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
```

### Values: Constants & Variables

```wittgenlang
!doc """
  Values are immutable by default. To allow a value to be changed, use the keyword `forNow`.
  To change a variable, use the keyword `change`.
  Types of the value are specified after the name using `#`.
  All values need types specified.
  Kebab case is the one true path for identifiers.
"""
one #Number is 1
ten #Number is 10
forNow eleven #Number is 0

change eleven to ten + one

message #Text is "What in the world?"
interpolated #Text is "The answer is {one + ten}."
we-live-like-this #Decision is yes
```

### Control flow

```wittgenlang
!doc """
  `of` is the switch statement.
  `if`, `for`, and `while`, you already know and love.
  `unless` is the opposite of `if`.
  Equality comparison uses the `=` keyword.
  Inequality comparison uses the `!=` keyword.
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

! Equality and inequality
if temperature = 22 {
  write ("It's exactly 22 degrees.")
}

if temperature != 0 {
  write ("It's not freezing.")
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
  write ("Iteration {i}")
}

! While loops for conditional iteration
forNow counter #Number is 0
while counter < 5 {
  write ("Counter: {counter}")
  change counter to counter + 1
}

! Break and continue
for i in 1..10 {
  if i % 2 = 0 {
    continue  ! Skip even numbers
  }
  
  if i > 7 {
    break ! Exit loop if i > 7
  }
  
  write ("Odd number: {i}")
}

! Pattern matching in conditions
shape #Shape is Circle(5)
if shape = Circle(radius) {
  write ("This is a circle with radius {radius}")
} else if shape = Rectangle(width, height) {
  write ("This is a rectangle {width}x{height}")
}
```

### Functions

```wittgenlang
!doc """
  Functions are declared using `by`.
  The return type is specified after the name using `#`.
  `#Bliss` is used to signify functions that don't return meaningful values (similar to void/unit).
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
  
  if denominator = 0 {
    produce 0  ! Exit early with default value
  }
  
  numerator / denominator
}

validate-age #Result(Age) by {
  @age #Number
  
  if age < 0 {
    produce Error("Age cannot be negative")
  }
  
  if age > 150 {
    produce Error("Age is unrealistic")
  }
  
  Success(age)
}

! Function that returns nothing meaningful
print-info #Bliss by {
  @user #User
  
  write("Name: {user'name}")
  write("Age: {user'age}")
  
  ! No explicit return needed, returns bliss implicitly
}
```

### Lambda functions

```wittgenlang
!doc """
  Lambda functions (anonymous functions) provide a concise way to create function values without naming them.
  They use the -> arrow notation and can be passed as arguments to other functions or assigned to variables.
  Lambda type is specified using #(ParamTypes) -> #ReturnType syntax.
  For multi-line lambdas, wrap the body in curly braces and use the produce keyword for early returns.
  
  There are two syntax forms for lambda functions:
  1. Arrow syntax: (params) -> expression
  2. Block syntax: (params) -> { statements }
"""

! Simple single-expression lambda
double #(Number) -> #Number is (x) -> x * 2
times-two #Number is double (5)  ! 10

! Lambda with multiple parameters
add #(Number, Number) -> #Number is (x, y) -> x + y
sum #Number is add (3, 4)  ! 7

! Using lambdas with higher-order functions
numbers #List(Number) is [1, 2, 3, 4, 5]
import List
doubled-list #List(Number) is numbers'map ((n) -> n * 2)  ! [2, 4, 6, 8, 10]
sum-of-squares #Number is numbers'reduce ((acc, n) -> acc + n * n, 0)  ! 55

! Multi-line lambda with curly braces
validate #(Number) -> #Decision is (value) -> {
  if value < 0 {
    produce no  ! Early return for negative values
  }
  
  if value > 100 {
    produce no  ! Early return for values over 100
  }
  
  yes ! Default return for valid values
}

is-valid #Decision is validate (42)  ! yes
is-too-large #Decision is validate (150)  ! no

! Capturing variables from outer scope
factor #Number is 10
scaled-numbers #List(Number) is numbers'map ((n) -> n * factor)  ! [10, 20, 30, 40, 50]

! Immediately invoked lambda expression (IILE)
result #Number is ((x, y) -> x * y) (6, 7)  ! 42

! Lambda that transforms a string to its length
string-length #(Text) -> #Number is (str) -> str'length
length #Number is string-length ("hello")  ! 5

! Lambda with explicitly typed parameters
explicitly-typed #(Number, Number) -> #Number is (x #Number, y #Number) -> x + y
```

### Math operations

```wittgenlang
!doc """
  Standard math operations include addition (+), subtraction (-), multiplication (*), division (/), 
  modulo (%), exponentiation (^), and integer division (//). 
  Comparison operators are: >, <, >=, <=
  Equality operators: =, !=
  Logical operators: and, or, not
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

! Compound assignment operators
forNow x #Number is 10
change x to x + 5   ! x is now 15

! Comparison operators
greater #Decision is number-a > number-b     ! yes
less #Decision is number-a < number-b        ! no
greater-equal #Decision is number-a >= number-b  ! yes
less-equal #Decision is number-a <= number-b     ! no
equal #Decision is number-a = number-b      ! no
not-equal #Decision is number-a != number-b  ! yes

! Logical operators
logical-and #Decision is yes and no     ! no
logical-or #Decision is yes or no       ! yes
logical-not #Decision is not yes        ! no

! Operator precedence follows mathematical conventions
complex #Number is 2 + 3 * 4            ! 14, not 20
with-parens #Number is (2 + 3) * 4      ! 20
```

### Data structures

```wittgenlang
!doc """
  Wittgenlang supports various data structures such as Lists, Maps, Records, and Tuples.
  Lists and Maps are mutable by default, while Records and Tuples are immutable.
  Type functions use a module-like syntax and must be imported to be used.
  The notation `value'function` is shorthand for `Type'function(value)`.
  
  Access operations:
  - Lists and Tuples: Use [index] notation, zero-indexed
  - Maps: Use [key] notation
  - Records: Use dot notation or function style after importing
"""

! Lists
numbers #List(Number) is [1, 2, 3, 4, 5]
words #List(Text) is ["hello", "world"]
mixed #List(Any) is [1, "two", yes]

! Accessing list elements (zero-indexed)
first-number #Number is numbers [0]    ! 1
third-word #Text is words [2]          ! Causes runtime error (out of bounds)

! Lists are mutable
change numbers[0] to 10              ! [10, 2, 3, 4, 5]

! Common list operations
import List
length #Number is numbers'length      ! 5
append #List(Number) is numbers'append(6)  ! [10, 2, 3, 4, 5, 6]
slice #List(Number) is numbers'slice(1, 3)  ! [2, 3]

! Maps (key-value pairs)
ages #Map(Text, Number) is {
  "alice": 30,
  "bob": 25,
  "charlie": 35
}

alice-age #Number is ages["alice"]    ! 30

! Updating maps
change ages["dave"] to 40             ! Adds a new key-value pair
change ages["alice"] to 31            ! Updates existing value

! Records (named fields)
see #Person is #Record {
  name #Text
  age #Number
  is-active #Decision
}

alice #Person is Person {
  name: "Alice",
  age: 30,
  is-active: yes
}

! Accessing record fields
import Person
alice-name #Text is alice'name        ! "Alice" 
! Alternatively: alice-name #Text is Person'name(alice)

! Immutable update for records (creates a new record)
bob #Person is alice'with { name: "Bob", age: 25 }

! Tuples (fixed-size collections of different types)
coordinate #Tuple(Number, Number) is (10.5, 20.3)
name-and-age #Tuple(Text, Number) is ("Dave", 28)

x-coord #Number is coordinate[0]      ! 10.5
y-coord #Number is coordinate[1]      ! 20.3

! Destructuring tuples
(name, age) is name-and-age           ! Assigns "Dave" to name and 28 to age
```

### Error handling

```wittgenlang
!doc """
  Wittgenlang uses a Result-based approach for error handling, similar to Rust and other modern languages.
  The `#Result` type is a built-in variant type with `Success` and `Error` variants.
  Error propagation is done using the `try` keyword, and error handling uses pattern matching.
  There is also a `rescue` block for catching exceptions and unexpected runtime errors.
  
  For functions that might not return a value, use the `#Optional` type rather than returning nil.
"""

! Result type for explicit error handling
see #Result(T) is variant {
  Success(value #T)
  Error(message #Text)
}

! Function that may fail
divide #Result(Number) by {
  @numerator #Number
  @denominator #Number
  
  if denominator = 0 {
    produce Error("Division by zero")
  }
  
  Success(numerator / denominator)
}

! Using the try operator for error propagation
calculate-ratio #Result(Number) by {
  @a #Number
  @b #Number
  @c #Number
  
  first-result #Result(Number) is try divide(a, b)
  second-result #Result(Number) is try divide(first-result, c)
  
  Success(second-result)
}

! Pattern matching with Result
forNow result #Result(Number) is divide(10, 2)
of result {
  Success(value) -> write("Result: {value}")
  Error(message) -> write("Error: {message}")
}

! Error chaining with the | operator
calculate-complex #Result(Number) by {
  @x #Number
  @y #Number
  
  try divide(x, y) | (error) -> Error("Failed division: {error}")
}

! Using rescue blocks for exception handling
forNow value #Number is 0
rescue {
  value is dangerous-operation()
} catch error {
  write("Caught error: {error'message}")
  change value to 0
}

! Custom error types
see #ValidationError is #Variant {
  InvalidFormat(field #Text)
  OutOfRange(field #Text, min #Number, max #Number)
  Missing(field #Text)
}

validate-input #Result(Person) by {
  @name #Text
  @age #Number
  
  if name'length = 0 {
    produce Error(Missing("name"))
  }
  
  if age < 0 {
    produce Error(OutOfRange("age", 0, 150))
  }
  
  if age > 150 {
    produce Error(OutOfRange("age", 0, 150))
  }
  
  Success(Person {
    name: name,
    age: age,
    is-active: yes
  })
}
```

### Types

```wittgenlang
!doc """
  Wittgenlang has several built-in types and supports custom type definitions.
  Type annotations are required for all variables and function returns.
  Type definitions use the `see` keyword followed by the type name with # prefix.
  Only structs and variants are available for custom types, and their functions must
  be imported before use.
  
  The language has no concept of null or undefined. Instead, use:
  - `nil` as a value literal to represent empty/null state
  - `#T*` as syntactic sugar for `#Optional(T)` for values that might not exist
  - `#Bliss` as a return type for functions that don't return meaningful values
"""

! Built-in primitive types
number #Number is 42                ! Floating point numbers (default)
integer #Integer is 42              ! Integer
text #Text is "Hello"               ! Text strings
decision #Decision is yes           ! Boolean (yes/no)
empty #Bliss is bliss               ! Similar to void/unit, represents nothing

! Type aliases
see #Age is #Number                  ! Age is now an alias for Number
see #CountryCode is #Text            ! CountryCode is an alias for Text

! Union types
see #Result is #Number | #Text        ! Can be either a Number or Text
forNow maybe-value #Result is "error" ! Holds a Text now
change maybe-value to 42              ! Now holds a Number

! Custom types with variants (sum types)
see #Shape is #Variant {
  Circle(radius #Number)
  Rectangle(width #Number, height #Number)
  Triangle(base #Number, height #Number)
}

my-shape #Shape is Circle(5)
another-shape #Shape is Rectangle(10, 20)

! Type checking and pattern matching
if my-shape = Circle {
  ! Do something with Circle
}

if my-shape = Circle(radius) {
  ! radius now holds the value 5
  area #Number is Math'pi * radius * radius
}

! Optional types using the * syntax sugar (#T* is equivalent to #Optional(T))
see #Optional(T) is #Variant {
  Some(value #T)
  None
}

maybe-number #Number* is Some(42)     ! Using Optional syntax sugar
maybe-text #Text* is None             ! Optional Text that is None

! Working with optional values
process-optional #Bliss by {
  @maybe #Number*                     ! Using * syntax for optional Number
  
  if maybe = Some(value) {
    write("Got a value: {value}")
  } else {
    write("No value present")
  }
}

! Instead of returning nil, return None
find-user #User* by {                 ! Returns an optional User
  @id #Text
  
  if id = "admin" {
    produce Some(User { name: "Admin", role: "admin" })
  }
  
  None  ! No user found
}

! Recursive types
see #BinaryTree(T) is #Variant {
  Leaf
  Node(value #T, left #BinaryTree(T), right #BinaryTree(T))
}

! Creating a simple binary tree
tree #BinaryTree(Number) is Node(
  10,
  Node(5, Leaf, Leaf),
  Node(15, Leaf, Leaf)
)
```

### Nil handling

```wittgenlang
!doc """
  Wittgenlang uses the nil value to represent an empty or void state.
  The #T* syntax (sugar for #Optional(T)) is used for values that might or might not exist.
  
  Best practices:
  - Never use nil as a function parameter or return value (use optional types instead)
  - Use nil only for void expressions or initializing variables
  - Pattern match on optional values to safely access their contents
"""

! Nil as a placeholder for uninitialized variables
forNow user-input #Text is nil
write("Enter your name:")
change user-input to Console'read-line.

! Optional as a nullable container
see #User is record {
  name #Text
  email #Text
  phone #Text*  ! Phone number might not exist, using * syntax
}

! Creating users with or without phone numbers
user-with-phone #User is User {
  name: "Alice",
  email: "alice@example.com",
  phone: Some("555-1234")
}

user-without-phone #User is User {
  name: "Bob",
  email: "bob@example.com",
  phone: None
}

! Safely accessing potentially missing values
display-contact-info #Bliss by {
  @user #User
  
  write("Name: {user'name}")
  write("Email: {user'email}")
  
  ! Pattern matching on optional value
  of user'phone {
    Some(phone) -> write("Phone: {phone}")
    None -> write("No phone number provided")
  }
}

! Map operations on optional values
import Optional

format-phone #Text by {
  @user #User
  
  ! Returns default text if phone is None
  Optional'map-or-else(
    user'phone,
    () -> "No phone available",
    (phone) -> "Call at: {phone}"
  )
}

! Optional chaining equivalent
get-area-code #Text* by {             ! Returns optional text (area code)
  @user #User
  
  ! Example of safe transformation chain
  Optional'flat-map(
    user'phone,
    (phone) -> {
      if phone'length >= 3 {
        produce Some(phone[0..3])
      }
      None
    }
  )
}

! Multiple optional values in a function
combine-names #Text* by {
  @first-name #Text*
  @last-name #Text*
  
  ! Only combine if both are present
  if first-name = Some(first) and last-name = Some(last) {
    produce Some("{first} {last}")
  }
  
  None
}
```

### Custom type definitions

```wittgenlang
!doc """
  Custom types are defined using the `see` keyword followed by the type name with # prefix.
  The definition starts with the type name, followed by `is` and then the type definition.
  This allows for creating expressive, domain-specific types that improve code readability.
  
  There are several forms of type definitions:
  1. Type aliases: see #TypeName is #ExistingType
  2. Record types: see #TypeName is #Record { ... }
  3. Variant types: see #TypeName is #Variant { ... }
  4. Generic types: see #TypeName(T) is ...
  5. Module types: see #ModuleName is #Module { ... }
  6. Optional types: #TypeName* is sugar for #Optional(TypeName)
"""

! Simple custom type definition
see #AccountId is #Text                 ! AccountId is just a specialized Text type

! Record type definition (struct)
see #Person is #Record {
  name #Text
  age #Number
  email #Text
  address #Text*                        ! Optional address using * syntax
}

! Variant type definition (sum type / tagged union)
see #Result(T) is #Variant {
  Success(value #T)
  Error(message #Text)
}

! Parametric type definition (generic)
see #Pair(A, B) is #Record {
  first #A
  second #B
}

! Using custom types
create-account #AccountId by {
  @name #Text
  
  name & "-" & random-digits(6)  ! Combines the name with random digits
}

process-result #Bliss by {
  @result #Result(Any)
  
  ! Import Result module to access its functions
  import Result
  
  if result = Success(value) {
    write ("Success: {value}")
  } else if result = Error(message) {
    write ("Error: {message}")
  }
}

create-pair #Pair(Number, Text) by {
  @number #Number
  @text #Text
  
  Pair {
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
  Modules in Wittgenlang are defined using the `see #ModuleName is #Module` syntax.
  They provide namespaces for functions and can be nested.
  All code belongs to a module, and functions defined within a module
  can be called directly within that module.

  Types behave like modules for their functions. To access type functions, you must first
  import the type module. The syntax `value'function` is shorthand for `Type'function(value)`.
  
  Module naming conventions:
  - Module names must start with an uppercase letter
  - Multi-word modules use PascalCase
  - Submodules are separated by dots (e.g., IO.File)
  
  Access modifiers:
  - Public functions are visible from outside the module (default)
  - Private functions use the `priv` keyword and are only visible within the module
"""

! Defining a module
see #Math is #Module {
  ! Module constant
  pi #Number is 3.14159

  ! Public function (exported by default)
  add #Number by {
    @x #Number
    @y #Number
    
    x + y
  }
  
  ! Private function (only accessible within this module)
  priv square #Number by {
    @x #Number
    
    x * x
  }
  
  ! Function that uses a private function in the module
  square-sum #Number by {
    @x #Number
    @y #Number
    
    square(add(x, y))
  }
  
  ! Function that performs an action but returns nothing meaningful
  log-calculation #Bliss by {
    @operation #Text
    @value #Number

    write("{operation} result: {value}")
  }
  
  ! Function that returns an optional value
  find-root #Number* by {
    @x #Number
    
    if x >= 0 {
      produce Some(x'sqrt)
    }
    
    None ! No real square root for negative numbers
  }
}

! Nested modules
see #Geometry is #Module {
  see #Circle is #Module {
    area #Number by {
      @radius #Number
      
      Math'pi * radius * radius
    }
    
    circumference #Number by {
      @radius #Number
      
      2 * Math'pi * radius
    }
  }
  
  see #Rectangle is #Module {
    area #Number by {
      @width #Number
      @height #Number
      
      width * height
    }
  }
  
  ! Protected function (accessible from submodules)
 calculate-area #Number by {
    @shape #Shape
    
    if shape = Circle(radius) {
      Circle'area(radius)
    } else if shape = Rectangle(width, height) {
      Rectangle'area(width, height)
    } else {
      0
    }
  }
}

! Using modules
import Math
sum #Number is Math'add (5, 10)  ! 15

! Using optional return values from modules
forNow root #Number* is Math'find-root(-4)  ! None
of root {
  Some(value) -> write("Square root is {value}")
  None -> write("No real square root exists")
}

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

! Module visibility - expose only specific functions
see #StringUtils is #Module {
  ! Public functions
  capitalize #Text by {
    @input #Text
    
    if input'length = 0 {
      produce ""
    }
    
    input[0]'uppercase & input[1..]'lowercase
  }
  
  ! Private implementation details
  priv is-letter #Decision by {
    @char #Text
    
    char >= "a" and char <= "z" or char >= "A" and char <= "Z"
  }
  
  ! Function that returns an optional value when something might be missing
  find-first-letter #Text* by {
    @input #Text
    
    if input'length = 0 {
      produce None
    }
    
    for i in 0..input'length {
      if is-letter(input[i]) {
        produce Some(input[i])
      }
    }
    
    None
  }
}
```

### Standard library

```wittgenlang
!doc """
  Wittgenlang comes with a standard library that provides common functionality.
  These are some of the core modules and functions available.
  Type functions must be imported before use.
  
  Core standard library modules include:
  - Text: String manipulation
  - Math: Mathematical operations
  - List: Collection operations
  - Map: Key-value operations
  - IO: Input/output operations
  - Time: Date and time utilities
  - Result: Error handling utilities
  - Optional: Utilities for handling optional values
"""

! Text operations
greeting #Text is "Hello, World!"
import Text  ! Import Text module to use its functions
length #Number is greeting'length            ! 13, shorthand for Text'length(greeting)
uppercase #Text is greeting'uppercase         ! "HELLO, WORLD!"
contains-hello #Decision is greeting'contains ("Hello") ! yes

! Text concatenation with & operator
full-name #Text is "John" & " " & "Doe"      ! "John Doe"

! String interpolation with curly braces
age #Number is 42
message #Text is "The answer is {age}."      ! "The answer is 42."

! Math utilities
import Math
square-root #Number is Math'sqrt (16)        ! 4
sine-value #Number is Math'sin (Math'pi / 2) ! 1.0
cosine-value #Number is Math'cos (0)         ! 1.0
random-value #Number is Math'random          ! Random number between 0 and 1

! IO operations
import IO.File
content #Text is File'read ("example.txt")
File'write ("output.txt", "Hello from Wittgenlang!")

import IO.Console
Console'read-line #Text by {
  produce "User input"  ! Simulated for example
}

! Working with optional values using * syntax
import Optional

! Maybe operations with * syntax
forNow maybe-user #User* is find-user("admin")
if maybe-user = Some(user) {
  write("Found user: {user'name}")
} else {
  write("User not found")
}

! Using Optional utilities with * syntax
name #Text is Optional'get-or-else(
  Optional'map(maybe-user, (user) -> user'name),
  "Guest"
)

! Convenience functions for optional types
parse-number #Number* by {
  @text #Text
  
  if text'is-number {
    produce Some(text'to-number)
  }

  None
}

! Collection of optional values
accounts #List(Text*) is [
  Some("admin"),
  None,
  Some("guest")
]

! Filtering Some values
valid-accounts #List(Text) is accounts'filter-map ((username) -> username)

! Date and time
import Time
current-time #DateTime is Time'now.
formatted-date #Text is Time'format (current-time, "YYYY-MM-DD")
yesterday #DateTime is Time'add-days (current-time, -1)
is-before #Decision is Time'before (yesterday, current-time)  ! yes

! Collections utilities
numbers #List(Number) is [1, 2, 3, 4, 5]
import List  ! Import List module to use its functions
sum #Number is numbers'sum                   ! 15, shorthand for List'sum(numbers)
doubled #List(Number) is numbers'map ((n) -> n * 2) ! [2, 4, 6, 8, 10]
evens #List(Number) is numbers'filter ((n) -> n % 2 = 0) ! [2, 4]
first #Number is numbers'first               ! 1
last #Number is numbers'last                 ! 5
contains-three #Decision is numbers'contains (3)  ! yes

! Optional first element
first-element #Number* is List'first-optional(numbers)  ! Some(1)

! Standard library functions for Maps
ages #Map(Text, Number) is { "alice": 30, "bob": 25 }
import Map
keys #List(Text) is ages'keys                ! ["alice", "bob"]
values #List(Number) is ages'values          ! [30, 25]
has-key #Decision is ages'has-key ("alice")  ! yes
updated-ages #Map(Text, Number) is ages'set ("charlie", 35)  ! Adds a new entry
get-value #Number* is ages'get("dave")       ! Returns None for missing key
```
