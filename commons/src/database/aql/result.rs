use arangors::Cursor;
use serde::Deserialize;

pub struct AqlResult<T: for<'de> Deserialize<'de>> {
    pub count: u64,
    pub results: Vec<T>,
    pub writes_executed: u64,
    pub writes_ignored: u64,
    pub scanned_full: u64,
    pub scanned_index: u64,
    pub filtered: u64,
    pub full_count: Option<u64>,
    pub http_requests: u64,
    pub execution_time: f64,
}

impl<T: for<'de> Deserialize<'de>> From<Cursor<T>> for AqlResult<T> {
    fn from(cursor: Cursor<T>) -> Self {
        let mut result = AqlResult {
            count: cursor.result.len() as u64,
            results: cursor.result,
            writes_executed: 0,
            writes_ignored: 0,
            scanned_full: 0,
            scanned_index: 0,
            filtered: 0,
            full_count: None,
            http_requests: 0,
            execution_time: 0.0,
        };

        if let Some(extra) = cursor.extra {
            if let Some(stats) = extra.stats {
                result.writes_executed = stats.writes_executed as u64;
                result.writes_ignored = stats.writes_ignored as u64;
                result.scanned_full = stats.scanned_full as u64;
                result.scanned_index = stats.scanned_index as u64;
                result.filtered = stats.filtered as u64;

                if let Some(full_count) = stats.full_count {
                    result.full_count = Some(full_count as u64);
                }

                result.http_requests = stats.http_requests as u64;
                result.execution_time = stats.execution_time;
            }
        }

        result
    }
}
