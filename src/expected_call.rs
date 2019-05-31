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
    pub fn answer(&mut self, f: impl FnMut<Args, Output = Output> + 'static) -> &mut Self {
        self.answer = Some(Box::new(f));
        self
    }

    pub fn times(&mut self, times: usize) -> &mut Self {
        self.times = times;
        self
    }
}
