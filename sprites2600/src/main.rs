use std::fs;
use serde::Deserialize;
use clap::Parser;
use image::GenericImageView;
use anyhow::{anyhow, Result};

/// Atari 2600 tool that generates C code for sprites described in a YAML file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// YAML input file
    filename: String
}

#[derive(Debug, Deserialize)]
struct AllSprites {
    #[serde(default)]
    sprite_sheets: Vec<SpriteSheet>,
}

#[derive(Debug, Deserialize)]
struct SpriteSheet {
    image: String,
    sprites: Vec<Sprite>,
}

#[derive(Debug, Deserialize)]
struct Sprite {
    name: String,
    top: u32,
    left: u32,
    height: u32,
    #[serde(default = "default_pixel_width")]
    pixel_width: u8
}

fn default_pixel_width() -> u8 { 1 }

const PALETTE: [u8; 128 * 5] = [
  0,  0,  0,0, 0,
 64, 64, 64,0, 2,
108,108,108,0, 4,
144,144,144,0, 6,
176,176,176,0, 8,
200,200,200,0, 10,
220,220,220,0, 12,
236,236,236,0, 14,
 68, 68,  0,1, 0,
100,100, 16,1, 2,
132,132, 36,1, 4,
160,160, 52,1, 6,
184,184, 64,1, 8,
208,208, 80,1, 10,
232,232, 92,1, 12,
252,252,104,1, 14,
112, 40,  0,2, 0,
132, 68, 20,2, 2,
152, 92, 40,2, 4,
172,120, 60,2, 6,
188,140, 76,2, 8,
204,160, 92,2, 10,
220,180,104,2, 12,
236,200,120,2, 14,
132, 24,  0,3, 0,
152, 52, 24,3, 2,
172, 80, 48,3, 4,
192,104, 72,3, 6,
208,128, 92,3, 8,
224,148,112,3, 10,
236,168,128,3, 12,
252,188,148,3, 14,
136,  0,  0,4, 0,
156, 32, 32,4, 2,
176, 60, 60,4, 4,
192, 88, 88,4, 6,
208,112,112,4, 8,
224,136,136,4, 10,
236,160,160,4, 12,
252,180,180,4, 14,
120,  0, 92,5, 0,
140, 32,116,5, 2,
160, 60,136,5, 4,
176, 88,156,5, 6,
192,112,176,5, 8,
208,132,192,5, 10,
220,156,208,5, 12,
236,176,224,5, 14,
 72,  0,120,6, 0,
 96, 32,144,6, 2,
120, 60,164,6, 4,
140, 88,184,6, 6,
160,112,204,6, 8,
180,132,220,6, 10,
196,156,236,6, 12,
212,176,252,6, 14,
 20,  0,132,7, 0,
 48, 32,152,7, 2,
 76, 60,172,7, 4,
104, 88,192,7, 6,
124,112,208,7, 8,
148,136,224,7, 10,
168,160,236,7, 12,
188,180,252,7, 14,
  0,  0,136,8, 0,
 28, 32,156,8, 2,
 56, 64,176,8, 4,
 80, 92,192,8, 6,
104,116,208,8, 8,
124,140,224,8, 10,
144,164,236,8, 12,
164,184,252,8, 14,
  0, 24,124,9, 0,
 28, 56,144,9, 2,
 56, 84,168,9, 4,
 80,112,188,9, 6,
104,136,204,9, 8,
124,156,220,9, 10,
144,180,236,9, 12,
164,200,252,9, 14,
  0, 44, 92,10, 0,
 28, 76,120,10, 2,
 56,104,144,10, 4,
 80,132,172,10, 6,
104,156,192,10, 8,
124,180,212,10, 10,
144,204,232,10, 12,
164,224,252,10, 14,
  0, 60, 44,11, 0,
 28, 92, 72,11, 2,
 56,124,100,11, 4,
 80,156,128,11, 6,
104,180,148,11, 8,
124,208,172,11, 10,
144,228,192,11, 12,
164,252,212,11, 14,
  0, 60,  0,12, 0,
 32, 92, 32,12, 2,
 64,124, 64,12, 4,
 92,156, 92,12, 6,
116,180,116,12, 8,
140,208,140,12, 10,
164,228,164,12, 12,
184,252,184,12, 14,
 20, 56,  0,13, 0,
 52, 92, 28,13, 2,
 80,124, 56,13, 4,
108,152, 80,13, 6,
132,180,104,13, 8,
156,204,124,13, 10,
180,228,144,13, 12,
200,252,164,13, 14,
 44, 48,  0,14, 0,
 76, 80, 28,14, 2,
104,112, 52,14, 4,
132,140, 76,14, 6,
156,168,100,14, 8,
180,192,120,14, 10,
204,212,136,14, 12,
224,236,156,14, 14,
 68, 40,  0,15, 0,
100, 72, 24,15, 2,
132,104, 48,15, 4,
160,132, 68,15, 6,
184,156, 88,15, 8,
208,180,108,15, 10,
232,204,124,15, 12,
252,224,140,15, 14];

