use mocktentacle::{mockable, Mock};

#[mockable]
fn f() -> i32 {
    0
}

mod drop {
    use super::*;

    #[test]
    #[should_panic]
    fn when_no_verify_should_panic() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1);
        f();
    }
}
