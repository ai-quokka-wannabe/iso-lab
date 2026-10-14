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

//! `fetch-block <block>` - downloads the features files of one block of the reference draw
//! ([`ORDER`]) into [`RAW`], each named by its record id, and accepts a file only once its MD5
//! matches the record's. A file already present and correct is kept; one that fails is fetched
//! again, up to [`ATTEMPTS`] times. Run from the repository's root; `data/raw/` is never
//! committed (data/README.md).

use fetch::verify_md5;
use fetch::zenodo::content_url;
use std::fs::File;
use std::io::BufReader;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const RAW: &str = "data/raw/owmd-n2";
const ATTEMPTS: u32 = 6;

/// One file of the block: its record, its name on Zenodo and its published MD5.
struct Wanted {
    record: u64,
    key: String,
    checksum: String,
}

fn block_rows(block: u32) -> Result<Vec<Wanted>, String> {
    let text = std::fs::read_to_string(ORDER).map_err(|error| format!("reading {ORDER}: {error}"))?;
    let mut wanted = Vec::new();
    for (index, line) in text.lines().enumerate().skip(1) {
        let fields: Vec<&str> = line.split('\t').collect();
        let [_, row_block, record, key, _, checksum] = fields[..] else {
            return Err(format!("line {} of {ORDER} does not have six fields", index + 1));
        };
        if row_block.parse::<u32>().ok() == Some(block) {
            let record = record.parse().map_err(|_| format!("line {} of {ORDER} has no record id", index + 1))?;
            wanted.push(Wanted {
                record,
                key: key.to_owned(),
                checksum: checksum.to_owned(),
            });
        }
    }
    Ok(wanted)
}

fn is_good(path: &str, checksum: &str) -> bool {
    File::open(path).map(|file| verify_md5(BufReader::new(file), checksum).is_ok()).unwrap_or(false)
}

fn download(agent: &ureq::Agent, url: &str, partial: &str) -> Result<(), String> {
    let mut response = agent.get(url).call().map_err(|error| error.to_string())?;
    let mut file = File::create(partial).map_err(|error| error.to_string())?;
    std::io::copy(&mut response.body_mut().as_reader(), &mut file).map_err(|error| error.to_string())?;
    Ok(())
}

fn fetch_one(agent: &ureq::Agent, wanted: &Wanted) -> Result<(), String> {
    let path = format!("{RAW}/{}_features.hdf5", wanted.record);
    if is_good(&path, &wanted.checksum) {
        return Ok(());
    }
    let partial = format!("{path}.part");
    let url = content_url(wanted.record, &wanted.key);
    for attempt in 0..ATTEMPTS {
        match download(agent, &url, &partial) {
            Ok(()) if is_good(&partial, &wanted.checksum) => {
                return std::fs::rename(&partial, &path).map_err(|error| error.to_string());
            }
            Ok(()) => eprintln!("record {}, attempt {}: MD5 mismatch", wanted.record, attempt + 1),
            Err(error) => eprintln!("record {}, attempt {}: {error}", wanted.record, attempt + 1),
        }
        let _ = std::fs::remove_file(&partial);
        sleep(Duration::from_secs(2 << attempt));
    }
    Err(format!("record {}: gave up after {ATTEMPTS} attempts", wanted.record))
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let block = match arguments.as_slice() {
        [block] => block.parse::<u32>().ok().filter(|&block| block > 0),
        _ => None,
    };
    let Some(block) = block else {
        eprintln!("usage: fetch-block <block>, a positive integer");
        return ExitCode::FAILURE;
    };
    let wanted = match block_rows(block) {
        Ok(wanted) if !wanted.is_empty() => wanted,
        Ok(_) => {
            eprintln!("block {block} is empty");
            return ExitCode::FAILURE;
        }
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(error) = std::fs::create_dir_all(RAW) {
        eprintln!("creating {RAW}: {error}");
        return ExitCode::FAILURE;
    }
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .proxy(ureq::Proxy::try_from_env())
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_global(Some(Duration::from_secs(900)))
        .build()
        .into();
    let mut failed = 0;
    for (index, file) in wanted.iter().enumerate() {
        match fetch_one(&agent, file) {
            Ok(()) => eprintln!("{}/{}: record {} checked", index + 1, wanted.len(), file.record),
            Err(error) => {
                eprintln!("{error}");
                failed += 1;
            }
        }
    }
    eprintln!("block {block}: {} of {} files fetched and checked", wanted.len() - failed, wanted.len());
    if failed == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
