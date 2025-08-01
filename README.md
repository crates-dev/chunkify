<center>

## chunkify

[![](https://img.shields.io/crates/v/chunkify.svg)](https://crates.io/crates/chunkify)
[![](https://img.shields.io/crates/d/chunkify.svg)](https://img.shields.io/crates/d/chunkify.svg)
[![](https://docs.rs/chunkify/badge.svg)](https://docs.rs/chunkify)
[![](https://github.com/eastspire/chunkify/workflows/Rust/badge.svg)](https://github.com/eastspire/chunkify/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/chunkify.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/chunkify/)

[Api Docs](https://docs.rs/chunkify/latest/chunkify/)

> A simple and efficient chunking library for Rust.

## Installation

To use this crate, you can run cmd:

```shell
cargo add chunkify
```

## Use

```rust
use chunkify::*;

let chunk_strategy: ChunkStrategy<'_> = ChunkStrategy::new(
    0,
    "./uploads",
    "abcdefg",
    "test.txt",
    1,
    |file_id: &str, chunk_index: usize| format!("{file_id}.{chunk_index}"),
)
.unwrap();
chunk_strategy.save_chunk(b"test", 0).await.unwrap();
chunk_strategy.merge_chunks().await.unwrap();
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
