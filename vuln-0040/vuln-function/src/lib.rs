pub use enums::{Error, Genre};
pub use types::{AudioTag, MP3Metadata, OptionalAudioTags};

mod enums;
mod types;
mod utils;

use utils::{
    create_utf8_str, get_text_field, get_text_fields,
};
use utils::{get_url_field, get_url_fields};

pub fn get_id3(i: &mut u32, buf: &[u8], meta: &mut MP3Metadata) -> Result<(), Error> {
    let mut x = *i as usize;
    // Get extended information
    if buf.len() > 32 && x + 32 < buf.len() && &buf[x..x + 8] == b"APETAGEX" {
        // APE
        *i += 31; // skip APE header / footer
        Ok(())
    } else if buf.len() > 127 && x + 127 < buf.len() && &buf[x..x + 3] == b"TAG" {
        // V1
        if meta.tag.is_some() {
            return Err(Error::DuplicatedIDV3);
        }
        if let Some(last) = meta.frames.last_mut() {
            if *i <= last.size {
                return Ok(());
            }
            last.size = *i - last.size - 1;
        }
        *i += 126;
        // tag v1
        meta.tag = Some(AudioTag {
            title: create_utf8_str(&buf[x + 3..][..30]),
            artist: create_utf8_str(&buf[x + 33..][..30]),
            album: create_utf8_str(&buf[x + 63..][..30]),
            year: create_utf8_str(&buf[x + 93..][..4])
                .parse::<u16>()
                .unwrap_or(0),
            comment: create_utf8_str(&buf[x + 97..][..if buf[x + 97 + 28] != 0 { 30 } else { 28 }]),
            genre: Genre::from(buf[x + 127]),
        });
        Ok(())
    } else if buf.len() > x + 13 && &buf[x..x + 3] == b"ID3" {
        // V2 and above
        let maj_version = buf[x + 3];
        let min_version = buf[x + 4];

        if maj_version > 4 {
            return Ok(());
        }

        let tag_size = ((buf[x + 9] as usize) & 0xFF)
            | (((buf[x + 8] as usize) & 0xFF) << 7)
            | (((buf[x + 7] as usize) & 0xFF) << 14)
            | ((((buf[x + 6] as usize) & 0xFF) << 21) + 10);
        let use_sync = buf[x + 5] & 0x80 != 0;
        let has_extended_header = buf[x + 5] & 0x40 != 0;

        x += 10;

        if has_extended_header {
            if x + 4 >= buf.len() {
                *i = x as u32;
                return Ok(());
            }
            let header_size = ((buf[x] as u32) << 21)
                | ((buf[x + 1] as u32) << 14)
                | ((buf[x + 2] as u32) << 7)
                | buf[x + 3] as u32;
            if header_size < 4 {
                return Ok(());
            }
            x += header_size as usize - 4;
        }

        *i = x as u32 + tag_size as u32;
        if x + tag_size >= buf.len() {
            return Ok(());
        }

        // Recreate the tag if desynchronization is used inside; we need to replace
        // 0xFF 0x00 with 0xFF
        let mut v = Vec::new();
        let (buf, length) = if use_sync {
            let mut new_pos = 0;
            let mut skip = false;
            v.reserve(tag_size);

            for i in 0..tag_size {
                if skip {
                    skip = false;
                    continue;
                }
                if i + 1 >= buf.len() {
                    return Ok(());
                }
                if i + 1 < tag_size && buf[i] == 0xFF && buf[i + 1] == 0 {
                    v[new_pos] = 0xFF;
                    new_pos += 1;
                    skip = true;
                    continue;
                }
                if new_pos >= v.len() {
                    return Ok(());
                }
                v[new_pos] = buf[i];
                new_pos += 1;
            }
            (v.as_slice(), new_pos)
        } else {
            (buf, tag_size)
        };

        let mut pos = x;
        let id3_frame_size = if maj_version < 3 { 6 } else { 10 };
        let mut op = OptionalAudioTags::default();
        let mut changes = false;
        loop {
            if pos + id3_frame_size > x + length {
                break;
            }

            // Check if there is there a frame.
            let c = buf[pos];
            #[allow(clippy::manual_range_contains)]
            if c < b'A' || c > b'Z' {
                break;
            }

            // Frame name is 3 chars in pre-ID3v3 and 4 chars after
            let (frame_name, frame_size) = if maj_version < 3 {
                (
                    &buf[pos..pos + 3],
                    (buf[pos + 5] as u32 & 0xFF)
                        | ((buf[pos + 4] as u32 & 0xFF) << 8)
                        | ((buf[pos + 3] as u32 & 0xFF) << 16),
                )
            } else if maj_version < 4 {
                (
                    &buf[pos..pos + 4],
                    (buf[pos + 7] as u32 & 0xFF)
                        | ((buf[pos + 6] as u32 & 0xFF) << 8)
                        | ((buf[pos + 5] as u32 & 0xFF) << 16)
                        | ((buf[pos + 4] as u32 & 0xFF) << 24),
                )
            } else {
                (
                    &buf[pos..pos + 4],
                    (buf[pos + 7] as u32 & 0xFF)
                        | ((buf[pos + 6] as u32 & 0xFF) << 7)
                        | ((buf[pos + 5] as u32 & 0xFF) << 14)
                        | ((buf[pos + 4] as u32 & 0xFF) << 21),
                )
            };

            pos += id3_frame_size;
            if pos + frame_size as usize > x + length {
                break;
            }

            // http://id3.org/id3v2.3.0#Declared_ID3v2_frames
            match frame_name {
                // -----------------------
                // ----- TEXT FRAMES -----
                // -----------------------
                b"TALB" => {
                    get_text_field(buf, pos, frame_size, &mut changes, &mut op.album_movie_show)
                }
                b"TBPM" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.bpm),
                b"TCOM" => get_text_fields(buf, pos, frame_size, &mut changes, &mut op.composers),
                b"TCON" => {
                    let mut s = None;
                    get_text_field(buf, pos, frame_size, &mut changes, &mut s);
                    if let Some(s) = s {
                        if !s.is_empty() {
                            if s.starts_with('(') && s.ends_with(')') {
                                let v = s
                                    .split(')')
                                    .collect::<Vec<&str>>()
                                    .into_iter()
                                    .filter_map(|a| match a.replace('(', "").parse::<u8>() {
                                        Ok(num) => Some(Genre::from(num)),
                                        _ => None,
                                    })
                                    .collect::<Vec<Genre>>();
                                if !v.is_empty() {
                                    for entry in v {
                                        op.content_type.push(entry);
                                    }
                                } else {
                                    op.content_type.push(Genre::from(s.as_str()));
                                }
                            } else {
                                op.content_type.push(Genre::from(s.as_str()));
                            }
                        }
                    }
                }
                b"TCOP" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.copyright),
                b"TDAT" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.date),
                b"TDLY" => {
                    get_text_field(buf, pos, frame_size, &mut changes, &mut op.playlist_delay)
                }
                b"TENC" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.encoded_by),
                b"TEXT" => {
                    get_text_fields(buf, pos, frame_size, &mut changes, &mut op.text_writers)
                }
                b"TFLT" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.file_type),
                b"TIME" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.time),
                b"TIT" | b"TIT2" => {
                    get_text_field(buf, pos, frame_size, &mut changes, &mut op.title)
                }
                b"TIT1" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.content_group_description,
                ),
                b"TIT3" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.subtitle_refinement_description,
                ),
                b"TKEY" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.initial_key),
                b"TLAN" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.language),
                b"TLEN" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.length),
                b"TMED" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.media_type),
                b"TOAL" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.original_album_move_show_title,
                ),
                b"TOFN" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.original_filename,
                ),
                b"TOLY" => get_text_fields(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.original_text_writers,
                ),
                b"TOPE" => {
                    get_text_fields(buf, pos, frame_size, &mut changes, &mut op.original_artists)
                }
                b"TORY" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.original_release_year,
                ),
                b"TOWN" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.file_owner),
                b"TPE1" => get_text_fields(buf, pos, frame_size, &mut changes, &mut op.performers),
                b"TPE2" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.band),
                b"TPE3" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.conductor),
                b"TPE4" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.interpreted),
                b"TPOS" => {
                    get_text_field(buf, pos, frame_size, &mut changes, &mut op.part_of_a_set)
                }
                b"TPUB" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.publisher),
                b"TRCK" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.track_number),
                b"TRDA" => {
                    get_text_field(buf, pos, frame_size, &mut changes, &mut op.recording_dates)
                }
                b"TRSN" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.internet_radio_station_name,
                ),
                b"TRSO" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.internet_radio_station_owner,
                ),
                b"TSIZ" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.size),
                b"TSRC" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.international_standard_recording_code,
                ),
                b"TSSE" => get_text_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.soft_hard_setting,
                ),
                b"TYER" => get_text_field(buf, pos, frame_size, &mut changes, &mut op.year),
                b"IPLS" => {
                    get_text_field(buf, pos, frame_size, &mut changes, &mut op.involved_people)
                }
                // ----------------------
                // ----- URL FRAMES -----
                // ----------------------
                b"WCOM" => get_url_fields(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.commercial_info_url,
                ),
                b"WCOP" => get_url_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.copyright_info_url,
                ),
                b"WOAF" => {
                    get_url_field(buf, pos, frame_size, &mut changes, &mut op.official_webpage)
                }
                b"WOAR" => get_url_fields(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.official_artist_webpage,
                ),
                b"WOAS" => get_url_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.official_audio_source_webpage,
                ),
                b"WORS" => get_url_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.official_internet_radio_webpage,
                ),
                b"WPAY" => get_url_field(buf, pos, frame_size, &mut changes, &mut op.payment_url),
                b"WPUB" => get_url_field(
                    buf,
                    pos,
                    frame_size,
                    &mut changes,
                    &mut op.publishers_official_webpage,
                ),
                _ => {
                    // TODO: handle other type of fields, like picture
                }
            };

            pos += frame_size as usize;
        }
        if changes {
            op.position = meta.frames.len() as u32;
            op.minor_version = min_version;
            op.major_version = maj_version;
            meta.optional_info.push(op);
        }
        Ok(())
    } else {
        Ok(())
    }
}
