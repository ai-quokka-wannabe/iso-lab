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
//! matches the record's. A file comes in [`PIECE`]-byte `Range` requests appended to a `.part`
//! file, each request with its own time limit, so a slow server never cuts a file short and an
//! interrupted run, or an interrupted file, resumes where it stopped. A file already present
//! and correct is kept. After [`ATTEMPTS`] failures in a row without progress, the file is
//! given up. Run from the repository's root; `data/raw/` is never committed (data/README.md).

use fetch::verify_md5;
use fetch::zenodo::content_url;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const RAW: &str = "data/raw/owmd-n2";
const ATTEMPTS: u32 = 6;
const PIECE: u64 = 8 * 1024 * 1024;

/// One file of the block: its record, its name on Zenodo, its size and its published MD5.
struct Wanted {
    record: u64,
    key: String,
    size: u64,
    checksum: String,
}

fn block_rows(block: u32) -> Result<Vec<Wanted>, String> {
    let text = std::fs::read_to_string(ORDER).map_err(|error| format!("reading {ORDER}: {error}"))?;
    let mut wanted = Vec::new();
    for (index, line) in text.lines().enumerate().skip(1) {
        let fields: Vec<&str> = line.split('\t').collect();
        let [_, row_block, record, key, size, checksum] = fields[..] else {
            return Err(format!("line {} of {ORDER} does not have six fields", index + 1));
        };
        if row_block.parse::<u32>().ok() == Some(block) {
            let record = record.parse().map_err(|_| format!("line {} of {ORDER} has no record id", index + 1))?;
            let size = size.parse().map_err(|_| format!("line {} of {ORDER} has no size", index + 1))?;
            wanted.push(Wanted {
                record,
                key: key.to_owned(),
                size,
                checksum: checksum.to_owned(),
            });
        }
    }
    Ok(wanted)
}

fn is_good(path: &str, checksum: &str) -> bool {
    File::open(path).map(|file| verify_md5(BufReader::new(file), checksum).is_ok()).unwrap_or(false)
}

/// Appends the next piece of the file at `url` to `partial`, which already holds `have` bytes.
fn next_piece(agent: &ureq::Agent, url: &str, partial: &str, have: u64, size: u64) -> Result<u64, String> {
    let last = (have + PIECE).min(size) - 1;
    let mut response = agent
        .get(url)
        .header("Range", format!("bytes={have}-{last}"))
        .call()
        .map_err(|error| error.to_string())?;
    if response.status() != 206 {
        return Err(format!("asked for a range, got status {}", response.status()));
    }
    let mut file = OpenOptions::new().append(true).create(true).open(partial).map_err(|error| error.to_string())?;
    let expected = last + 1 - have;
    let copied = std::io::copy(&mut response.body_mut().as_reader().take(expected), &mut file).map_err(|error| error.to_string())?;
    if copied == expected {
        Ok(copied)
    } else {
        Err(format!("a piece ended after {copied} of {expected} bytes"))
    }
}

fn fetch_one(agent: &ureq::Agent, wanted: &Wanted) -> Result<(), String> {
    let path = format!("{RAW}/{}_features.hdf5", wanted.record);
    if is_good(&path, &wanted.checksum) {
        return Ok(());
    }
    let partial = format!("{path}.part");
    let url = content_url(wanted.record, &wanted.key);
    let mut failures = 0;
    while failures < ATTEMPTS {
        let have = std::fs::metadata(&partial).map(|meta| meta.len()).unwrap_or(0);
        if have >= wanted.size {
            if have == wanted.size && is_good(&partial, &wanted.checksum) {
                return std::fs::rename(&partial, &path).map_err(|error| error.to_string());
            }
            eprintln!("record {}: {have} bytes that do not match the MD5; starting again", wanted.record);
            let _ = std::fs::remove_file(&partial);
            failures += 1;
            continue;
        }
        match next_piece(agent, &url, &partial, have, wanted.size) {
            Ok(_) => failures = 0,
            Err(error) => {
                failures += 1;
                eprintln!("record {}, at byte {have}, failure {failures}: {error}", wanted.record);
                if failures < ATTEMPTS {
                    sleep(Duration::from_secs(2 << failures));
                }
            }
        }
    }
    Err(format!("record {}: gave up after {ATTEMPTS} failures in a row", wanted.record))
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
        .timeout_global(Some(Duration::from_secs(600)))
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
