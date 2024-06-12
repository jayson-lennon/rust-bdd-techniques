# Rust TDD Notes & Techniques

(WIP) This repo has some examples of ways you can structure Rust code to more
easily use test-driven development or behavior-driven development.

## Trait Abstraction

[Example source file](src/trait_abstraction.rs)

Using TDD in Rust inevitably demands applying trait bounds to structures
(unless you are OK with `Box<dyn T>`). This is problematic because the bounds
impact all usages of the structure making it unreasonable to use in larger
codebases. However, it's possible to abstract away the trait bounds into
another trait which then allows you to add or remove structure trait bounds
without impacting other code.

