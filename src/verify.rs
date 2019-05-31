use crate::expected_call::ExpectedCall;

pub(crate) trait Verify {
    fn verify(&self) -> bool;
}

impl<Args, Output> Verify for ExpectedCall<Args, Output> {
    fn verify(&self) -> bool {
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
