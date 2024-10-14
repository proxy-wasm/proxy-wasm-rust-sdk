// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::RefCell;
use std::rc::Rc;

enum PromiseState<T> {
    Pending,
    Fulfilled(T),
    Rejected(String),
}

type ThenCallbackRef<T> = RefCell<Option<Box<dyn FnOnce(T)>>>;
type CatchCallbackRef = RefCell<Option<Box<dyn FnOnce(String)>>>;

pub struct Promise<T> {
    state: RefCell<PromiseState<T>>,
    then_callback: ThenCallbackRef<T>,
    catch_callback: CatchCallbackRef,
}

impl<T> Promise<T>
where
    T: 'static + Clone,
{
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            state: RefCell::new(PromiseState::Pending),
            then_callback: RefCell::new(None),
            catch_callback: RefCell::new(None),
        })
    }

    pub fn fulfill(self: &Rc<Self>, value: T) {
        *self.state.borrow_mut() = PromiseState::Fulfilled(value.clone());
        if let Some(callback) = self.then_callback.borrow_mut().take() {
            callback(value);
        }
    }

    pub fn reject(self: &Rc<Self>, reason: String) {
        *self.state.borrow_mut() = PromiseState::Rejected(reason.clone());
        if let Some(callback) = self.catch_callback.borrow_mut().take() {
            callback(reason);
        }
    }

    pub fn then<F, R>(self: &Rc<Self>, f: F) -> Rc<Promise<R>>
    where
        F: FnOnce(T) -> R + 'static,
        R: 'static + Clone,
    {
        let new_promise = Promise::new();
        let new_promise_clone = new_promise.clone();
        match &*self.state.borrow() {
            PromiseState::Pending => {
                *self.then_callback.borrow_mut() = Some(Box::new(move |value| {
                    let result = f(value);
                    new_promise_clone.fulfill(result);
                }));
                let new_promise_for_catch = new_promise.clone();
                *self.catch_callback.borrow_mut() = Some(Box::new(move |reason| {
                    new_promise_for_catch.reject(reason);
                }));
            }
            PromiseState::Fulfilled(value) => {
                let result = f(value.clone());
                new_promise.fulfill(result);
            }
            PromiseState::Rejected(reason) => new_promise.reject(reason.clone()),
        }
        new_promise
    }

    pub fn catch<F>(self: &Rc<Self>, f: F) -> Rc<Self>
    where
        F: FnOnce(String) + 'static,
    {
        match &*self.state.borrow() {
            PromiseState::Pending => *self.catch_callback.borrow_mut() = Some(Box::new(f)),
            PromiseState::Fulfilled(_) => {}
            PromiseState::Rejected(reason) => f(reason.clone()),
        }
        self.clone()
    }

    pub fn all_of(promises: Vec<Rc<Self>>) -> Rc<Promise<Vec<T>>> {
        let next_promise = Promise::new();

        if promises.is_empty() {
            next_promise.fulfill(vec![]);
            return next_promise;
        }

        let total = promises.len();
        let results = Rc::new(RefCell::new(vec![None; total]));
        let remaining = Rc::new(RefCell::new(total));
        let rejected = Rc::new(RefCell::new(false));

        for (i, promise) in promises.iter().enumerate() {
            let next_promise_clone = next_promise.clone();
            let next_promise_clone_for_catch = next_promise.clone();
            let results_clone = results.clone();
            let remaining_clone = remaining.clone();
            let rejected_clone = rejected.clone();
            let rejected_clone_for_catch = rejected.clone();
            promise
                .then(move |result| {
                    if *rejected_clone.borrow() {
                        return;
                    }
                    results_clone.borrow_mut()[i] = Some(result);
                    *remaining_clone.borrow_mut() -= 1;

                    if *remaining_clone.borrow() == 0 {
                        let final_results: Vec<T> = results_clone
                            .borrow_mut()
                            .iter_mut()
                            .map(|res| res.take().unwrap())
                            .collect();
                        next_promise_clone.fulfill(final_results);
                    }
                })
                .catch(move |reason| {
                    if !*rejected_clone_for_catch.borrow() {
                        *rejected_clone_for_catch.borrow_mut() = true;
                        next_promise_clone_for_catch.reject(reason);
                    }
                });
        }
        next_promise
    }

    pub fn any_of(promises: Vec<Rc<Self>>) -> Rc<Promise<T>> {
        let next_promise = Promise::new();
        let total = promises.len();
        let remaining = Rc::new(RefCell::new(total));
        let first_error = Rc::new(RefCell::new(None));

        for promise in promises {
            let next_promise_clone = next_promise.clone();
            let next_promise_clone_for_catch = next_promise.clone();
            let remaining_clone = remaining.clone();
            let remaining_clone_for_catch = remaining.clone();
            let first_error_clone = first_error.clone();

            promise
                .then(move |result| {
                    if *remaining_clone.borrow() > 0 {
                        next_promise_clone.fulfill(result);
                        *remaining_clone.borrow_mut() = 0;
                    }
                })
                .catch(move |err| {
                    if first_error_clone.borrow().is_none() {
                        *first_error_clone.borrow_mut() = Some(err);
                    }

                    *remaining_clone_for_catch.borrow_mut() -= 1;

                    if *remaining_clone_for_catch.borrow() == 0 {
                        if let Some(first_err) = first_error_clone.borrow().clone() {
                            next_promise_clone_for_catch.reject(first_err);
                        }
                    }
                });
        }

        next_promise
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_promise_new() {
        let promise = Promise::<i32>::new();
        assert!(matches!(*promise.state.borrow(), PromiseState::Pending));
        assert!(promise.then_callback.borrow().is_none());
        assert!(promise.catch_callback.borrow().is_none());
    }

    #[test]
    fn test_promise_fulfill() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let promise = Promise::<i32>::new();
        let _next_promise = promise.then(move |result| {
            assert_eq!(result, 42);
            *touched_clone.borrow_mut() = true;
        });

        promise.fulfill(42);
        assert!(touched.take())
    }

    #[test]
    fn test_promise_reject() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let promise = Promise::<String>::new();
        let _next_promise = promise.catch(move |err| {
            assert_eq!(err, "Error");
            *touched_clone.borrow_mut() = true;
        });

        promise.reject("Error".to_string());
        assert!(touched.take())
    }

    #[test]
    fn test_promise_chain() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let promise = Promise::<i32>::new();
        let next_promise = promise.then(|result| {
            assert_eq!(result, 10);
            20
        });

        next_promise.then(move |result| {
            assert_eq!(result, 20);
            *touched_clone.borrow_mut() = true;
        });

        promise.fulfill(10);
        assert!(touched.take())
    }

    #[test]
    fn test_all_of_success() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let promise1 = Promise::<i32>::new();
        let promise2 = Promise::<i32>::new();

        let all_promise = Promise::all_of(vec![promise1.clone(), promise2.clone()]);

        promise1.fulfill(42);
        promise2.fulfill(100);

        all_promise
            .then(move |results| {
                assert_eq!(results.len(), 2);
                assert_eq!(results[0], 42);
                assert_eq!(results[1], 100);
                *touched_clone.borrow_mut() = true;
            })
            .catch(|_err| {
                panic!("Should not reach here");
            });

        assert!(touched.take())
    }

    #[test]
    fn test_all_of_failure() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let promise1 = Promise::<i32>::new();
        let promise2 = Promise::<i32>::new();

        let all_promise = Promise::all_of(vec![promise1.clone(), promise2.clone()]);

        promise1.reject("Error 1".to_string());
        promise2.reject("Error 2".to_string());

        all_promise
            .then(|_results| {
                panic!("Should not reach here");
            })
            .catch(move |err| {
                assert_eq!(err, "Error 1");
                *touched_clone.borrow_mut() = true;
            });

        assert!(touched.take())
    }

    #[test]
    fn test_all_of_mixed_results() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let promise1 = Promise::<i32>::new();
        let promise2 = Promise::<i32>::new();

        let all_promise = Promise::all_of(vec![promise1.clone(), promise2.clone()]);

        promise1.reject("Error".to_string());
        promise2.fulfill(100);

        all_promise
            .then(|_| {
                panic!("Should not reach here");
            })
            .catch(move |reason| {
                assert_eq!(reason, "Error".to_string());
                *touched_clone.borrow_mut() = true;
            });

        assert!(touched.take())
    }

    #[test]
    fn test_all_of_empty() {
        let touched = Rc::new(RefCell::new(false));
        let touched_clone = touched.clone();

        let all_promise = Promise::<i32>::all_of(vec![]);

        all_promise
            .then(move |results| {
                assert!(results.is_empty());
                *touched_clone.borrow_mut() = true;
            })
            .catch(|_err| {
                panic!("Should not reach here");
            });

        assert!(touched.take())
    }
}
