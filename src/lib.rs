use serde::Deserialize;

pub fn is_valid<S: Into<String>>(stale: S, latest: S, ot_json: S) -> Result<(), ValidationError> {
    let stale = stale.into();
    let latest = latest.into();
    let operations: Vec<Operation> =
        serde_json::from_str(&ot_json.into()).map_err(|_| ValidationError::ParseError)?;

    let mut cursor = 0;
    let mut result = stale.clone();

    for operation in operations {
        dbg!(&operation);

        match operation {
            Operation::Skip { count } => {
                if cursor + count > result.len() {
                    return Err(ValidationError::SkipPastEnd);
                }
                cursor += count;
            }
            Operation::Delete { count } => {
                if cursor + count > result.len() {
                    return Err(ValidationError::DeletePastEnd);
                }
                result.replace_range(cursor..cursor + count, "");
            }
            Operation::Insert { chars } => {
                result.insert_str(cursor, &chars);
                cursor += chars.len();
            }
        }

        dbg!(&cursor);
        dbg!(&result);
    }

    if result != latest {
        return Err(ValidationError::DoesNotMatch);
    }

    Ok(())
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum Operation {
    Skip { count: usize },
    Delete { count: usize },
    Insert { chars: String },
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    ParseError,
    DoesNotMatch,
    SkipPastEnd,
    DeletePastEnd,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "Repl.it uses operational transformations.",
  r#"[{"op": "skip", "count": 40}, {"op": "delete", "count": 47}]"#
        );
        assert!(result.is_ok());
    }

    #[test]
    fn second() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "Repl.it uses operational transformations.",
  r#"[{"op": "skip", "count": 45}, {"op": "delete", "count": 47}]"#
        );
        assert_eq!(result.unwrap_err(), ValidationError::DeletePastEnd);
    }

    #[test]
    fn third() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "Repl.it uses operational transformations.",
  r#"[{"op": "skip", "count": 40}, {"op": "delete", "count": 47}, {"op": "skip", "count": 2}]"#
        );
        assert_eq!(result.unwrap_err(), ValidationError::SkipPastEnd);
    }

    #[test]
    fn fourth() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "We use operational transformations to keep everyone in a multiplayer repl in sync.",
  r#"[{"op": "delete", "count": 7}, {"op": "insert", "chars": "We"}, {"op": "skip", "count": 4}, {"op": "delete", "count": 1}]"#
        );
        assert!(result.is_ok());
    }

    #[test]
    fn fifth() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "We can use operational transformations to keep everyone in a multiplayer repl in sync.",
  r#"[{"op": "delete", "count": 7}, {"op": "insert", "chars": "We"}, {"op": "skip", "count": 4}, {"op": "delete", "count": 1}]"#
        );
        assert_eq!(result.unwrap_err(), ValidationError::DoesNotMatch);
    }

    #[test]
    fn sixth() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "[]"
        );
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error() {
        let result = is_valid(
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
  "[dd_1]"
        );
        assert_eq!(result.unwrap_err(), ValidationError::ParseError);
    }
}
