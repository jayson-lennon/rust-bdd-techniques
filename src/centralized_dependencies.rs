//! If you don't want to apply trait bounds to a structure, then using a dependency container can
//! solve this.
//!
//! Benefits:
//! - All dependencies are in one place, so it's easy to manage
//! - You only need to write one extra trait for the container
//! - Dependent structures only need to have trait bounds on methods
//! - Only required dependencies need to be mocked for testing
//!
//! Drawbacks:
//! - The container must be behind a pointer in order to be properly shared aross a complex
//!   application. This might incur a performance cost.
//! - By design, the container itself can be used as a trait bound for functions. If a function
//!   only requires one dependency, but uses the container as a trait bound, then this obscures
//!   what the function actually requires.
//! - The container has to be passed around to all or a majority of the functions across the entire
//!   application

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::default_constructed_unit_structs)]
#![allow(clippy::disallowed_names)]

use std::sync::Arc;

// Here are two traits we will use for the example.
pub trait Foo {
    fn foo(&self);
}
pub trait Bar {
    fn bar(&self);
}

// These structures provide implementations for the traits.
pub struct FooImplA;
pub struct FooImplB;
impl Foo for FooImplA {
    fn foo(&self) {
        println!("foo A");
    }
}
impl Foo for FooImplB {
    fn foo(&self) {
        println!("foo B");
    }
}

// Alternative implementations for the traits (ie: test versions).
pub struct BarImplA;
pub struct BarImplB;
impl Bar for BarImplA {
    fn bar(&self) {
        println!("bar A");
    }
}
impl Bar for BarImplB {
    fn bar(&self) {
        println!("bar B");
    }
}

// An an accessor trait to get a dependency. The name is intentionally short because we will likely
// use this as a trait bound all over the code base.
pub trait Deps {
    // Each dependency required by the program would be listed here as an associated type.
    type FooImpl: Foo;
    type BarImpl: Bar;

    // We then provide getters for each dependency.
    fn foo(&self) -> &Self::FooImpl;
    fn bar(&self) -> &Self::BarImpl;
}

// We need a container for all of our dependencies. `Clone` is derived here because we will be
// sharing this across the application. Any mutations need to be done internally by implementations
// of `Foo` or `Bar`.
#[derive(Clone)]
pub struct DependencyContainer<F, B>
where
    F: Foo,
    B: Bar,
{
    // Wrap the dependencies in `Arc` so we can share them easily.
    foo: Arc<F>,
    bar: Arc<B>,
}

// Implement the accessor trait.
impl<F, B> Deps for DependencyContainer<F, B>
where
    F: Foo,
    B: Bar,
{
    type FooImpl = F;
    type BarImpl = B;

    fn foo(&self) -> &Self::FooImpl {
        &self.foo
    }

    fn bar(&self) -> &Self::BarImpl {
        &self.bar
    }
}

// This is a structure that requires a dependency. We don't apply the dependency as a trait bound
// on the structure because the bound would be propagated to all users.
#[derive(Default)]
pub struct MyStruct;

impl MyStruct {
    // Instead of trait bounds on the structure, we place trait bounds on the specific methods that
    // require a dependency. We want to be as specific as possible here and _not_ depend on the
    // entire `Deps` container. Depending on the entire `Deps` container would mean that we need to
    // provide implementations for everything in order to run a test.
    pub fn do_foo<F: Foo>(&self, foo: &F) {
        foo.foo();
    }

    pub fn do_bar<B: Bar>(&self, bar: &B) {
        bar.bar();
    }
}

// On functions, we can pass around the `Deps` by either borrowing or cloning.
fn use_my_struct_with_deps<D: Deps>(my_struct: &MyStruct, deps: &D) {
    // Methods that require a specific dependency can be given access to it via the getter method.
    my_struct.do_foo(deps.foo())
}

// This also works too, but we cannot go back to a `Deps` if we need to. Functions like this should
// only be used at the very end of a chain of function calls.
fn use_my_struct_with_foo<F: Foo>(my_struct: &MyStruct, foo: &F) {
    my_struct.do_foo(foo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let deps = DependencyContainer {
            foo: Arc::new(FooImplA),
            bar: Arc::new(BarImplB),
        };

        let my_struct = MyStruct::default();
        use_my_struct_with_deps(&my_struct, &deps);
        use_my_struct_with_foo(&my_struct, deps.foo());
    }
}
