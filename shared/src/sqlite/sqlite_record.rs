use crate::AppError;

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

/// One result record of a query, read by column position.
#[derive(Debug, Clone, PartialEq)]
pub struct SqlRecord {
    values: Vec<SqlValue>,
}

impl SqlRecord {
    pub fn new(values: Vec<SqlValue>) -> SqlRecord {
        SqlRecord { values }
    }

    pub fn text(&self, column_index: usize) -> Result<String, AppError> {
        match self.value(column_index)? {
            SqlValue::Text(text) => Ok(text.clone()),
            other => Err(unexpected_value(column_index, "text", other)),
        }
    }

    pub fn optional_text(&self, column_index: usize) -> Result<Option<String>, AppError> {
        match self.value(column_index)? {
            SqlValue::Null => Ok(None),
            SqlValue::Text(text) => Ok(Some(text.clone())),
            other => Err(unexpected_value(column_index, "text or null", other)),
        }
    }

    pub fn integer(&self, column_index: usize) -> Result<i64, AppError> {
        match self.value(column_index)? {
            SqlValue::Integer(integer) => Ok(*integer),
            other => Err(unexpected_value(column_index, "integer", other)),
        }
    }

    pub fn optional_integer(&self, column_index: usize) -> Result<Option<i64>, AppError> {
        match self.value(column_index)? {
            SqlValue::Null => Ok(None),
            SqlValue::Integer(integer) => Ok(Some(*integer)),
            other => Err(unexpected_value(column_index, "integer or null", other)),
        }
    }

    pub fn position(&self, column_index: usize) -> Result<usize, AppError> {
        let integer: i64 = self.integer(column_index)?;

        usize::try_from(integer).map_err(|_| {
            AppError::from(format!("sqlite record: negative position; [column={column_index} value={integer}]"))
        })
    }

    fn value(&self, column_index: usize) -> Result<&SqlValue, AppError> {
        self.values.get(column_index).ok_or_else(|| {
            AppError::from(format!(
                "sqlite record: no such column; [column={column_index} column_count={}]",
                self.values.len(),
            ))
        })
    }
}

fn unexpected_value(column_index: usize, expected: &str, found: &SqlValue) -> AppError {
    AppError::from(format!("sqlite record: unexpected value; [column={column_index} expected={expected} found={found:?}]"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_reads_a_text_column_and_rejects_other_types() {
        let record: SqlRecord = SqlRecord::new(vec![SqlValue::Text("oi".to_string()), SqlValue::Integer(3)]);

        assert_eq!(record.text(0).unwrap(), "oi");
        assert!(record.text(1).is_err());
        assert!(record.text(2).is_err());
    }

    #[test]
    fn optional_text_maps_null_to_none() {
        let record: SqlRecord = SqlRecord::new(vec![SqlValue::Null, SqlValue::Text("a".to_string())]);

        assert_eq!(record.optional_text(0).unwrap(), None);
        assert_eq!(record.optional_text(1).unwrap(), Some("a".to_string()));
    }

    #[test]
    fn position_rejects_a_negative_integer() {
        let record: SqlRecord = SqlRecord::new(vec![SqlValue::Integer(-1), SqlValue::Integer(4)]);

        assert!(record.position(0).is_err());
        assert_eq!(record.position(1).unwrap(), 4);
    }
}
