# Prefixed TSIDs

[Documentation](https://docs.rs/prefixed-tsid/latest/prefixed_tsid/index.html)

This library allows you to easily create and manage Stripe-style prefixed IDs for your database, using the [`tsid`](https://github.com/jakudlaty/tsid/) crate. For example:

```
org_0GTM2MERJJE1T
```

Each type of resource gets its own unique prefix, and is represented as a type-safe ID in your code. You can declare resources manually or using our handy macro:

```rust
id_resource_type!(IDOrganisation, "org");

// equals:

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct IDOrganisation;
impl TSIDResource for IDOrganisation {
    fn prefix() -> Option<String> {
        Some("org".to_owned())
    }
}
```

Then, you can easily create a random instance of the ID:

```rust
let org_id = TSIDDatabaseID::<IDOrganisation>::random();
```

When stringified (or serialised using the `serde` feature), it will always include the prefix in the `TSIDResource` type.

When stored in your database, it's simply a `u64` without the prefix information, since your database presumably keeps different resources in different tables anyway. The library comes with a built-in `sea-orm` integration.

## Features

By default, only `serde` is enabled.

- `serde`: serialize/deserialize integration. Adds the correct prefix when serializing, and returns an error if the prefix is missing or is wrong during deserialization.
- `rocket`: integration for `FromParam` and `FromFormField` so you can use a `TSIDDatabaseID` in path components, query parameters, form requests, etc.
- `sea-orm`: allows you to use `TSIDDatabaseID` in your entity structs, so the ID gets turned into a `u64` and vice versa.
- `schemars`: allows generating JSON Schema containing a RegEx to match your prefixed ID.
- `ts-rs`: for extra-nice WASM compatibility, allows generating a string type to represent the ID type in Typescript.

## License

Made by Palform Ltd (registered company 15796859 in the UK) under the **MIT License**.
