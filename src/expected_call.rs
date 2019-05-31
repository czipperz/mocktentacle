/// Specify the details of how to mock a function.
pub struct ExpectedCall<Args, Output> {
    answer: Option<Box<FnMut<Args, Output = Output>>>,
    pub(crate) times: usize,
    pub(crate) num: usize,
}

impl<Args, Output> ExpectedCall<Args, Output> {
    pub(crate) fn new() -> Self {
        ExpectedCall {
            answer: None,
            times: 1,
            num: 0,
        }
    }

    pub(crate) fn can_invoke(&self) -> bool {
        self.num < self.times
    }

    pub(crate) fn invoke(&mut self, args: Args) -> Output {
        self.num += 1;
        self.answer.as_mut().unwrap().call_mut(args)
    }
}

impl<Args, Output> ExpectedCall<Args, Output> {
    /// Specify what happens when the mock is executed.
    ///
    /// This function will replace the default implementation of the mock.  If
    /// it is not specified, any call of the mocked function will panic.
    pub fn answer(&mut self, f: impl FnMut<Args, Output = Output> + 'static) -> &mut Self {
        self.answer = Some(Box::new(f));
        self
    }

    /// Specify the number of times this mock should be called.
    ///
    /// [`Mock::verify`] will ensure the minimum boundary.  The maximum boundary
    /// is automatically enforced.
    ///
    /// This defaults to 1.
    ///
    /// [`Mock::verify`]: struct.Mock.html#method.verify
    ///
    /// # Examples
    ///
    /// Panics if called too few times:
    ///
    /// ```should_panic
    /// use mocktentacle::{mockable, Mock};
    ///
    /// #[mockable]
    /// fn f() -> i32 { 0 }
    ///
    /// # /*
    /// #[test]
    /// #[should_panic]
    /// fn test_expect_multiple_times() {
    /// # */
    ///     let mut mock = Mock::new();
    ///     mock.expect(f).answer(|| 1).times(2);
    ///     assert_eq!(f(), 1);
    ///     mock.verify();
    /// # /*
    /// }
    /// # */
    /// ```
    ///
    /// Panics if called too many times:
    ///
    /// ```should_panic
    /// use mocktentacle::{mockable, Mock};
    ///
    /// #[mockable]
    /// fn f() -> i32 { 0 }
    ///
    /// # /*
    /// #[test]
    /// #[should_panic]
    /// fn test_expect_multiple_times() {
    /// # */
    ///     let mut mock = Mock::new();
    ///     mock.expect(f).answer(|| 1).times(2);
    ///     assert_eq!(f(), 1);
    ///     assert_eq!(f(), 1);
    ///     assert_eq!(f(), 1);
    ///     mock.verify();
    /// # /*
    /// }
    /// # */
    /// ```
    pub fn times(&mut self, times: usize) -> &mut Self {
        self.times = times;
        self
    }
}
