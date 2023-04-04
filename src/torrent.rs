// SPDX-License-Identifier: BSD-2-Clause-Patent

use std::borrow::Cow;
use std::path::Path;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

use lava_torrent::bencode::BencodeElem;
use lava_torrent::torrent::v1::{Integer, TorrentBuilder};
use tracing::{info, warn};
use walkdir::WalkDir;

use crate::qtm_config::QtmVersion;
use crate::{data_local_dir, DialogMessage};

fn get_total_length<P: AsRef<Path>>(path: P) -> u64 {
    if path.as_ref().is_file() {
        return path.as_ref().metadata().unwrap().len();
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.metadata().unwrap().len())
            .sum()
    }
}

fn calculate_piece_length(mut total_length: u64) -> u64 {
    // Aim to split to 1024 pieces
    total_length /= 1024;
    total_length += 1;
    2u64.pow(total_length.ilog2()).min(2u64.pow(20))
}

pub(crate) fn create_torrent_file<P: AsRef<Path>>(
    content_path: P,
    sender: mpsc::Sender<DialogMessage>,
) {
    let content_path = content_path.as_ref();
    if !content_path.exists() {
        warn!(?content_path, "Content path does not exist; upload aborted");
        sender
            .send(DialogMessage(
                Cow::Borrowed("Content path does not exist; upload aborted"),
                true,
            ))
            .unwrap();
    }
    let applicaton_name = format!(
        "Quick Torrent Maker 2, v{}",
        QtmVersion::get_current_version()
    );
    let creation_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let torrent = TorrentBuilder::new(
        content_path,
        calculate_piece_length(get_total_length(content_path)) as Integer,
    )
    .set_announce(Some("http://gaytor.rent:2710/announce".to_owned()))
    .set_name(
        content_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
    )
    .set_privacy(true)
    .add_extra_field(
        "creation date".to_owned(),
        BencodeElem::Integer(creation_time),
    )
    .add_extra_field(
        "created by".to_owned(),
        BencodeElem::String(applicaton_name.clone()),
    )
    .add_extra_field(
        "comment".to_owned(),
        BencodeElem::String(format!("This torrent was created by {}", applicaton_name)),
    )
    .add_extra_field(
        "encoding".to_owned(),
        BencodeElem::String("UTF-8".to_owned()),
    )
    .build();

    match torrent {
        Ok(torrent) => {
            let filename = &format!("qtm2-{}.torrent", creation_time);
            info!("{filename} has been created successfully");
            match torrent.write_into_file(data_local_dir(filename)) {
                Ok(_) => {
                    sender
                        .send(DialogMessage(
                            Cow::Borrowed(
                                "Torrent has been written to disk successfully\n\nUploading...",
                            ),
                            true, // TODO: Add uploading & turn this to false
                        ))
                        .unwrap();
                    info!("{filename} has been written to disk successfully");
                }
                Err(err) => {
                    warn!(?err, "Failed to write torrent to disk; upload aborted");
                    sender.send(DialogMessage(
                        Cow::Borrowed("Failed to write torrent to disk\n\nUpload aborted\n\nCheck log for more information."),
                        true,
                    )).unwrap();
                }
            }
        }

        Err(err) => {
            warn!(?err, "Failed to create torrent; upload aborted");
            sender.send(DialogMessage(
                Cow::Borrowed(
                    "Failed to create torrent\n\nUpload aborted\n\nCheck log for more information.",
                ),
                true,
            )).unwrap();
        }
    }
}
