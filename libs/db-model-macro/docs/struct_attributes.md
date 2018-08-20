# Struct Attributes

The struct attributes affect only the struct and are introduced before it:

```rust
#[attribute]
struct Name {
// ...
}
```

## Attributes

- `#[db_attr(...)]`: adds an attribute only to the database model.
- `#[api_attr(...)]`: adds an attribute only to the api model.
- `#[skip_default]`: disables the generation of the default derives.