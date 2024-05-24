# PAF-lang
Partially applied functions

The idea:
- Everything is a function
    - IDK, maybe not
    - Lets go with "everything is an expression" first
    - Some kind of operator overloading
        - Maybe everything is translated to a function call on objects instead, kinda like C# does + -> add function
- Every function can be partially applied by default
    - Should support named parameters

###### Crazy additional ideas for a weird interpreter
- Functions can define the syntax for calling them
    - Basically lisp, make up syntax as you go
    - I have absolutely no idea how i would implement this
        - Is it even possible without building a new parser after analysing all functions?

