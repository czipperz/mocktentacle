use crate::expected_call::ExpectedCall;

pub(crate) trait Validate {
    fn validate(&self) -> bool;
}

impl<Args, Output> Validate for ExpectedCall<Args, Output> {
    fn validate(&self) -> bool {
        if self.num < self.times {
            eprintln!(
                "Error: Too few invocations: expected {:?}, performed {:?}",
                self.times, self.num
            );
            false
        } else if self.num > self.times {
            eprintln!(
                "Error: Too many invocations: expected {:?}, performed {:?}",
                self.times, self.num
            );
            false
        } else {
            true
        }
    }
}
