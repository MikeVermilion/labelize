use image::RgbaImage;
use super::bit_matrix::BitMatrix;

pub const ESCAPE_FNC_1: char = '\u{00F1}';

// Code 128 character set patterns
const CODE_A_START: u8 = 103;
const CODE_B_START: u8 = 104;
const CODE_C_START: u8 = 105;
const STOP: u8 = 106;

// Code 128 bar patterns (each code has 6 alternating bar/space widths)
static PATTERNS: [[u8; 6]; 108] = [
    [2,1,2,2,2,2],[2,2,2,1,2,2],[2,2,2,2,2,1],[1,2,1,2,2,3],[1,2,1,3,2,2],
    [1,3,1,2,2,2],[1,2,2,2,1,3],[1,2,2,3,1,2],[1,3,2,2,1,2],[2,2,1,2,1,3],
    [2,2,1,3,1,2],[2,3,1,2,1,2],[1,1,2,2,3,2],[1,2,2,1,3,2],[1,2,2,2,3,1],
    [1,1,3,2,2,2],[1,2,3,1,2,2],[1,2,3,2,2,1],[2,2,3,2,1,1],[2,2,1,1,3,2],
    [2,2,1,2,3,1],[2,1,3,2,1,2],[2,2,3,1,1,2],[3,1,2,1,3,1],[3,1,1,2,2,2],
    [3,2,1,1,2,2],[3,2,1,2,2,1],[3,1,2,2,1,2],[3,2,2,1,1,2],[3,2,2,2,1,1],
    [2,1,2,1,2,3],[2,1,2,3,2,1],[2,3,2,1,2,1],[1,1,1,3,2,3],[1,3,1,1,2,3],
    [1,3,1,3,2,1],[1,1,2,3,1,3],[1,3,2,1,1,3],[1,3,2,3,1,1],[2,1,1,3,1,3],
    [2,3,1,1,1,3],[2,3,1,3,1,1],[1,1,2,1,3,3],[1,1,2,3,3,1],[1,3,2,1,3,1],
    [1,1,3,1,2,3],[1,1,3,3,2,1],[1,3,3,1,2,1],[3,1,3,1,2,1],[2,1,1,3,3,1],
    [2,3,1,1,3,1],[2,1,3,1,1,3],[2,1,3,3,1,1],[2,1,3,1,3,1],[3,1,1,1,2,3],
    [3,1,1,3,2,1],[3,3,1,1,2,1],[3,1,2,1,1,3],[3,1,2,3,1,1],[3,3,2,1,1,1],
    [3,1,4,1,1,1],[2,2,1,4,1,1],[4,3,1,1,1,1],[1,1,1,2,2,4],[1,1,1,4,2,2],
    [1,2,1,1,2,4],[1,2,1,4,2,1],[1,4,1,1,2,2],[1,4,1,2,2,1],[1,1,2,2,1,4],
    [1,1,2,4,1,2],[1,2,2,1,1,4],[1,2,2,4,1,1],[1,4,2,1,1,2],[1,4,2,2,1,1],
    [2,4,1,2,1,1],[2,2,1,1,1,4],[4,1,3,1,1,1],[2,4,1,1,1,2],[1,3,4,1,1,1],
    [1,1,1,2,4,2],[1,2,1,1,4,2],[1,2,1,2,4,1],[1,1,4,2,1,2],[1,2,4,1,1,2],
    [1,2,4,2,1,1],[4,1,1,2,1,2],[4,2,1,1,1,2],[4,2,1,2,1,1],[2,1,2,1,4,1],
    [2,1,4,1,2,1],[4,1,2,1,2,1],[1,1,1,1,4,3],[1,1,1,3,4,1],[1,3,1,1,4,1],
    [1,1,4,1,1,3],[1,1,4,3,1,1],[4,1,1,1,1,3],[4,1,1,3,1,1],[1,1,3,1,4,1],
    [1,1,4,1,3,1],[3,1,1,1,4,1],[4,1,1,1,3,1],[2,1,1,4,1,2],[2,1,1,2,1,4],
    [2,1,1,2,3,2],[2,3,3,1,1,1],[2,1,1,1,3,2],
];

static STOP_PATTERN: [u8; 7] = [2, 3, 3, 1, 1, 1, 2];

