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

//! `list-n2 <manifest.tsv>` - lists the Open Worm Movement Database's N2 records on Zenodo,
//! keeps the eligible ones by the survey's rule (`fetch::zenodo`), and writes their manifest.
//!
//! The listing is the survey's: the community's records answering `title:"N2 Schafer Lab N2"`,
//! 25 a page, sorted oldest first (unsorted pages repeat some records and miss others), each
//! page retried with a doubling pause when Zenodo drops the connection.

use fetch::zenodo::{Listing, parse_page};
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

const LISTING: &str = "https://zenodo.org/api/communities/open-worm-movement-database/records";
const QUERY: &str = "title:\"N2 Schafer Lab N2\"";
const ATTEMPTS: u32 = 8;

fn page(agent: &ureq::Agent, number: u32) -> Result<String, String> {
    let mut last = String::new();
    for attempt in 0..ATTEMPTS {
        let answer = agent
            .get(LISTING)
            .query("q", QUERY)
            .query("size", "25")
            .query("sort", "oldest")
            .query("page", number.to_string())
            .call()
            .and_then(|mut response| response.body_mut().read_to_string());
        match answer {
            Ok(body) => return Ok(body),
            Err(error) => {
                last = error.to_string();
                eprintln!("page {number}, attempt {}: {last}", attempt + 1);
                sleep(Duration::from_secs(2 << attempt));
            }
        }
    }
    Err(format!("gave up at page {number}: {last}"))
}

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: list-n2 <manifest.tsv>");
        return ExitCode::FAILURE;
    };
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .proxy(ureq::Proxy::try_from_env())
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_global(Some(Duration::from_secs(120)))
        .build()
        .into();
    let mut listing = Listing::default();
    for number in 1.. {
        let records = match page(&agent, number).and_then(|body| parse_page(&body).map_err(|error| error.to_string())) {
            Ok(records) => records,
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        };
        if records.is_empty() {
            break;
        }
        listing.add(records);
        eprintln!("page {number}: {} records so far", listing.len());
    }
    let manifest = listing.manifest();
    let eligible = manifest.lines().count() - 1;
    if let Err(error) = std::fs::write(&path, manifest) {
        eprintln!("writing {path}: {error}");
        return ExitCode::FAILURE;
    }
    eprintln!("{} records listed, {eligible} eligible, written to {path}", listing.len());
    ExitCode::SUCCESS
}
