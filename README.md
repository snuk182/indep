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

Also, depending on the sync/async version of macros, you will need several usage definitions, either:
```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{Display,Formatter,Result};
```
or
```rust
use std::sync::Arc;
use std::sync::RwLock;
use std::fmt::{Display,Formatter,Result};
```
respectively,
and 
```rust
use your_mod::{Dependency,Dependent,Implementation};
```
in DI-enabled trait implementations, where your_mod is module in your project where the pool creation macro is applied.

## Dependencies

None except log, which is used only to `trace!` dependency injections.

## License

 * MIT
