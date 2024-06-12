//! It is recommended to avoid using trait bounds on structures because the bounds are infectious
//! and must be present on all usages of the structure. However, this is unavoidable when composing
//! larger structures where you also want to allow the implementations to be stubbed/mocked out
//! while using TDD.
//!
//! To avoid the infectious trait bounds, we can create another trait and encapsulate all of the
//! required traits within.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::default_constructed_unit_structs)]

// Here we have two traits. These are the ones that are required by our structure.
pub trait TraitOne {}
pub trait TraitTwo: Default {}

// These structures provide implementations for the traits.
pub struct FooOne;
#[derive(Default)]
pub struct FooTwo;
impl TraitOne for FooOne {}
impl TraitTwo for FooTwo {}

// Alternative implementations for the traits (ie: test versions).
pub struct BarOne;
#[derive(Default)]
pub struct BarTwo;
impl TraitOne for BarOne {}
impl TraitTwo for BarTwo {}

// The structure that requires two pieces of functionality defined by `TraitOne` and `TraitTwo`.
//
// We set a default for `TraitTwo` to use the `FooTwo` implementation in the event that the `Two`
// is not provided.
pub struct MyStruct<One, Two = FooTwo>
where
    One: TraitOne,
    Two: TraitTwo,
{
    one: One,
    two: Two,
}

// Implementation where we provide a default `Two`.
impl<One> MyStruct<One, FooTwo>
where
    One: TraitOne,
{
    pub fn new(one: One) -> Self {
        Self {
            one,
            two: FooTwo::default(),
        }
    }
}

// Implementation where `Two` is provided.
impl<One, Two> MyStruct<One, Two>
where
    One: TraitOne,
    Two: TraitTwo,
{
    pub fn with_two(one: One, two: Two) -> Self {
        Self { one, two }
    }
}

// If we use `MyStruct` as-is, then we need to include these bounds on every call site, and on
// every structure that contains `MyStruct`.
//
// Adding another piece of functionality via trait bounds will require changing the bounds on this
// function and likely all callers of this function, even though the function body is unchanged.
fn use_my_struct<One, Two>(my_struct: MyStruct<One, Two>)
where
    One: TraitOne,
    Two: TraitTwo,
{
}

// To avoid infectious traits, we create this "handle" trait. I call it "handle" because if
// provides a "handle" to the concrete structure.
trait MyStructHandle {
    // These associated types are the same bounds as the structure.
    //
    // We can add add/remove types later and downstream will be unaffected. We only need to update
    // the struct, this trait, and the below impl block.
    type OneImpl: TraitOne;
    type TwoImpl: TraitTwo;

    /// Borrows the concrete structure using whichever implementations it happens to contain.
    fn get(&self) -> &MyStruct<Self::OneImpl, Self::TwoImpl>;
}

// To implement the `MyStructHandle`, we need to do it generically for all `MyStruct`.
impl<One, Two> MyStructHandle for MyStruct<One, Two>
where
    // Same trait bounds again.
    One: TraitOne,
    Two: TraitTwo,
{
    // The associated types are generic using whatever types `One` and `Two` happen to be.
    type OneImpl = One;
    type TwoImpl = Two;

    fn get(&self) -> &MyStruct<Self::OneImpl, Self::TwoImpl> {
        // return self since it's already borrowed
        self
    }
}

// We can use the `MyStructHandle` instead of providing the same bounds as the structure like in `use_my_struct`.
fn use_my_struct_with_handle<M>(handle: &M)
where
    M: MyStructHandle,
{
    // Whenever we call `.get()`, we get a `&MyStruct<One, Two>` where `One` is some type that
    // implements `One`, and `Two` is some type that implements `Two`. We don't know what `One` and
    // `Two` are, only that they implement those traits.
    let my_struct = handle.get();

    // Since `MyStruct` implements `MyStructHandle`, we can pass either directly to another
    // function that uses the same bounds:
    use_my_struct_with_handle_again(my_struct);
}

fn use_my_struct_with_handle_again<M>(handle: &M)
where
    M: MyStructHandle,
{
    let my_struct = handle.get();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // Type annotations are optional.

        // using the default type for `Two`
        let my_struct: MyStruct<BarOne, FooTwo> = MyStruct::new(BarOne);
        use_my_struct_with_handle(&my_struct);

        // manually setting thw `Two` type
        let my_struct: MyStruct<FooOne, BarTwo> = MyStruct::with_two(FooOne, BarTwo);
        use_my_struct_with_handle(&my_struct);
    }
}
