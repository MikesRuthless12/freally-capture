//! CAP-M16 — **Text from file**: the pure extraction a bound Text source
//! runs over its watched file's content. Three bindings: the whole file,
//! one CSV cell (quote-aware, header-or-index column), or one JSON value
//! (RFC 6901 pointer via serde_json). No I/O here — the studio's render
//! loop owns the polling and hands content in; everything below is
//! deterministic and unit-tested.

/// The most a bound file may weigh — a scoreboard/lower-third data file is
/// bytes, not megabytes; the cap keeps a mistyped path to a video from
/// stalling the render loop's poll.
pub const MAX_BOUND_FILE_BYTES: u64 = 262_144;

/// Extract one CSV cell. `row` is 1-based over DATA rows when `column`
/// names a header (the header row is skipped), or over ALL rows when
/// `column` is a 1-based index. Quote-aware (`"a,b"`, doubled `""` quotes).
pub fn csv_cell(content: &str, row: u32, column: &str) -> Result<String, String> {
    let rows: Vec<Vec<String>> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(split_csv_line)
        .collect();
    if rows.is_empty() {
        return Err("the file has no rows".into());
    }
    let column = column.trim();
    let (col_index, data_start) = match column.parse::<usize>() {
        Ok(index) if index >= 1 => (index - 1, 0),
        _ => {
            let header = &rows[0];
            let found = header
                .iter()
                .position(|name| name.trim().eq_ignore_ascii_case(column))
                .ok_or_else(|| format!("no column named {column:?} in the header"))?;
            (found, 1)
        }
    };
    let row_index = data_start + row.max(1) as usize - 1;
    let cells = rows
        .get(row_index)
        .ok_or_else(|| format!("the file has no row {row}"))?;
    cells
        .get(col_index)
        .map(|cell| cell.trim().to_string())
        .ok_or_else(|| format!("row {row} has no column {}", col_index + 1))
}

/// Split one CSV line, honoring quotes and doubled-quote escapes.
fn split_csv_line(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '"' if quoted && chars.peek() == Some(&'"') => {
                cell.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                cells.push(std::mem::take(&mut cell));
            }
            other => cell.push(other),
        }
    }
    cells.push(cell);
    cells
}

/// Extract one JSON value at an RFC 6901 pointer. Strings come back bare
/// (no quotes); numbers/booleans render as written; objects/arrays are
/// refused (a lower-third shows a value, not a blob).
pub fn json_value(content: &str, pointer: &str) -> Result<String, String> {
    let parsed: serde_json::Value =
        serde_json::from_str(content).map_err(|err| format!("not valid JSON: {err}"))?;
    let pointer = pointer.trim();
    let normalized = if pointer.is_empty() || pointer.starts_with('/') {
        pointer.to_string()
    } else {
        format!("/{pointer}") // "teams/0/score" is an obvious intent — accept it
    };
    let value = parsed
        .pointer(&normalized)
        .ok_or_else(|| format!("nothing at {pointer:?}"))?;
    match value {
        serde_json::Value::String(text) => Ok(text.clone()),
        serde_json::Value::Number(number) => Ok(number.to_string()),
        serde_json::Value::Bool(flag) => Ok(flag.to_string()),
        serde_json::Value::Null => Ok(String::new()),
        _ => Err(format!("{pointer:?} is an object/array, not a value")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_cells_by_header_name_skip_the_header_row() {
        let csv = "team,score\nRed,17\nBlue,21\n";
        assert_eq!(csv_cell(csv, 1, "score").unwrap(), "17");
        assert_eq!(csv_cell(csv, 2, "score").unwrap(), "21");
        assert_eq!(
            csv_cell(csv, 2, "TEAM").unwrap(),
            "Blue",
            "case-insensitive"
        );
    }

    #[test]
    fn csv_cells_by_index_count_every_row() {
        let csv = "team,score\nRed,17\n";
        assert_eq!(csv_cell(csv, 1, "2").unwrap(), "score", "row 1 = header");
        assert_eq!(csv_cell(csv, 2, "2").unwrap(), "17");
    }

    #[test]
    fn csv_quotes_hold_commas_and_doubled_quotes() {
        let csv = "name,motto\n\"Reds, the\",\"say \"\"go\"\"\"\n";
        assert_eq!(csv_cell(csv, 1, "name").unwrap(), "Reds, the");
        assert_eq!(csv_cell(csv, 1, "motto").unwrap(), "say \"go\"");
    }

    #[test]
    fn csv_misses_are_honest() {
        let csv = "a,b\n1,2\n";
        assert!(csv_cell(csv, 1, "zzz")
            .unwrap_err()
            .contains("no column named"));
        assert!(csv_cell(csv, 9, "a").unwrap_err().contains("no row 9"));
        assert!(csv_cell("", 1, "a").unwrap_err().contains("no rows"));
    }

    #[test]
    fn json_pointers_reach_values_of_every_scalar_kind() {
        let json = r#"{"teams":[{"name":"Red","score":17,"live":true}],"note":null}"#;
        assert_eq!(json_value(json, "/teams/0/name").unwrap(), "Red");
        assert_eq!(json_value(json, "/teams/0/score").unwrap(), "17");
        assert_eq!(json_value(json, "/teams/0/live").unwrap(), "true");
        assert_eq!(json_value(json, "/note").unwrap(), "");
        // A missing leading slash is an obvious intent — accepted.
        assert_eq!(json_value(json, "teams/0/name").unwrap(), "Red");
    }

    #[test]
    fn json_misses_and_blobs_are_honest() {
        let json = r#"{"teams":[1,2]}"#;
        assert!(json_value(json, "/nope")
            .unwrap_err()
            .contains("nothing at"));
        assert!(json_value(json, "/teams")
            .unwrap_err()
            .contains("not a value"));
        assert!(json_value("{broken", "/x")
            .unwrap_err()
            .contains("not valid JSON"));
    }
}
