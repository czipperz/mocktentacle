use crate::mock::{cast_call, MockInstance};
use mocktopus::mocking::MockResult;
use std::any::TypeId;
use std::marker::PhantomData;

pub(crate) struct CallGlue<'a, Args, Output> {
    mock: *mut MockInstance<'a>,
    id: TypeId,
    _phantom: PhantomData<(Args, Output)>,
}

impl<'a, Args, Output> CallGlue<'a, Args, Output> {
    pub(crate) fn new(mock: *mut MockInstance<'a>, id: TypeId) -> Self {
        CallGlue {
            mock,
            id,
            _phantom: PhantomData,
        }
    }
}

impl<'a, Args, Output> FnOnce<Args> for CallGlue<'a, Args, Output> {
    type Output = MockResult<Args, Output>;

    extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, Args, Output> FnMut<Args> for CallGlue<'a, Args, Output> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        for mock in unsafe { &mut *self.mock }.calls.get_mut(&self.id).unwrap() {
            let mock = cast_call(&mut **mock);
            if mock.can_invoke() {
                return MockResult::Return(mock.invoke(args));
            }
        }
        panic!("Error: All handlers have been exhausted");
    }
}