fn main() -> Result<()> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.filename).expect("Unable to read input file");
    let all_sprites: AllSprites = serde_yaml::from_str(&contents)?;
    for sprite_sheet in all_sprites.sprite_sheets {
        let img = image::open(&sprite_sheet.image).expect(&format!("Can't open image {}", sprite_sheet.image));

        // Generate sprites data
        for sprite in &sprite_sheet.sprites {

            let mut gfx = Vec::<u8>::new();
            let mut colors = Vec::<u8>::new();
            for y in 0..sprite.height {
                let mut current_byte: u8 = 0;
                let mut cx: Option<u8> = None;
                for x in 0..8 {
                    current_byte <<= 1;
                    let color = img.get_pixel(sprite.left + x * sprite.pixel_width as u32, sprite.top + y);
                    // Find the color in the Atari 2600 color palette
                    if color[3] != 0 && (color[0] != 0 || color[1] != 0 || color[2] != 0) {
                        // Find the color in the VCS palette
                        for c in 0..128 {
                            if color[0] == PALETTE[c * 5] && color[1] == PALETTE[c * 5 + 1] && color[2] == PALETTE[c * 5 + 2] {
                                let cxx = (PALETTE[c * 5 + 3] << 4) | PALETTE[c * 5 + 4];
                                if let Some(cxxx) = cx {
                                    if cxxx != cxx {
                                        return Err(anyhow!("Second color found on line {y} of sprite {}", sprite.name));
                                    }
                                } else {
                                    cx = Some(cxx);
                                }
                            }
                        } 
                        current_byte |= 1;
                    }
                }
                gfx.push(current_byte);
                colors.push(cx.unwrap_or(0));
            }
            // Whoaw. We do have our pixels vector. Let's output it
            print!("const char {}_gfx[{}] = {{0, 0, \n\t", sprite.name, gfx.len() + 4);
            for c in 0..gfx.len() {
                print!("0x{:02x}", gfx[c]);
                if (c + 1) % 16 != 0 {
                    print!(", ");
                } else {
                    print!(",\n\t");
                }
            } 
            println!("0, 0}};");
            // Check if colors contain different values
            let mut cs = colors.clone();
            cs.sort();
            cs.dedup();
            if cs.len() > 1 {
                print!("const char {}_colors[{}] = {{0, 0, \n\t", sprite.name, colors.len() + 2);
                let mut c = 0;
                for _ in 0..colors.len() - 1 {
                    print!("0x{:02x}", colors[c]);
                    if (c + 1) % 16 != 0 {
                        print!(", ");
                    } else {
                        print!(",\n\t");
                    }
                    c += 1;
                } 
                println!("0x{:02x}}}", colors[c]);
            }
        }
    } 

    Ok(())
}
