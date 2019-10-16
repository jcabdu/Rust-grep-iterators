# Rust-grep-iterators
Rust version of the classic command line tool grep (globally search a regular expression and print), implemented with functional programming features (iterators and closures): 
- Functional programming prefers to minimize  the amount of mutable state to make code clearer.
- Removing the mutable state might enable a future enhancement to make searching happen in parallel.
- Iterators have better performance than loops: 
  - Iterators are zero-cost abstractions, using the abstraction imposes no additional runtime overhead ("What you don’t use, you don’t pay for; & what you do use, you couldn’t hand code any better!"). 
  - Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself.


Software development using the TEST-DRIVEN DEVELOPMENT (TDD) PROCESS: 
1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1! - 
