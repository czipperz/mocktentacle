use mocktentacle::{mockable, Mock};

#[mockable]
fn f() -> i32 {
    0
}

mod times {
    use super::*;

    #[test]
    #[should_panic]
    fn expect_custom_times_too_few() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1).times(2);
        assert_eq!(f(), 1);
        mock.validate();
    }

    #[test]
    fn expect_custom_times_just_right() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1).times(2);
        assert_eq!(f(), 1);
        assert_eq!(f(), 1);
        mock.validate();
    }

    #[test]
    #[should_panic]
    fn expect_custom_times_too_many() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1).times(2);
        assert_eq!(f(), 1);
        assert_eq!(f(), 1);
        assert_eq!(f(), 1);
        mock.validate();
    }
}

mod expect {
    use super::*;

    #[test]
    #[should_panic]
    fn when_expect_with_no_answer_then_calling_should_panic() {
        let mut mock = Mock::new();
        mock.expect(f);
        f();
    }

    #[test]
    #[should_panic]
    fn when_mocked_function_called_twice_should_panic() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1);
        f();
        f();
        mock.validate();
    }

    #[test]
    fn when_mock_is_called_once_should_call_answer() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1);
        assert_eq!(f(), 1);
        mock.validate();
    }

    #[test]
    fn when_expect_two_calls_and_call_twice_then_call_answers_in_order() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1);
        mock.expect(f).answer(|| 2);
        assert_eq!(f(), 1);
        assert_eq!(f(), 2);
        mock.validate();
    }

    #[test]
    fn multiple_expects_interaction_with_times() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1).times(2);
        mock.expect(f).answer(|| 2).times(0);
        assert_eq!(f(), 1);
        assert_eq!(f(), 1);
        mock.validate();
    }

    #[test]
    fn skip_expect_zero_times() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1).times(0);
        mock.expect(f).answer(|| 2).times(2);
        assert_eq!(f(), 2);
        assert_eq!(f(), 2);
        mock.validate();
    }
}
