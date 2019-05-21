# lazy-pipe

Chain closures into a pipeline

## Usage

```rust
let expected = ((3 + 1) / 2).to_string();

let received = pipe(3)
    .map(|x| x + 1)
    .map(|x| x / 2)
    .map(|x| x.to_string())
    .unwrap();

assert_eq!(received, expected);
```

[Documentation](https://docs.rs/lazy-pipe/)

## License

[MIT](https://git.io/fj43Z) © [Hoàng Văn Khải](https://github.com/KSXGitHub)
