use std::collections::HashMap;
use std::io::Write;

use serde::Serialize;

use crate::database::AqlBuilder;

pub trait AQLMapping {
    /// Includes the necessary let steps into the aql.
    #[allow(unused_variables)]
    fn include_let_steps(&self, aql: &mut AqlBuilder, path: &str, next_id: &mut usize) {}

    /// Maps this value into a JSON string.
    fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize);
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl<T: AQLMapping> AQLMapping for Vec<T> {
    fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize) {
        buffer.write_all(b"[").unwrap();

        for (i, v) in self.iter().enumerate() {
            let sub_path = format!("{}[{}]", path, i);
            v.map_to_json(buffer, sub_path.as_str(), next_id);
            buffer.write_all(b",").unwrap();
        }

        buffer.write_all(b"]").unwrap();
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

impl<K: Serialize, T: AQLMapping> AQLMapping for HashMap<K, T> {
    fn map_to_json(&self, buffer: &mut Vec<u8>, path: &str, next_id: &mut usize) {
        buffer.write_all(b"{").unwrap();

        for (k, v) in self.iter() {
            let sub_path = format!("{}.{}", path, serde_json::to_string(k).unwrap());
            v.map_to_json(buffer, sub_path.as_str(), next_id);
            buffer.write_all(b",").unwrap();
        }

        buffer.write_all(b"}").unwrap();
    }
}
