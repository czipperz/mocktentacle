use crate::call_glue::CallGlue;
use crate::expected_call::ExpectedCall;
use crate::verify::Verify;
use mocktopus::mocking::{Mockable, ScopedMock};
use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomPinned;
use std::pin::Pin;

/// The instance of the mocktentacle library.
///
/// [`expect`] allows you to control mocks of specific functions.  [`verify`]
/// ensures that the mocks were called the correct number of times.
///
/// [`expect`]: struct.Mock.html#method.expect
/// [`verify`]: struct.Mock.html#method.verify
pub struct Mock<'a>(Pin<Box<MockInstance<'a>>>);

/// The `calls` field is implementing by pointing to the `MockInstance`.  Thus
/// we can wrap the underlying instance in a pinned box.
pub(crate) struct MockInstance<'a> {
    mocks: HashMap<TypeId, ScopedMock<'a>>,
    pub(crate) calls: HashMap<TypeId, Vec<Box<Verify>>>,
    _pin: PhantomPinned,
}

impl<'a> Mock<'a> {
    /// Create a `Mock` instance.
    pub fn new() -> Self {
        Mock(Box::pin(MockInstance {
            mocks: Default::default(),
            calls: Default::default(),
            _pin: PhantomPinned,
        }))
    }

    /// Expects the function to be called.
    ///
    /// This returns an [`ExpectedCall`] instance where you can specify more
    /// information about this instance of the call.  For instance, what should
    /// happen when it is called and how many times should it be called.
    ///
    /// Multiple invocations of `expect` are encouraged!  They are sequenced in
    /// order of `expect` calls.  They are each invoked at most [`times`] times.
    ///
    /// This does bad things if the function is not annotated with
    /// `#[mockable]`.  This will be detected by [`verify`] as it will always
    /// panic.
    ///
    /// [`ExpectedCall`]: struct.ExpectedCall.html
    /// [`times`]: struct.ExpectedCall.html#method.times
    /// [`verify`]: struct.Mock.html#method.verify
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mocktentacle::{mockable, Mock};
    ///
    /// #[mockable]
    /// fn f() -> i32 { 0 }
    ///
    /// # /*
    /// #[test]
    /// fn test_expect_multiple_times() {
    /// # */
    ///     let mut mock = Mock::new();
    ///     mock.expect(f).answer(|| 1).times(2);
    ///     mock.expect(f).answer(|| 2);
    ///     assert_eq!(f(), 1);
    ///     assert_eq!(f(), 1);
    ///     assert_eq!(f(), 2);
    ///     mock.verify();
    /// # /*
    /// }
    /// # */
    /// ```
    pub fn expect<Args: 'static, Output: 'static>(
        &mut self,
        f: impl Mockable<Args, Output>,
    ) -> &mut ExpectedCall<Args, Output> {
        self.instance().expect(f)
    }

    /// Ensure that all mocks were called the correct number of times.
    ///
    /// This also releases all mocks acquired with [`expect`], making the
    /// methods return to their normal behavior.
    ///
    /// The `Mock` must be verified if [`expect`] is ever invoked.  Otherwise, the
    /// `Drop` implementation will panic.
    ///
    /// [`expect`]: #method.expect
    ///
    /// # Panics
    ///
    /// This will panic if any mock implementations was called an incorrect
    /// number of times.
    pub fn verify(&mut self) {
        self.instance().verify()
    }

    fn instance(&mut self) -> &mut MockInstance<'a> {
        unsafe { Pin::get_unchecked_mut(Pin::as_mut(&mut self.0)) }
    }
}

pub(crate) fn cast_call<Args, Output>(call: &mut Verify) -> &mut ExpectedCall<Args, Output> {
    unsafe { &mut *((call as *mut Verify) as *mut ExpectedCall<_, _>) }
}

impl<'a> MockInstance<'a> {
    pub(crate) fn expect<Args: 'static, Output: 'static>(
        &mut self,
        f: impl Mockable<Args, Output>,
    ) -> &mut ExpectedCall<Args, Output> {
        let id = unsafe { f.get_mock_id() };

        let this = self as *mut Self;
        self.mocks
            .entry(id)
            .or_insert_with(|| unsafe { f.mock_scoped(CallGlue::new(this, id)) });

        let calls = self.calls.entry(id).or_insert_with(Default::default);
        let last = calls.len();
        calls.push(Box::new(ExpectedCall::<Args, Output>::new()));
        cast_call(&mut *calls[last])
    }

    pub(crate) fn verify(&mut self) {
        self.mocks.clear();
        let calls = std::mem::replace(&mut self.calls, Default::default());

        let mut any_failed = false;
        for entry in calls {
            any_failed = any_failed || !entry.1.into_iter().all(|call| call.verify())
        }

        if any_failed {
            panic!("Failed to verify Mock");
        }
    }
}

/// If [`verify`] was not called, this will panic.
///
/// [`verify`]: #method.verify
impl<'a> Drop for Mock<'a> {
    fn drop(&mut self) {
        if !std::thread::panicking() && (!self.0.mocks.is_empty() || !self.0.calls.is_empty()) {
            panic!("Forgot to call Mock::verify()");
        }
    }
}
