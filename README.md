# Introducing Recolon

Recolon is an experimental programming language project developed in Rust. It currently supports fundamental operations such as calculations, conditional statements, variable declarations, and logging. This project represents my take on the simplest approach to coding efficiently.

## Features
- Conditional Statements: Utilize `if-elif-else`, `for` and `while` logic for control flow.
- Variables: Declare and use variables to manage data efficiently.
- Logging Functions: Output messages and errors using `log` and `err` for easier debugging.
- Arithmetic Operators: Perform basic mathematical operations with +, -, *, and /.
- Comparison Operators: Use `==, !=, >, <, >=,` and `<=` to compare values.
- Logical Operators: Implement logic using `and` and `or`.
- Loop: `compose { }` for runtime loop.
- Math Module: consts `math.pi`, `math.tau`, `math.e` and functions `math.ceil()` and `math.floor()`.
## Usage
Write Recolon programs in files with a .rcn extension and execute them using the Recolon interpreter (once available).

### Syntax
```
var x = 5;
var y = 2;

if (x == y) {
  log("This is a log message");
} elif (x == 5) {
  log(x);
} else {
  err("This is an error message");
}

while (x != y and x == 5) {
    log("This will log infinitely);
}

for (var i = 0; i < 10; i = i + 1) {
    log(i);
}
```
