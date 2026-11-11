//! The sleepy cat that replaces the Claude mark.
//!
//! Five pixel-art stages live in `assets/cat.txt` as 16x16 letter grids and
//! are baked into the exe at build time. The more of the 5-hour window is
//! used, the sleepier the cat. The light outline is added here, not drawn by
//! hand: any empty pixel touching the cat becomes outline, so a black cat
//! stays visible on a dark taskbar.

use crate::render::Rgba;

/// The sprites, copied into the exe when it's built (no file to lose).
const SPRITES: &str = include_str!("../assets/cat.txt");
/// Sprites are GRID x GRID cells.
pub const GRID: i32 = 16;

/// Which stage (0 = wide awake .. 4 = out cold) a 5-hour usage % gets.
pub fn stage(used: Option<f64>) -> usize {
    match used {
        Some(p) if p >= 95.0 => 4,
        Some(p) if p >= 80.0 => 3,
        Some(p) if p >= 50.0 => 2,
        Some(p) if p >= 25.0 => 1,
        _ => 0, // under 25%, or no data yet
    }
}

/// The letter grid for one stage.
fn grid(stage: usize) -> Vec<Vec<u8>> {
    let mut stages: Vec<Vec<Vec<u8>>> = Vec::new();
    for line in SPRITES.lines() {
        let line = line.trim_end();
        if line.starts_with("==") {
            stages.push(Vec::new());
        } else if line.is_empty() || line.starts_with('#') {
            continue;
        } else if let Some(s) = stages.last_mut() {
            s.push(line.bytes().collect());
        }
    }
    let last = stages.len().saturating_sub(1);
    stages.into_iter().nth(stage.min(last)).unwrap_or_default()
}

fn color(ch: u8) -> Option<Rgba> {
    let c = |r: u8, g: u8, b: u8| Some(Rgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0));
    match ch {
        b'K' => c(20, 20, 24),    // fur: near-black
        b'E' => c(190, 240, 70),  // eyes: yellow-green
        b'c' => c(120, 120, 135), // closed eyes, mouth: grey
        b'P' => c(255, 140, 175), // nose, mouth, tongue: pink
        b'Z' => c(170, 210, 255), // Zzz: pale blue
        b'o' => c(200, 200, 215), // outline: light grey
        _ => None,                // see-through
    }
}

/// The cat, `GRID * cell` pixels square, row by row. `None` = see-through.
/// Each grid cell becomes a `cell` x `cell` block, so edges stay sharp.
pub fn draw(stage: usize, cell: i32) -> Vec<Option<Rgba>> {
    let g = grid(stage);
    let at = |y: i32, x: i32| -> u8 {
        if y < 0 || x < 0 {
            return b'.';
        }
        *g.get(y as usize).and_then(|r| r.get(x as usize)).unwrap_or(&b'.')
    };
    let size = GRID * cell;
    let mut out = vec![None; (size * size) as usize];
    for y in 0..GRID {
        for x in 0..GRID {
            let mut ch = at(y, x);
            let touches_cat = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .any(|&(dy, dx)| b"KEcP".contains(&at(y + dy, x + dx)));
            if ch == b'.' && touches_cat {
                ch = b'o';
            }
            if let Some(c) = color(ch) {
                for yy in 0..cell {
                    for xx in 0..cell {
                        out[((y * cell + yy) * size + x * cell + xx) as usize] = Some(c);
                    }
                }
            }
        }
    }
    out
}
