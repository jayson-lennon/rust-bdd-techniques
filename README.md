# Rust TDD Notes & Techniques

(WIP) This repo has some examples of ways you can structure Rust code to more
easily use test-driven development or behavior-driven development.

## Trait Abstraction

Using TDD in Rust inevitably demands applying trait bounds to structures
(unless you are OK with `Box<dyn T>`). This is problematic because the bounds
impact all usages of the structure making it unreasonable to use in larger
codebases. However, it's possible to abstract away the trait bounds into
another trait which then allows you to add or remove structure trait bounds
without impacting other code.

Given this example structure:

```rust
struct Bar<A, B>
where
    A: BoundA,
    B: BoundB
{
    a: A,
    b: B
}
```

If you specify the bounds on function `foo`, then changing them on the
structure will require updating `foo` and likely many functions that `foo`
relies on:

```rust
// Bounds specified here will need to be updated if `Bar` changes:
fn foo<A, B>(bar: &Bar<A, B>)
where
    A: BoundA,
    B: BoundB;
```

You can instead use a trait abstraction which hides the trait bounds of `Bar`:

```rust
// `BoundA` and `BoundB` are hidden within `BarHandle`, so the bounds on `Bar`
// can be changed freely without impacting `foo`:
fn foo<T>(bar: &T)
where
    T: BarHandle;
```

See the [example source file](src/trait_abstraction.rs) on how to implement `BarHandle`.
