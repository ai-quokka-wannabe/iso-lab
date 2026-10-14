/*
    Copyright (C) 2026 The AiQuokkaWannabe Authors <https://github.com/ai-quokka-wannabe/iso-lab>

    This program is free software: you can redistribute it and/or modify it under the terms of
    the GNU General Public License as published by the Free Software Foundation, either version
    3 of the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
    without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
    See the GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along with this program.
    If not, see <https://www.gnu.org/licenses/>.
*/

//! The Open Worm Movement Database's N2 records, as Zenodo lists them, and the rule that
//! picks the lab's reference set out of them (data/README.md, *The Phase 0 Survey*).
//!
//! The rule is the survey's, unchanged: a record is eligible when its description says
//! *hermaphrodite*, *adult*, *OP50* and *30m wait*, and its file names begin with `N2 `
//! (31 records named after mutants, or oddly, are left out until traced).

use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;

/// One file of a record, as its metadata gives it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordFile {
    /// The file name.
    pub key: String,
    /// Its size in bytes.
    pub size: u64,
    /// Its checksum, as Zenodo writes it (`md5:<hex>`).
    pub checksum: String,
}

/// One Zenodo record of the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// Zenodo's record id.
    pub id: u64,
    /// The title, `N2 Schafer Lab N2 (Bristol, UK) | <timestamp>`.
    pub title: String,
    /// The description, in which the recording's fields are written as `<b>name</b>: value`.
    pub description: String,
    /// The record's files.
    pub files: Vec<RecordFile>,
}

/// A page of a listing that does not have the shape Zenodo's API documents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MalformedPage(pub String);

impl fmt::Display for MalformedPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "malformed Zenodo page: {}", self.0)
    }
}

impl std::error::Error for MalformedPage {}

/// The records of one page of a listing, in the order the page gives them.
pub fn parse_page(json: &str) -> Result<Vec<Record>, MalformedPage> {
    let page: Value = serde_json::from_str(json).map_err(|error| MalformedPage(error.to_string()))?;
    let hits = page["hits"]["hits"].as_array().ok_or_else(|| MalformedPage("no hits.hits array".into()))?;
    hits.iter().map(parse_record).collect()
}

