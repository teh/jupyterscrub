use serde::Serialize;

#[derive(Debug)]
pub struct ScrubResult {
    pub modified: bool,
    pub json: String,
}

pub fn scrub(input: &str) -> anyhow::Result<ScrubResult> {
    let mut doc: crate::nbformat::Document = serde_json::from_str(input)?;

    // Some tools (e.g. make, treefmt) use mtime to check for modifications. We
    // only want to write out a new file if we actually modified it.
    let mut modified = false;

    doc.cells = doc
        .cells
        .into_iter()
        .map(|mut x: crate::nbformat::Cell| {
            if let Some(outputs) = x.outputs {
                modified = modified || !outputs.is_empty();
            }
            x.outputs = Some(vec![]);
            x
        })
        .collect();

    // construct a custom formatter to match the 1-indent produced by jupyter notebooks.
    let formatter = serde_json::ser::PrettyFormatter::with_indent(&[b' ']);
    let mut writer = Vec::with_capacity(128);
    let mut serializer = serde_json::ser::Serializer::with_formatter(&mut writer, formatter);
    doc.serialize(&mut serializer)?;

    Ok(ScrubResult {
        modified,
        json: String::from_utf8(writer)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scrub_empty() {
        let doc = std::include_str!("testcases/empty.ipynb");
        scrub(doc).unwrap_err();

        let doc = std::include_str!("testcases/empty2.ipynb");
        scrub(doc).unwrap();
    }

    #[test]
    fn modified() {
        let indoc = std::include_str!("testcases/test.ipynb.in");
        let outdoc = std::include_str!("testcases/test.ipynb.out");
        let result = scrub(indoc).unwrap();
        assert!(result.modified);
        assert_eq!(result.json, outdoc);
    }
}
