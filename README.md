## CRUST
### C/C++ to Rust Transpiler
C++ is a system programming language widely used in development of operating system,  firmwares, device drivers and in some application development. Major drawback with C++ or any other system programming language is memory safety, null pointers and dangling pointers, which are very dangerous if not handled properly by the programmer. The new programming language *Rust* is system programming language provides the safe and secure programming with highly enforced compiler restrictions with zero cost abstraction.

------------------------------------------------------------------------
This tool is intended to translate existing C++ code base into Rust with less effort.
May require manual lookup or minute edit to the translated code.
------------------------------------------------------------------------
##Usage
First, make sure you are setup for Rust development. Check out [http://www.rust-lang.org](http://www.rust-lang.org) for more information. The installation sets up the Rust compiler and Cargo package management system. Also, it adds `rustc` and `cargo` commands to your PATH variable.

Now that you're setup for rust. Open any suitable terminal and `cd` into the crust directory

Run `cargo build` to compile the entire project and download some dependencies.

Now, you many test out CRUST using some of the examples in the example folder as follows:
`cargo run`

The program asks for the C/C++ file to be converted to Rust (Enter the path relative to the current location):
```
Enter the C/C++ file to be converted to Rust : examples/prog.cpp
```
Next, enter the translation mode, Strict or Loose (default is Loose). Strict mode assumes all variables to be immutable and Loose mode makes all variables mutable.

It also asks whether the program should be converted into a cargo project. It essentially sets up the program into a project with package management. Check out [http://doc.crates.io/](http://doc.crates.io/) for more information.
```
Enter the translation mode [(S/s)trict/(L/l)oose] : l
Do you want to create a cargo project :[Y/N]n
```

Now, it completes the translation and shows where the translated file is stored
```
Input file size : 177bytes
Tokenizing.......       :DONE
Invoking Parser ........        :DONE
Rust equivalent of source of `examples/prog.cpp` is generated successfully, View the rust code in file : ./examples/pro
g.rs
```
------------------------------------------------------------------------
Alternatively, you may generate an executable similar to C executable as follows:

`cargo build --release`

This generates an executable, `/target/release/crust` which can be moved anywhere and run anytime using:
`./crust`
