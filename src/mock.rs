use crate::call_glue::CallGlue;
use crate::expected_call::ExpectedCall;
use crate::verify::Verify;
use mocktopus::mocking::{Mockable, ScopedMock};
use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomPinned;

pub struct Mock<'a> {
    mocks: HashMap<TypeId, ScopedMock<'a>>,
    pub(crate) calls: HashMap<TypeId, Vec<Box<Verify>>>,
    _pin: PhantomPinned,
}

impl<'a> Mock<'a> {
    pub fn new() -> Self {
        Mock {
            mocks: Default::default(),
            calls: Default::default(),
            _pin: PhantomPinned,
        }
    }

    pub fn expect<Args: 'static, Output: 'static>(
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
        Mock::cast_call(&mut *calls[last])
    }

    pub(crate) fn cast_call<Args, Output>(call: &mut Verify) -> &mut ExpectedCall<Args, Output> {
        unsafe { &mut *((call as *mut Verify) as *mut ExpectedCall<_, _>) }
    }

    pub fn verify(&mut self) {
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

impl<'a> Drop for Mock<'a> {
    fn drop(&mut self) {
        if !std::thread::panicking() && (!self.mocks.is_empty() || !self.calls.is_empty()) {
            panic!("Forgot to call Mock::verify()");
        }
    }
}
