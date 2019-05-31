# Mocktentacle

A mocking framework for mocking functions in Rust.  This utilizes [Mocktopus] as
the backend.

[Mocktopus](https://github.com/CodeSandwich/Mocktopus)

```rust
#[cfg(test)]
use mocktentacle::mockable;

#[cfg_attr(test, mockable)]
fn f() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use mocktentacle::Mock;

    #[test]
    fn test_mocktentacle() {
        let mut mock = Mock::new();
        mock.expect(f).answer(|| 1);

        assert_eq!(f(), 1);

        mock.verify();
    }
}
```