fn encode_pattern(codes: &[u8]) -> BitMatrix {
    let mut total_width = 0usize;
    for &code in codes.iter().take(codes.len() - 1) {
        let pattern = &PATTERNS[code as usize];
        let pw: usize = pattern.iter().map(|&w| w as usize).sum();
        total_width += pw;
    }
    // Stop pattern
    let sw: usize = STOP_PATTERN.iter().map(|&w| w as usize).sum();
    total_width += sw;
    // Quiet zones
    total_width += 20;

    let mut bm = BitMatrix::new(total_width, 1);
    let mut pos = 10; // quiet zone

    for (i, &code) in codes.iter().enumerate() {
        let pattern = if i == codes.len() - 1 {
            &STOP_PATTERN[..]
        } else {
            &PATTERNS[code as usize][..]
        };

        let mut bar = true;
        for &w in pattern {
            for _ in 0..w {
                if bar {
                    bm.set(pos, 0, true);
                }
                pos += 1;
            }
            bar = !bar;
        }
    }

    bm
}

pub fn encode_auto(content: &str, height: i32, bar_width: i32) -> Result<RgbaImage, String> {
    let mut codes: Vec<u8> = Vec::new();
    codes.push(CODE_B_START);

    let mut checksum = CODE_B_START as u32;

    for (i, ch) in content.chars().enumerate() {
        let code = if ch == ESCAPE_FNC_1 {
            102 // FNC1
        } else {
            let b = ch as u8;
            if b >= 32 && b <= 127 {
                b - 32
            } else {
                0 // fallback
            }
        };
        codes.push(code);
        checksum += code as u32 * (i as u32 + 1);
    }

    codes.push((checksum % 103) as u8);
    codes.push(STOP);

    let bm = encode_pattern(&codes);
    Ok(bm.to_1d_image(bar_width.max(1) as usize, height.max(1) as usize))
}

pub fn encode_no_mode(content: &str, height: i32, bar_width: i32) -> Result<(RgbaImage, String), String> {
    // In no-mode, subset codes like >: >; >9 >0 etc. select subsets
    let mut codes: Vec<u8> = Vec::new();
    let mut text = String::new();
    let mut current_set = 'B'; // Default to Code B

    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    // Determine start code
    let start_code = if !chars.is_empty() && chars[0] == '>' && chars.len() > 1 {
        match chars[1] {
            ':' => { i = 2; current_set = 'C'; CODE_C_START }
            ';' => { i = 2; current_set = 'B'; CODE_B_START }
            '9' | '0' => { i = 2; current_set = 'A'; CODE_A_START }
            _ => CODE_B_START,
        }
    } else {
        CODE_B_START
    };
    codes.push(start_code);
    let mut checksum = start_code as u32;
    let mut weight = 1u32;

    while i < chars.len() {
        if chars[i] == '>' && i + 1 < chars.len() {
            match chars[i + 1] {
                ':' => { current_set = 'C'; i += 2; continue; }
                ';' => { current_set = 'B'; i += 2; continue; }
                '9' | '0' => { current_set = 'A'; i += 2; continue; }
                '8' => {
                    // FNC1
                    codes.push(102);
                    checksum += 102 * weight;
                    weight += 1;
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        let code = match current_set {
            'C' => {
                if i + 1 < chars.len() && chars[i].is_ascii_digit() && chars[i + 1].is_ascii_digit() {
                    let val = (chars[i] as u8 - b'0') * 10 + (chars[i + 1] as u8 - b'0');
                    text.push(chars[i]);
                    text.push(chars[i + 1]);
                    i += 1;
                    val
                } else {
                    let ch = chars[i];
                    text.push(ch);
                    let b = ch as u8;
                    if b >= 32 && b <= 127 { b - 32 } else { 0 }
                }
            }
            'A' => {
                let ch = chars[i];
                text.push(ch);
                let b = ch as u8;
                if b >= 32 && b <= 95 { b - 32 }
                else if b < 32 { b + 64 }
                else { 0 }
            }
            _ => { // B
                let ch = chars[i];
                text.push(ch);
                let b = ch as u8;
                if b >= 32 && b <= 127 { b - 32 } else { 0 }
            }
        };

        codes.push(code);
        checksum += code as u32 * weight;
        weight += 1;
        i += 1;
    }

    codes.push((checksum % 103) as u8);
    codes.push(STOP);

    let bm = encode_pattern(&codes);
    Ok((bm.to_1d_image(bar_width.max(1) as usize, height.max(1) as usize), text))
}
