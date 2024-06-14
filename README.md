# BDD Notes & Techniques for Rust

This repo has some examples of different techniques you can use with Rust for behavior-driven development.

| File | Description |
|----------------------------------------------------------|---------------------------------------------------------------|
| [src/function_as_service.rs](src/function_as_service.rs) | Basic BDD & how to work with a slow function while keeping tests running fast. |
| [src/centralized_dependencies.rs](src/centralized_dependencies.rs) | Shows how to create a dependency container. |
| [src/trait_abstraction.rs](src/trait_abstraction.rs) | Shows how a `struct` can carry it's own generic dependencies without impacting callers.  |

Table of Contents
=================

* [Behavior-Driven Development](#behavior-driven-development)
* [Change-Resistant Tests](#change-resistant-tests)
  * [Integration Testing / E2E](#integration-testing--e2e)
* [Test Doubles](#test-doubles)
  * [Dummy](#dummy)
  * [Stub](#stub)
  * [Spy](#spy)
  * [True Mock](#true-mock)
  * [Fake](#fake)
* [Test Tables](#test-tables)
* [Centralized Dependency Container](#centralized-dependency-container)
* [Trait Abstraction](#trait-abstraction)
* [Traits as Associated Types](#traits-as-associated-types)
* [Mutation Testing](#mutation-testing)

## Behavior-Driven Development

Quick step-by-step guide for BDD:
1. Isolate individual behaviors that your app needs to do
2. Write a test case for a single behavior with a single outcome
3. In the test case, write the ideal API that you would like in order to achieve the behavior
  - After writing the API in the test case, your test will fail because there is no implementation yet
4. Implement the API in order to pass _just_ this test case.
  - Use a minimal implementation. Don't add code in anticipation of what you might need later.
  - Refactor your implementation _and_ your test cases as needed.
5. Go back to step 1

Structure your tests like this:

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
- Tests should assert that a single behavior works, ideally being checked with a single assert.
  - "The file is saved" is one behavior, but may have two asserts like `assert!(path.exists())` and `assert_eq!(file_contents, "expected data")`.
  - A single assert is the _goal_, but not all tests will meet this goal and that's OK.
- The `Given, When, Then` exist in the test as comments and properly describe what is happening

### Given

The "Given" line describes the state of the application where the test is
applicable. It should be concise and doesn't have to be a complete sentence.
Here are some examples:
- Given the search box is selected
- Given the file does not exist
- Given the user didn't enter a matching password in the password confirmation box
- Given a network failure when connecting to the database

After the comment line, write the setup code required to get the system to the
state described. This should be somewhere between 1 to 3 function calls
regardless of system complexity. If your setup code is longer than 3 calls,
then please check out the next section on [Change-Resistant
Tests](#change-resistant_tests).

### When

The "When" line describes _a single_ action that is taken:
- When the form is saved for later
- When trying to submit a bug report
- When checking the password complexity
- When pruning log files

After the comment line, write the code required to perform the action. This
should almost always be a single function call.

### Then

The "Then" line describes the end result of what should happen after executing
the "When":
- Then the file is saved
- Then the program exits
- Then no new records are created
- Then a popup is displayed

After the comment line, write the assertion needed to confirm that the end
result is what was expected. Nearly all tests should have 1 assertion and it
should confirm whatever was described in the "Then". If you feel like you need
more than 1 assertion, ask yourself these questions:
- Do both assertions confirm what was expected by the "Then"?
- Am I asserting that a single behavior occurred?
If you answered "no" to either question, then you need another test.

## Change-Resistant Tests

Test code is production code, treat it with the same level of craftsmanship
that you would for the code under test.

Nearly all setup ("Given") code should be placed behind helper functions or
builders, even for trivial cases. These will break when you change an API, but
dependent tests won't break since the breakage was isolated. When done
correctly, you can have hundreds of tests and still freely change your APIs
with just a small amount of isolated breakage.

Here is an example of _not_ using helpers, which will result in frequent test
breakage:

```rust
#[test]
fn bad_example() {
    // `Foo` is created directly, so if we change the parameters on `Foo::new`,
    // then this test will break.
    let foo: Foo = Foo::new(5);

    // `Bar` is created using it's own builder. This isn't _too_ bad, but
    // creating test-specific builders allow more control on exactly how `Bar`
    // is built.
    let bar: Bar = Bar::builder().using_feature(DummyFeature).build();

    let result = super::foo_the_bars(&foo, &bar);

    assert!(result.is_ok());
}
```

Correctly encapsulating setup code isolates breakage to single points:

```rust
fn create_foo() -> Foo {
  // (setup code here)
}

// Builder to create a `Bar`
#[derive(Default)]
struct StubBarBuilder;

impl StubBarBuilder {
  // (methods and stuff)
}

#[test]
fn good_example() {
    // `Foo` is created with a helper function. All tests using this function
    // won't break when `Foo` changes.
    let foo: Foo = create_foo();

    // We typically have many different configurations we want to test. Creating
    // a test-specific builder to build the target structure encapsulates the
    // target structure behind the builder. As a bonus, we can wrap up
    // multiple build steps and generate dummy data.
    let bar: Bar = StubBarBuilder::default()
        .using_that_big_clients_features()
        .with_fake_entries(10)
        .build();

    let result = super::foo_the_bars(&foo, &bar);

    assert!(result.is_ok());
}
```

### Integration Testing / E2E

Integration tests (and definitely end-to-end testing) will require a lot of
setup code, and the function under test may be called many times under
different circumstances. To avoid breakage in these situations you can
encapsulate the function under test with a wrapper. However, this also requires
encapsulating the entire setup code to make it specific for that one function.
For this reason, it's better to use this on the edges of a system where the
function under test is frequently the same across many tests. Here is how it
would look:

```rust
// Create a builder that encapsulates everything required by the function under
// test.
#[derive(Default)]
struct FooTheBarsArgs {
    foo: Foo,
    bar: Bar,
}

impl FooTheBarsArgs {
  // (methods and stuff)
}


#[test]
fn fully_encapsulated_example() {
    // Test-specific builder just for the `foo_the_bars` function. Using this
    // will allow the setup code to resist changes. Recommend packing a lot of
    // different options into this builder in order to maximize re-use across
    // many  tests.
    let args = FooTheBarsArgs::builder()
        .using_foo(FooA)
        .with_bar(BarB)
        .and_feature(DummyFeature)
        .with_fake_entries(5)
        .build();

    // Pass the args to a helper function. Since the helper function always
    // accepts a `FooTheBarsArgs`, we don't have to worry about changes to
    // the setup code nor changes to the function under test.
    let result = foo_the_bars(&args);

    assert!(result.is_ok());
}

// wrapper function for the function under test
fn foo_the_bars(args: FooTheBarsArgs) -> Result<usize, ()> {
    // now the API can change and only this function needs to be updated
    super::foo_the_bars(&args.foo, &args.bar)
}
```

## Test Doubles

When developing with TDD you'll essentially be programming with traits. It is
very easy to generate a test double from a trait using an IDE and then fill in
the parts that you need. So don't be afraid to manually write a stub or spy
just for a specific test.

There are mocking tools such as [`mockall`](https://crates.io/crates/mockall)
which can generate mocks, but as trait bounds become more complex it ends up
being easier to just write one by hand. Most of the time a mock isn't needed
and a stub or spy works just fine.


[Uncle Bob's
post](https://blog.cleancoder.com/uncle-bob/2014/05/14/TheLittleMocker.html) is
a fantastic resource to learn about different test doubles. Here is a breakdown
of each kind:

| **Test Double Kind**                  | **Behavior**                                                                                           |
|---------------------------------------|--------------------------------------------------------------------------------------------------------|
| **Dummy**                             | Used when the object is required but not actually used in the test. Can just leave the implementation as `todo!()`.             |
| **Stub**                              | Provides predefined responses to method calls. Used to control the behavior of the tested code.         |
| **Spy**                               | Records information about the interactions with its methods. Used to verify the interactions.           |
| **True Mock**                         | Similar to a Spy but with built-in assertions to check expected behaviors.                              |
| **Fake**                              | Contains business logic for testing purposes. Simulates parts of the system with real behavior.         |

I try to use the above terms in the test double structure in order to identify
what kind it is. So if I have `DummyThing` then I know it doesn't do anything,
if I have a `SpyThing` then I know it's tracking method calls, etc.

### Dummy

Dummy implementations don't do anything. They are there because they have to
be. Use your IDE to generate it from a trait and you're done.

```rust
trait Feature {
    fn foo(&self, n: i32) -> String;
}

struct DummyFoo;

impl Feature for DummyFoo {
    fn foo(&self, n: i32) -> String {
        todo!()
    }
}
```

### Stub

Stubs respond with some data that you provide. A simple stub will respond with
the same thing each time it's called, and more complicated ones will respond
with different things based on the input arguments or the number of times
called. For example, you may want a stub that succeeds on the first few calls
and then fails on the next in order to simulate intermittent failures.

```rust
trait Feature {
    fn foo(&self, n: i32) -> String;
}

struct StubFoo;

impl Feature for StubFoo {
    fn foo(&self, n: i32) -> String {
        if n == 5 {
            "foo".to_string()
        } else {
            "bar".to_string()
        }
    }
}
```

### Spy

Spies track how they are interacted with. You can track any combination of
arguments, number of method calls, and return values. If you are going to use
spies frequently, it helps to write a few small helpers to encapsulate inner
mutability.

```rust
trait Feature {
    fn foo(&self, n: i32) -> String;
}

#[derive(Default)]
struct SpyFoo {
    // Put Arc<AtomicUsize> in own struct if using frequently
    foo_calls: Arc<AtomicUsize>,
    // Put Arc<Mutex<Vec<T>>> in own struct if using frequently
    n_args: Arc<parking_lot::Mutex<Vec<i32>>>,
}

impl Feature for SpyFoo {
    fn foo(&self, n: i32) -> String {
        self.foo_calls.fetch_add(1, Ordering::SeqCst);

        let mut n_args = self.n_args.lock();
        n_args.push(n);

        "foo".to_string()
    }
}

// using helpers would look something like this
impl Feature for SpyFoo {
    fn foo(&self, n: i32) -> String {
        self.foo_calls.increment();
        self.n_args.push(n);

        "foo".to_string()
    }
}

#[test]
fn sample() {
    let spy = SpyFoo::default();

    // run the test
    // ...

    // make sure `Foo` was called with the expected values
    assert_eq!(*spy.n_args.lock(), vec![42, 999]);
}
```

### True Mock

True mocks are basically spies with an extra assert method attached. So after
you run your behavior, instead of `assert!(thing);`, you can use
`my_mock.assert_the_thing();` or `assert!(my_mock.the_thing_happened())`.

True mocks are useful in integration tests. You can import a true mock from one
crate/module and then use it in a test for your current module. Since the true
mock can assert it's own behavior, you don't need to worry about the specifics
of the other crate/module.

```rust
trait Feature {
    fn foo(&self, n: i32) -> String;
}

struct MockFoo {
    discovered_life: Arc<AtomicBool>,
}

impl MockFoo {
    // This is the thing that the mock is aware of, but users of the mock may
    // not necessarily know.
    pub fn discovered_the_meaning_of_life(&self) -> bool {
        self.discovered_life.load(Ordering::Relaxed)
    }
}

impl Feature for MockFoo {
    fn foo(&self, n: i32) -> String {
        // This will trigger the mock assertion.
        if n == 42 {
            self.discovered_life.store(true, Ordering::Relaxed);
        }
        "foo".to_string()
    }
}

#[test]
fn feature() {
    let mock = MockFoo::default();

    // run the test
    // ...

    assert!(mock.discovered_the_meaning_of_life());
}
```

### Fake

Fakes are simulated versions of the real thing. They can be useful for
simulating external services and for services that you make heavy use of in
many test cases. But be careful not to re-implement the entire service in a
fake. You'll also probably want a few unit tests to make sure the fake works
correctly.

```rust
type Id = usize;
type Record = &'static str;

struct FakeFoo {
    next_id: Arc<AtomicUsize>,
    data: DashMap<Id, Record>,
}

// A data repository is a common thing that will be used heavily throughout an
// application, making it a good fake candidate.
pub trait Repo {
    fn create(&self, record: Record);
    fn read(&self, id: Id) -> Option<Record>;
    fn update(&self, id: Id, record: Record);
    fn delete(&self, id: Id) -> bool;
}

impl Repo for FakeFoo {
    fn create(&self, record: Record) {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.data.insert(id, record);
    }

    fn read(&self, id: Id) -> Option<Record> {
        self.data.get(&id).map(|rec| rec.value().clone())
    }

    fn update(&self, id: Id, record: Record) {
        self.data.insert(id, record);
    }

    fn delete(&self, id: Id) -> bool {
        self.data.remove(&id).is_some()
    }
}
```

You can also add flags to a fake builder to trigger different behaviors at
different points. Like if you want a failure to occur after some number of
method calls, you can add a counter and then return errors after the counter
reaches the desired amount.

## Test Tables

Test tables provide a way to test multiple things in one test. These are useful
when testing something where the return value is obvious, and doing a "Given,
When Then" doesn't add much (or any) value:

```rust
#[test]
fn calculates_line_slope() {
    let cases = [
        // (x1, y1), (x2, y2), slope kind
        (Line::from((0, 0), (1, 1)), Slope::Up(1)),
        (Line::from((1, 1), (0, 0)), Slope::Down(-1)),
        (Line::from((0, 0), (1, 0)), Slope::Flat),
        (Line::from((0, 1), (0, 0)), Slope::Vertical),
    ];
    for (i, (a, b, expect)) in cases.into_iter().enumerate() {
        assert_eq!(calculate_slope(a, b), expect, "failure on case {i}");
    }
}
```

You can also wrap up creation of each case with a function if the setup code is
long or noisy. Add comments to each case if it's not obvious what is being
tested.

## Centralized Dependency Container

An simple way to manage dependencies is to put all of them into a single
container structure. This container can then be shared across the application and
individual dependencies can be accessed when needed.

This works well, however it is important that the dependency container _not_ be
shared outside of the core of your application. In other words, functions and
structures should only take what they need from the container, otherwise it
becomes unclear which parts of the application have a certain dependency. Using
the entire container everywhere also implies that the entire container would
need a test double for every test, even if only 1 dependency is used.

Using a dependency container looks like this:

```rust
// Functions should only use required dependencies and avoid using the entire
// container.
fn use_foo<F: Foo>(foo: &F) {
  foo.do_foo();
}

let container = DependencyContainer::default();
// access the `foo` dependency
use_foo(container.foo());
```

See the [example source file](src/centralized_dependencies.rs) for details and
how to implement a centralized dependency container.

## Trait Abstraction

At some point it may be beneficial to apply trait bounds to structures, such as
when you want the structure to manage the lifetime of it's dependency, or if
you want to avoid reaching out into an `Arc` in a high-performance part of the
system. However, applying trait bounds to structures is infectious: all users
of the structure must specify which bounds (dependencies) are used. This may
also make it all the way up the call stack, which is not at all manageable.

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

If you specify the bounds on function `foo` and then change `Bar` later, this
will require updating `foo` and likely many functions in the call stack for
`foo`:

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

However, it can be convenient to encapsulate the dependencies within the
structure because they they don't need to be pulled from a central dependency
container. This benefit increases the more dependencies that the particular
structure uses.

The implementation is a bit verbose, so check out the [example source
file](src/trait_abstraction.rs) for more details on how to implement the
`BarHandle` abstraction.

## Mutation Testing

It's difficult to model all possible behaviors of a program. To help with this,
mutation testing can be used.

Mutation testing works by modifying various parts of your code such as changing
`<` to `>`, always returning `true` from a function that returns a `bool`, or
returning `-1`, `0`, and `1` from a function that returns a number. After
changing the code, the mutation test will run your test suite and check if your
tests still pass. If all the tests pass, then this means that there is a code
path that was not tested because at least 1 test should fail if the
implementation was changed to a broken state.

You can use the mutation test report to help identify untested behaviors in
order to improve your application.

Use [cargo-mutants](https://mutants.rs/) to generate mutants and run the mutation testing.
