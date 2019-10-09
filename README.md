# is-up?

## Or, "baby's first Rust project."

The purpose of this project is exclusively as an accelerated first dive into Rust.

## Areas of Review

General structure. From the file system, to the [src/commands/](commands/) directory, and bigger picture decisions like the daemon being separate binary (see: [src/daemon.rs](daemon.rs) and [Cargo.toml](Cargo.toml)).

### Idiomatic Rust

- In [src/is_up.rs](is_up.rs), I've made a few decisions I'm not proud of with respect to composability. A similar pattern in [src/configuration.rs](configuration.rs).
- In [src/is_up.rs](is_up.rs), the `handler` function exists solely to turn a complex error type into a string. I don't love the pattern.
- Some challenges with the use of enums/abstract types. Looking for broad feedback on how to do that better.
- Identify some sane rules-of-thumb for where to use `.expect`, `?` and `match`.
- Appropriate file structure. Looking for idiomatic feedback/great projects to learn from here.
- Traits and implementations on objects. I'm not yet sure how to balance the object-oriented approach these features offer with the functional approach I generally prefer.

### Language Features

- Use of closures. In [src/main.rs](main.rs), I had initially used a closure to capture the config file, but it didn't feel very Rusty.
- Use of scoping rules like lifetimes. I have not wrapped my mind around lifetimes at all yet.

### Testing Culture

I've run into some challenges with writing adequate tests in Rust.

- Culturally, I am unsure if we should prefer carefully structured mocks or a full dependency injection style.
- Mocking the HTTP requests proved to be a minor challenge.
- No clear way to generate coverage reports.
- Testing libraries yet to be explored. How to broadly do nice assertions?

### Other

- Use of the nightly build appears to be very common in Rust to enable certain compiler features (e.g., Mocktopus required it).
- I'm finding a lot of loss of encapsulation problems. For example, [src/main.rs](main.rs) has the `extern` statements and the macros that are only used in other files.
