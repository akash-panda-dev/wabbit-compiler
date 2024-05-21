## Big Ideas to think about

1. A map like ergonomics where you can map on operation to the program and it would
   go through the structure and modify the concerned parts.

```
map(fold_expression, program)
```

2. Is there a way to ensure the code is correct after each stage?

### Initial approach for organizing transforms and types

I like the idea of mapping a function to a program.

So the initial approach is that the program will have a function called transform which will accept a transform_function and then apply the transform to every expression and statement in the program.

the transform functions can then match one the type of node i.e Statement or Expression and then modify only the concerned elements.

## Problem with Davids approach

Approach: Have different transform functions which either take in a generic Statement or Instruction and then match over it and then modify the specific Statments or Instructions
https://github.com/dabeaz-course/compilers_2024_05/blob/dabeaz/wabbi/stmtcode.py

Problem: The tree traversal code traverses the tree and applies this transform to all instructions or statements. But this means that for every instruction certain matching will have to be done. And there will be a lot of useless matching.

## Probelm with DJ's approach
Approach: 
https://github.com/dabeaz-course/compilers_2024_05/blob/djmitche/wab/src/compiler/transform.rs

This is better than David's approach in that the transforms act on specific AST nodes rather than generic ones but it still tries to do pattern matching because it doesn't know how exactly to dispatch. So it also does redundant calculations. 

Is this the concept of dynamic dispatch?


So the best solution so far looks like it is Visitor pattern.


I don't think I can use formatter this way. For formatter lets use regular pattern matching and for others use Visitor.


## Goals I want to achieve with this expression problem

1. The Transformers don't change all the AST nodes, so I need a way to implement transforms only for the nodes I want to change and then use a default implementation for it.
2. (Minor) A way to add transforms easily
3. (Minor) A way to add AST nodes easily