# Rust TDD Notes & Techniques

(WIP) This repo has some examples of ways you can structure Rust code for
test-driven development or behavior-driven development.

## Behavior-Driven Development

See the [this source file](src/bdd.rs) for examples of BDD tests.

The quickest way to get started with
[behavior-driven-development](https://en.wikipedia.org/wiki/Behavior-driven_development)
is to structure your tests like this:

```rust
#[test]
fn my_object_under_test_does_a_specific_thing_with_correct_input() {
    // Given [initial state]

    // When [action is taken]

    // Then [assert result]
}
```

Things to note:

- The name of the test function should be long and descriptive with enough information to understand:
  1. The behavior under test
  2. What category of input we are using with the behavior (happy path, error path, etc)
  3. What the result should be
- Tests should assert that a single concept works, ideally being checked with a single assert.
  - A _concept_ means one behavior of a program. "The file is saved" is one concept, but may have two asserts like `assert!(path.exists())` and `assert_eq!(file_contents, "expected data")`.
  - A single assert line is the _goal_, but not all tests will meet this goal and that's OK.
- The `Given, When, Then` exists in the test as comments and properly describe what is happening

### Given

The "Given" line describes the state of the application where the test is
applicable. It should be concise and doesn't have to be a complete sentence.
Here are some examples:
- Given the search box is selected
- Given the file does not exist
- Given the user didn't enter a matching password in the password confirmation box
- Given a network failure when connecting to the database

After the comment line, write the setup code required to get the system to the
state described. This should be somewhere between 1 to 5 function calls
regardless of system complexity. If your setup code is longer than 5 calls,
then please check out the next section on [Change-Resistant
Tests](#change-resistant_tests).

### When

### Then

## Change-Resistant Tests

(how to write tests that don't break)

## Test Doubles

## Test Tables

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

See the [example source file](src/trait_abstraction.rs) for how to implement `BarHandle`.

## Traits as Associated Types

(example using traits that have associated types that are traits and then storing those in structures)

## Mutation Testing
