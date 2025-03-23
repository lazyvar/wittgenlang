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
"""

add #Number by {
  @x #Number
  @y #Number
  
  x + y
}

fourtyTwo is add (12, 30)
twenty is add (x: 1, y: 23)

log-happiness #bliss by {
  write "If you can't be happy now, then you'll never be happy :D"
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
    "Ignorant"
  }
}

calculate-understanding (power-level: 8999)
```

### Math operations

```wittgenlang

```
