# indep
Dead simple Dependency Injection library in Rust.

## Documentation

The library contains support for single- and multi-threaded environments. See examples/sync.rs and examples/async.rs respectively, everything is described there.

Put the following in your `Cargo.toml`:

```toml
[dependencies]
indep = "*"
```

And the following - to the crate root:

```rust
#[macro_use]
extern crate indep;
#[macro_use]
extern crate log;
```

Also you will need several usage definitions:
```rust
use your_mod::{Dependency,Dependent,Implementation};
```
in DI-enabled trait implementations, where your_mod is module in your project where the pool creation macro is applied.

## Dependencies

None except `log`, which is used only to `trace!` dependency injections.

## License

 * MIT OR Apache-2.0
