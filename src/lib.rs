use serde::Deserialize;

pub fn is_valid<S: Into<String>>(stale: S, latest: S, ot_json: S) -> Result<(), ValidationError> {
    let stale = stale.into();
    let latest = latest.into();
    let operations: Vec<Operation> =
        serde_json::from_str(&ot_json.into()).map_err(|_| ValidationError::ParseError)?;

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    SkipPastEnd,
    DeletePastEnd,
    ParseError,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum Operation {
    Skip { count: usize },
    Delete { count: usize },
    Insert { chars: String },
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
        assert_eq!(result.unwrap_err(), ValidationError::SkipPastEnd);
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