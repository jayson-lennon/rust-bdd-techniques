//! The purpose of this module is to show some basic BDD that encapsulates a single long-running
//! function. Using test doubles, we reduce the run time of the tests so we can get results
//! quicker.
//!
//! The long-running function is tested, but the tests are ignored so they have to be ran manually.
//! Use `cargo t -- --ignored` to run these ignored tests.

#![allow(dead_code)]

use std::time::Duration;

// This is some function that we need to call. It may be from a third-party crate, or it may be our
// own thing. It returns `true` on success, and `false` on failure.
fn is_meaning_of_life(n: i32) -> bool {
    std::thread::sleep(Duration::from_millis(500));
    n == 42
}

// Create a trait to run the function that we want.
trait LifeService {
    fn check(&self, n: i32) -> bool;
}

// Here is an implementor for the actual service
struct Life;
impl LifeService for Life {
    fn check(&self, n: i32) -> bool {
        is_meaning_of_life(n)
    }
}

// This is our entry point for accessing the function. We don't call `is_meaning_of_life` directly,
// and we don't deal with `Life` directly. We only use the trait.
fn run_meaning_of_life<L>(life: L, n: i32) -> String
where
    L: LifeService,
{
    if life.check(n) {
        "yay!".to_string()
    } else {
        ":frown:".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Here is an implementor for our test double.
    #[derive(Default)]
    struct StubLife {
        meaning: bool,
    }

    // Implement some methods to change the behavior.
    impl StubLife {
        fn always_correct(mut self) -> Self {
            self.meaning = true;
            self
        }
        fn always_wrong(mut self) -> Self {
            self.meaning = false;
            self
        }
    }

    // Implement the service
    impl LifeService for StubLife {
        fn check(&self, _: i32) -> bool {
            self.meaning
        }
    }

    #[test]
    #[ignore = "slow test only needs to run during full suite"]
    fn meaning_of_life_is_42() {
        assert!(is_meaning_of_life(42));
    }

    #[test]
    #[ignore = "slow test only needs to run during full suite"]
    fn get_wrong_answer_if_we_dont_provide_42() {
        assert!(!is_meaning_of_life(0));
    }

    // You'll notice in these tests that the number we provide to the `run_meaning_of_life`
    // function doesn't matter. We are controlling the result via the `StubLife`. We could create a
    // `FakeLife` which checks `n == 42`, but if the `is_meaning_of_life` changes from 42 to some
    // other number, then our tests will be incorrect.
    #[test]
    fn we_get_a_happy_result_if_we_guess_the_correct_meaning_of_life() {
        // Given that the correct meaning of life is provided
        let service = StubLife::default().always_correct();

        // When we check the meaning
        let result = run_meaning_of_life(service, 0);

        // Then we get a happy result
        assert_eq!(&result, "yay!");
    }

    #[test]
    fn we_get_a_sad_result_if_we_guess_the_wrong_meaning_of_life() {
        // Given that the wrong meaning of life is provided
        let service = StubLife::default().always_wrong();

        // When we check the meaning
        let result = run_meaning_of_life(service, 0);

        // Then we have a sad result
        assert_eq!(&result, ":frown:");
    }
}
