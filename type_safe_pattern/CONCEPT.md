🦀 Rust TIL: Type Safe Pattern

The Type-State Pattern uses the static type system and affine types (ownership and move semantics) to encode an objects runtime state into its compile-time type.

This makes invalid states or out-of-order method invocations unrepresentable at compile time. In standard builder patterns, methods often take `mut self` and return `mut Self`. However, when state transitions alter generic parameters (e.g., from `RequestBuilder<NoUrl>` to `RequestBuilder<HasUrl>`):

a. Ownership must be consumed (`self`): Taking self by value consumes/moves the builder from its previous state, preventing the caller from continuing to use the old builder instance in its previous state.

b. Return the new state type (`RequestBuilder<HasUrl>`): Because Self in an `impl RequestBuilder<NoUrl>` block is an alias for `RequestBuilder<NoUrl>`, the method must explicitly declare the return type with the new state parameter: `RequestBuilder<HasUrl>`