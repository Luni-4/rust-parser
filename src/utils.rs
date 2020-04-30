// Credits: (musl libc library)[https://www.musl-libc.org/]
pub(crate) fn iswspace(wc: u32) -> i32 {
    const SPACES: [u32; 21] = [
        b' ' as _, b'\t' as _, b'\n' as _, b'\r' as _, 11, 12, 0x0085, 0x2000, 0x2001, 0x2002,
        0x2003, 0x2004, 0x2005, 0x2006, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029, 0x205f, 0x3000,
    ];

    (wc != 0 && SPACES.contains(&wc)) as i32
}

// Credits: (musl libc library)[https://www.musl-libc.org/]
pub(crate) fn iswdigit(wc: u32) -> i32 {
    (wc.wrapping_sub(b'0' as u32) < 10) as i32
}

// Credits: (musl libc library)[https://www.musl-libc.org/]
pub(crate) fn iswalpha(wc: u32) -> i32 {
    const TABLE: [u32; 3904] = include!("./alpha.data");

    if wc < 0x20000 {
        ((TABLE[(TABLE[(wc >> 8) as usize].wrapping_mul(32) + ((wc & 255) >> 3)) as usize]
            >> (wc & 7))
            & 1) as i32
    } else if wc < 0x2fffe {
        1
    } else {
        0
    }
}

// Credits: (musl libc library)[https://www.musl-libc.org/]
pub(crate) fn iswalnum(wc: u32) -> i32 {
    (iswdigit(wc) != 0 || iswalpha(wc) != 0) as i32
}
