<p align="center">
  <img src="https://github.com/breijen/recolon/blob/main/rcn-logo.png" alt="Recolon Logo">
</p>


# Recolon Programming Language

Recolon is an experimental programming language developed in Rust, designed to merge the expressive power of high-level languages with the safety and performance of Rust. It's a language that prioritizes simplicity and efficiency, making it ideal for developers who want to build robust applications with minimal overhead.

## Philosophy

The core idea behind Recolon is to provide a language that is easy to learn and use while maintaining a strong focus on safety and performance. By leveraging Rust's memory safety guarantees, Recolon allows developers to write high-level code without sacrificing control over low-level details.

## Getting Started

To begin working with Recolon, visit the [official documentation](https://recolon-lang.org/) for detailed installation instructions, language syntax, and more.

### A Simple Example

Here’s what a basic "Hello, World!" program looks like in Recolon:

```recolon
fn main() {
    log("Hello, world!");
}

main();
```

This small example highlights Recolon's straightforward syntax and ease of use. The log function is a built-in feature for outputting text, similar to the println! macro in Rust.

## Syntax Highlighting

There is a very barebones syntax highlighter available for Visual Studio Code. This highlighter provides basic syntax highlighting for Recolon code, making it slightly easier to work with. However, it is still in its early stages and may not support all features of the language.

### Accessing the Syntax Highlighter

The syntax highlighter is available in a separate repository. You can find it [here](https://github.com/Breijen/recolon-highlights).

### Installing the Syntax Highlighter

1. **Download the Syntax Highlighter:** Clone or download the repository containing the syntax highlighter.
2. **Install in VS Code:**
    - Open Visual Studio Code.
    - Go to `File > Preferences > Extensions`.
    - Search for `Recolon Syntax` if published, or manually install by selecting the "Install from VSIX..." option.
3. **Activate the Highlighter:**
    - Open your Recolon file (`.rcn` extension).
    - The syntax highlighter will be applied automatically, providing basic syntax coloring.

## Community and Contributions

Recolon is an open-source project, and contributions are welcome! Whether you find a bug, want to suggest a new feature, or contribute to the documentation, feel free to open an issue or submit a pull request.

## License

Recolon is distributed under the MIT License.