fn parse_record(hit: &Value) -> Result<Record, MalformedPage> {
    let text = |value: &Value, what: &str| value.as_str().map(str::to_owned).ok_or_else(|| MalformedPage(format!("no {what}")));
    let id = match &hit["id"] {
        Value::Number(number) => number.as_u64(),
        Value::String(string) => string.parse().ok(),
        _ => None,
    }
    .ok_or_else(|| MalformedPage("no record id".into()))?;
    let files = hit["files"]
        .as_array()
        .ok_or_else(|| MalformedPage(format!("record {id} has no files array")))?
        .iter()
        .map(|file| {
            Ok(RecordFile {
                key: text(&file["key"], "file key")?,
                size: file["size"].as_u64().ok_or_else(|| MalformedPage(format!("record {id}: a file has no size")))?,
                checksum: text(&file["checksum"], "file checksum")?,
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(Record {
        id,
        title: text(&hit["metadata"]["title"], "title")?,
        description: text(&hit["metadata"]["description"], "description")?,
        files,
    })
}

/// The value of the field `name` in a record's description, written there as
/// `<b>name</b>: value<`; the value is trimmed. The survey read the fields with the regular
/// expression `<b>name</b>\s*:\s*([^<]*)<`, and this matches it: `None` when the field is
/// missing, or when its value is not followed by a tag.
pub fn field(description: &str, name: &str) -> Option<String> {
    let start = description.find(&format!("<b>{name}</b>"))? + name.len() + 7;
    let rest = description[start..].trim_start().strip_prefix(':')?.trim_start();
    let end = rest.find('<')?;
    Some(rest[..end].trim().to_owned())
}

impl Record {
    /// The features file, `<name>_features.hdf5`, if the record has one.
    pub fn features_file(&self) -> Option<&RecordFile> {
        self.files.iter().find(|file| file.key.ends_with("_features.hdf5"))
    }

    /// The recording's name: the features file's name without `_features.hdf5`.
    pub fn name(&self) -> Option<&str> {
        self.features_file().map(|file| file.key.trim_end_matches("_features.hdf5"))
    }

    /// Whether the record belongs to the lab's reference set (the module's rule).
    pub fn is_eligible(&self) -> bool {
        let wanted = [("sex", "hermaphrodite"), ("stage", "adult"), ("food", "OP50"), ("habituation", "30m wait")];
        wanted.iter().all(|(name, value)| field(&self.description, name).as_deref() == Some(*value)) && self.name().is_some_and(|name| name.starts_with("N2 "))
    }
}

/// Collects pages into one set, keyed and so ordered by record id: Zenodo's pages, even
/// sorted, are not trusted to be free of repeats, and the manifest must not depend on the
/// order the pages came in.
#[derive(Debug, Default)]
pub struct Listing {
    records: BTreeMap<u64, Record>,
}

impl Listing {
    /// Adds a page's records; a record seen before is replaced by the later copy.
    pub fn add(&mut self, page: Vec<Record>) {
        for record in page {
            self.records.insert(record.id, record);
        }
    }

    /// How many distinct records were listed.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether nothing was listed.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// The manifest of the eligible records, one line each in ascending record id, tab
    /// separated: record id, features file name, its size in bytes, its MD5 as Zenodo gives it.
    pub fn manifest(&self) -> String {
        let mut out = String::from("record\tfeatures_file\tsize\tchecksum\n");
        for record in self.records.values().filter(|record| record.is_eligible()) {
            if let Some(file) = record.features_file() {
                out.push_str(&format!("{}\t{}\t{}\t{}\n", record.id, file.key, file.size, file.checksum));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn description(sex: &str, food: &str) -> String {
        format!("<p><b>strain</b>: N2<br><b>sex</b>: {sex}<br><b>stage</b>: adult<br><b>food</b>: {food}<br><b>habituation</b>: 30m wait<br></p>")
    }

    fn hit(id: u64, name: &str, sex: &str, food: &str) -> String {
        serde_json::json!({
            "id": id,
            "metadata": {"title": format!("N2 Schafer Lab N2 (Bristol, UK) | {id}"), "description": description(sex, food)},
            "files": [
                {"key": format!("{name}.wcon.zip"), "size": 2, "checksum": "md5:bb"},
                {"key": format!("{name}_features.hdf5"), "size": 1, "checksum": "md5:aa"},
            ],
        })
        .to_string()
    }

    fn page(hits: &[String]) -> String {
        format!("{{\"hits\": {{\"hits\": [{}], \"total\": {}}}}}", hits.join(","), hits.len())
    }

    #[test]
    fn field_reads_the_survey_form_and_trims() {
        let text = "<b>sex</b> :  hermaphrodite <br><b>food</b>:OP50<br>";
        assert_eq!(field(text, "sex").as_deref(), Some("hermaphrodite"));
        assert_eq!(field(text, "food").as_deref(), Some("OP50"));
        assert_eq!(field(text, "stage"), None);
        assert_eq!(field("<b>food</b>: OP50", "food"), None, "a value not closed by a tag is not read, as in the survey");
    }

    #[test]
    fn eligibility_is_the_survey_rule() {
        let records = parse_page(&page(&[
            hit(1, "N2 on food R_2010", "hermaphrodite", "OP50"),
            hit(2, "N2 on food R_2011", "male", "OP50"),
            hit(3, "N2 off food R_2012", "hermaphrodite", "no food"),
            hit(4, "dop-1 on food R_2013", "hermaphrodite", "OP50"),
            hit(5, "n2 on food R_2014", "hermaphrodite", "OP50"),
        ]))
        .unwrap();
        let eligible: Vec<u64> = records.iter().filter(|record| record.is_eligible()).map(|record| record.id).collect();
        assert_eq!(eligible, vec![1]);
    }

    #[test]
    fn manifest_is_ordered_by_id_and_free_of_repeats() {
        let mut listing = Listing::default();
        listing.add(parse_page(&page(&[hit(30, "N2 c", "hermaphrodite", "OP50"), hit(10, "N2 a", "hermaphrodite", "OP50")])).unwrap());
        listing.add(parse_page(&page(&[hit(10, "N2 a", "hermaphrodite", "OP50"), hit(20, "N2 b", "male", "OP50")])).unwrap());
        assert_eq!(listing.len(), 3);
        assert_eq!(
            listing.manifest(),
            "record\tfeatures_file\tsize\tchecksum\n10\tN2 a_features.hdf5\t1\tmd5:aa\n30\tN2 c_features.hdf5\t1\tmd5:aa\n"
        );
    }

    #[test]
    fn a_page_without_hits_is_refused() {
        assert!(parse_page("{\"hits\": {}}").is_err());
        assert!(parse_page("not json").is_err());
    }
}
