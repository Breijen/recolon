# Recolon Roadmap

## Overview
Recolon is a new programming language, built on the principles of Rust, designed to deliver high-performance capabilities while maintaining a high level of abstraction. The idea behind Recolon is to make it easier to build high-performance applications while still enjoying the benefits of a modern, expressive language.

As we work on Recolon, we're setting clear milestones to keep us focused and moving forward. Each milestone represents an important step in making Recolon a solid and practical choice for developers who want the best of both worlds: speed and safety. The development of Recolon is guided by key milestones, each representing significant steps towards creating a robust and versatile language. These milestones are intended to span more than a year, ensuring long-term functional progress, and are assigned version numbers for easy reference and integration into development workflows.

## Version 0.1.0: MVP (Minimum Viable Product)

### Goals
The primary goal of Version 0.1 is to establish a foundational version of Recolon that developers can use for initial evaluation. This version aims to showcase the language's potential for combining high-level programming constructs with high performance and safety. The key objectives for this milestone include:

- **Clear Understanding of Recolon’s Evolution:** Evaluators should gain insight into Recolon’s long-term goals, particularly how it integrates performance optimization with high-level abstractions and safety features.
- **Cohesive and Documented Language Design:** The language design should be consistent and well-documented, providing developers with a clear understanding of Recolon's core principles and features.
- **Core Language Features:** The 0.1 version should introduce essential features that support high-level programming, such as structs and classes, while ensuring that performance is not compromised.
- **Standard Library:** An initial standard library should be available, covering fundamental components needed for basic development tasks.
- **Project Features:** The key project components, including the toolchain and basic documentation, should be operational and accessible.

### Language Features
At this stage, the focus is on implementing the essential language features necessary for a functional MVP. These include:

#### Code Organization and Structuring
- Modular packages and libraries to support organized codebases.
- Importing mechanisms to facilitate code reuse and modularity.

#### Type System and Structs
- Definition and use of structs and classes for data organization.
- Support for single inheritance and basic polymorphism within classes.
- Implementation of Rust-inspired safety features such as ownership and borrowing, integrated within the type system.

#### Functions, Statements, and Expressions
- Function overloading to provide flexibility in method definitions.
- Control flow statements, including conditions, loops, and pattern matching, to enable robust program logic.
- Basic error handling mechanisms that emphasize safety and prevent common pitfalls like null pointer dereferencing.

#### Standard Library Components
- Fundamental types such as integers, floats, and booleans, along with support for strings and arrays.
- Initial implementations of collections and utilities that leverage Recolon’s safety features.

### Project Features
In addition to language features, several critical project-related components are necessary for the 0.1 release:

- **Recolon Toolchain:** A fully functional toolchain that supports major platforms (Windows, macOS, Linux), enabling developers to compile and run Recolon code efficiently.
- **Basic Documentation:** Comprehensive documentation that includes installation guides, a getting started tutorial, and FAQs to help developers quickly onboard and begin experimenting with Recolon.