# Maths - A new way to do math 🧮

Maths is a simple expressive language for performing mathmatical operations
and calculations. 

## Features ⚡️

* Variable Assignment: Using the `let` keyword create variables that can store expressions or values.
* Context Parameters: Using `#[paramater1, paramater2]` at the top of your maths file, you can pass in parameters to your maths expression..
* Functions: Using '@' followed by a pattern name, and the list of required parameters, to return a new value based on a set of operations, and rules.
* Your Basic Math: Your standard symbols for math `*, +, ^` etc.
* Conditional Statements: Using `???` or `if` to create an if block that takes in a condition.
* Print out a variables value by using `;` followed by the variable name.
   
Notes:

To create a pattern for functions, create a new maths file, use context paramaters to pass in the required parameters, 
do your calculations, the final variable checked for final result is `result`.
## Examples 📝

- Context Parameters: (Volume)
```maths
#[height, width, length]
let result = height * width * length

;result
```
Pass the paramaters in by arguments in the cli or using a function

- Functions: 
```maths
let height = 10
let width = 10
let length = 10

let result = @Volume(height,width,length)

;result
```

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


