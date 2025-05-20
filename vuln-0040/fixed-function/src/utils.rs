use crate::types::Url;

pub fn create_latin1_str(buf: &[u8]) -> String {
    // interpret each byte as full codepoint. UTF-16 is big enough to
    // represent those, surrogate pairs can't be created that way
    let utf16 = buf.iter().map(|c| *c as u16).collect::<Vec<u16>>();
    String::from_utf16_lossy(utf16.as_ref())
}

pub fn create_utf16_str(buf: &[u8]) -> String {
    let mut v = Vec::<u16>::new();
    if buf.len() >= 2 {
        // BOM: \u{feff}
        if buf[0] == 0xfe && buf[1] == 0xff {
            // UTF-16BE
            v.reserve(buf.len() / 2 - 1);
            for i in 1..buf.len() / 2 {
                v.push(((buf[2 * i] as u16) << 8) | (buf[2 * i + 1] as u16));
            }
            return String::from_utf16_lossy(v.as_ref());
        } else if buf[0] == 0xff && buf[1] == 0xfe {
            // UTF-16LE
            v.reserve(buf.len() / 2 - 1);
            for i in 1..buf.len() / 2 {
                v.push(((buf[2 * i + 1] as u16) << 8) | (buf[2 * i] as u16));
            }
            return String::from_utf16_lossy(v.as_ref());
        }
    }
    // try as UTF-16LE
    v.reserve(buf.len() / 2);
    for i in 0..buf.len() / 2 {
        v.push(((buf[2 * i + 1] as u16) << 8) | (buf[2 * i] as u16))
    }
    String::from_utf16_lossy(v.as_ref())
}

pub fn create_utf8_str(mut buf: &[u8]) -> String {
    // Remove trailing NUL bytes from the input
    while let [rest @ .., last] = buf {
        if *last == 0 {
            buf = rest;
        } else {
            break;
        }
    }

    // String::from_utf8_lossy(buf).into_owned()
    String::from_utf8(buf.to_owned()).unwrap_or_default()
}

pub fn get_url_field(
    buf: &[u8],
    pos: usize,
    size: u32,
    changes: &mut bool,
    value: &mut Option<Url>,
) {
    if value.is_some() || size < 2 {
        return;
    }
    if !(*changes) {
        *changes = true;
    }
    let tmp_v = buf[pos..pos + size as usize].to_vec();
    *value = Some(Url(String::from_utf8(tmp_v).unwrap_or_default()));
}

pub fn get_url_fields(buf: &[u8], pos: usize, size: u32, changes: &mut bool, value: &mut Vec<Url>) {
    let mut tmp = None;
    get_url_field(buf, pos, size, changes, &mut tmp);
    if let Some(tmp) = tmp {
        value.push(tmp);
    }
}

pub fn get_field(buf: &[u8], pos: usize, size: u32) -> String {
    let buf = &buf[pos..][..size as usize];
    if buf.is_empty() {
        String::new()
    } else if buf[0] == 0 {
        // ISO-8859-1
        create_latin1_str(&buf[1..])
    } else if buf[0] == 1 {
        // UTF-16, requires a BOM
        create_utf16_str(&buf[1..])
    } else if buf[0] == 3 {
        // UTF-8
        create_utf8_str(&buf[1..])
    } else {
        String::new()
    }
}

pub fn get_text_field(
    buf: &[u8],
    pos: usize,
    size: u32,
    changes: &mut bool,
    value: &mut Option<String>,
) {
    if value.is_some() || size < 2 {
        return;
    }
    if !(*changes) {
        *changes = true;
    }
    *value = Some(get_field(buf, pos, size));
}

pub fn get_text_fields(
    buf: &[u8],
    pos: usize,
    size: u32,
    changes: &mut bool,
    value: &mut Vec<String>,
) {
    let tmp = get_field(buf, pos, size);
    let tmp_v = tmp.split('/');
    for entry in tmp_v {
        if !entry.is_empty() {
            value.push(entry.to_owned());
        }
    }
    if !(*changes) {
        *changes = true;
    }
}
