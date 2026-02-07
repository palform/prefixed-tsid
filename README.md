# Prefixed TSIDs

[Documentation](https://docs.rs/prefixed-tsid/latest/prefixed_tsid/index.html)

This library allows you to easily create and manage resource-specific prefixed IDs for your database, using the [`tsid`](https://github.com/jakudlaty/tsid/) crate. See [this article](https://www.foxhound.systems/blog/time-sorted-unique-identifiers/) for an explanation of why TSIDs are cool. For example:

```ignore
org_0GTM2MERJJE1T
```

Each type of resource gets its own unique prefix, and is represented as a type-safe ID in your code. You can declare resources manually or using our handy macro:

```rust,ignore
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

```rust,ignore
let org_id = TSIDDatabaseID::<IDOrganisation>::random();
```

When stringified (or serialised using the `serde` feature), it will always include the prefix in the `TSIDResource` type.

When stored in your database, it's simply a `u64` without the prefix information, since your database presumably keeps different resources in different tables anyway. The library comes with a built-in `sea-orm` integration.

## Features

By default, only `serde` is enabled. You can enable additional features to support third-party libraries.

- `serde`: serialize/deserialize integration. Adds the correct prefix when serializing, and returns an error if the prefix is missing or is wrong during deserialization.
- `rocket`: integration for `FromParam` and `FromFormField` so you can use a `TSIDDatabaseID` in path components, query parameters, form requests, etc.
- `sea-orm`: allows you to use `TSIDDatabaseID` in your entity structs, so the ID gets turned into a `u64` and vice versa.
- `schemars`: allows generating JSON Schema containing a RegEx to match your prefixed ID.
- `ts-rs`: for extra-nice WASM compatibility, allows generating a string type to represent the ID type in Typescript.
- `diesel`: allows you to use `TSIDDatabaseID` in your models, so you can store the ID in your database.

## Roadmap

- Unit tests for sensitive bits, such as the prefix parsing
- Better support and documentation for `IDUnknown` to allow working with non-resource-specific IDs in niche cases where it's needed.

## Contributing

Contributions are super welcome, especially towards any roadmap items. Please open a PR with a short explanation of your additions, and we'll review them ASAP.

## License

Made by Palform Ltd (registered company 15796859 in the UK) under the **MIT License**.
