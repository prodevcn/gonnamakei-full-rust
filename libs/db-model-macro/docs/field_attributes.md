# Field Attributes

The field attributes affect only the field itself and are introduced before it:

```rust
struct Name {
    #[attribute]
    field: Type
}
```

## Attributes

- `#[db_attr(...)]`: adds an attribute only to the database field.
- `#[api_attr(...)]`: adds an attribute only to the api field.
- `#[skip_in_db]`: does not include this field into the database model.
- `#[skip_in_api]`: does not include this field into the api model.
- `#[skip_normalize]`: changes how this field is included in the normalize method.
- `#[db_name = ".."]`: renames this field using serde in the database model.
- `#[inner_model = "<value>"]`: specifies the inner model. Values are:
    - `data`: Not a model.
    - `struct`: A struct-like model.
    - `enum`: An enum-like model.
- `#[api_inner_type = ".."]`: specifies the name of the inner type for the api model. This is used when the sub-model
  changes from DB to API.
- `#[api_sensible_info]`: includes this field into the `remove_sensible_info` method for the api model.
- `#[api_skip_map_to_null]`: excludes this field from the method `map_values_to_null`.