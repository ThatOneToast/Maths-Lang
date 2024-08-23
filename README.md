# Maths - A new way to do math 🧮

Maths is a simple expressive language for performing mathmatical operations
and calculations. 

## Features ⚡️

* Variable Assignment: Using the `let` keyword create variables that can store expressions or values.
* Context Parameters: Using `#[paramater1, paramater2]` at the top of your maths file, you can pass in parameters to your maths expression..
* Your Basic Math: Your standard symbols for math `*, +, ^` etc.
* Conditional Statements: Using `???` or `if` to create an if block that takes in a condition.
* Print out a variables value by using `;` followed by the variable name.
   
## Examples 📝

- Context Parameters:
```maths
#[height, width, length]
let result = height * width * length

;result
```
Pass the paramaters in by arguments in the cli

- Conditional Statements:
```maths
let cond1 = 60
let cond2 = 30

if cond1 > cond2 {
    let result = 1
} else {
    let result = 0
}

;result
```




## PS

* This is still in early stages. I am new to rust, and just trying to learn the way.
* This is a project I will find my self using, so if you have any additions, or comments, please GIMMEE.


