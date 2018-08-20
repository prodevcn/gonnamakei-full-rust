# Macro Options

The macro options are introduced at the beginning using the pattern:

```rust
#![attribute]
```

## Options

- `#![build_api]`: enables the api mapping that mainly removes the serde renames.
- `#![skip_impl]`: disables the generation of the database impls.
- `#![skip_fields]`: disables the generation of the database field enum for the model.
- `#![sync_level = "<level>"]`: enables the synchronization of the model or the collection. The values are:
    - `document`: only synchronizes at document level.
    - `collection`: only synchronizes at collection level.
    - `document_and_collection`: synchronizes at both document and collection levels.
- `#![sync_collection_key_method = ".."]`: replaces the default name for the method used to get the key of the document
  that represents the collection in DB.
- `#![collection_name = ".."]`: replaces the default name for the collection.
- `#![collection_kind = ".."]`: replaces the default name for `CollectionKind` enum.
- `#![replace_normalize = ".."]`: replaces the normalize method by the one specified.
- `#![replace_normalize_fields = ".."]`: replaces the normalize_fields method by the one specified.
- `#![replace_filter = ".."]`: replaces the filter method by the one specified.
- `#![api_name = ".."]`: replaces the default name for the api model.
- `#![api_paginated]`: enables the pagination of the api model. This options requires to have also
  set `#![api_min_rows_per_page = ".."]`, `#![api_max_rows_per_page = ".."]` and the `is_valid_for_sorting`
  and `is_valid_for_filtering` methods.
- `#![api_min_rows_per_page = ".."]`: the minimum rows available for the api model pages.
- `#![api_max_rows_per_page = ".."]`: the maximum rows available for the api model pages.
