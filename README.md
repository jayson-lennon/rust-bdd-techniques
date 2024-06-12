# Rust TDD Notes & Techniques

(WIP) This repo has some examples of ways you can structure Rust code for
test-driven development or behavior-driven development.

## Test-Driven Development

## Behavior-Driven Development

See [this source file](src/bdd.rs) for examples of BDD tests.

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

The "When" line describes _a single_ action that is taken:
- When the form is saved for later
- When trying to submit a bug report
- When checking the password complexity
- When pruning log files

After the comment line, write the code required to perform the action. This
should nearly always be a single function call.

### Then

The "Then" line describes the end result of what should happen after executing
the "When" comment:
- Then the file is saved
- Then the program exits
- Then no new records are created
- Then a popup is displayed

After the comment line, write the assertion needed to confirm that the end
result is what was expected. Nearly all tests should have 1 assertion and it
should confirm whatever was described in the "Then" comment. If you feel like
you need more than 1 assertion, ask yourself these questions:
- Do both assertions confirm what was expected by the "Then" comment?
- Am I asserting that a single behavior occurred?
If you answered "no" to either question, then you need another test.

## Change-Resistant Tests

(how to write tests that don't break)

## Test Doubles

## Test Tables

## Centralized Dependency Container

An simple way to manage dependencies is to put all of them into a single
container structure. This container can then be shared across the application and
individual dependencies can be accessed when needed.

This works well, however it is important not to share the central dependency
container outside of the core of your application. In other words, functions
and structures should only take what they need from the container, otherwise it
becomes unclear which parts of the application have a certain dependency.

Using a dependency container looks like this:

```rust
// Functions should only use needed dependencies and avoid using the entire
// container.
fn use_foo<F: Foo>(foo: &F) {
  foo.do_foo();
}

let container = DependencyContainer::default();
// access the `foo` dependency
use_foo(container.foo());
```

Using a central dependency container is convenient and makes it easier to write
tests because functions can be tested by implementing only the required traits
and not an entire container.

See the [example source file](src/centralized_dependencies.rs) for details and
how to implement a centralized dependency container.

## Trait Abstraction

At some point it may be beneficial to apply trait bounds to structures, such as
when you want the structure to manage the lifetime of it's dependency, or if
you want to avoid reaching out into an `Arc` in a high-performance part of the
system. However, applying trait bounds to structures is infectious: all users
of the structure now must specify which bounds (dependencies) are used. This
may also apply all the way up the call stack, which is not at all manageable.

It's possible to encapsulate all the dependencies a structure has by using
trait abstraction. Only one trait bound will be required to pass the structure
to functions, and the dependencies can be changed without impacting the rest of
the application.

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

If you specify the bounds on function `foo` and then change them later, this
will require updating `foo` and likely many functions that `foo` relies on:

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

Using trait abstraction directly on a structure (as opposed to using a central
dependency container) can have better performance, but makes testing slightly
more complicated because all dependencies must be constructed for all tests,
and there is also more boilerplate for each structure.

See the [example source file](src/trait_abstraction.rs) for more details and
how to implement the `BarHandle` abstraction.

## Traits as Associated Types

(example using traits that have associated types that are traits and then storing those in structures)

## Mutation Testing
