use std::fs;
use std::collections::HashMap;
use clap::Parser;
use image::GenericImageView;
use anyhow::{anyhow, Result};

/// Atari 2600 tool that generates C code for text display on the Atari 2600 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text input file to be converted
    txt_filename: String,
    /// Width in pixel of the generated ROM data
    #[arg(short, long, default_value_t = 48)]
    width: u8
}

type Font = HashMap<char, [u8;7]>;

fn read_font() -> Result<Font>
{
    let mut font = Font::new();
    let font_image = image::open("font.png").expect("Can't open font.png");
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 <>°\"'?!@*#$%&()+-";
    for (i, c) in CHARACTERS.chars().enumerate() {
        //debug!("char #{i}: {c}");
        let xp: u32 = (i as u32 % 20) * 4;
        let yp: u32 = (i as u32 / 20) * 8;
        let mut ch = [0u8; 7];
        for y in 0..7 {
            for x in 0..3 {
                ch[y] <<= 1;
                let v = font_image.get_pixel(xp + x, yp + y as u32);
                if v[3] != 255 {
                    ch[y] |= 1;
                }
            }
        }
        font.insert(c, ch);
    } 
    Ok(font)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.txt_filename).expect("Unable to read input file");
    let font = read_font()?;
    let linelen = args.width as usize / 4;
    for (ln, line) in contents.lines().enumerate() {
        let mut v = vec![0;linelen / 2 * 7];
        let mut l = line.to_string();
        if l.len() < linelen {
            for _ in 0..(linelen - l.len()) {
                l.push(' ');
            }
        } else {
            l = l[0..linelen].to_string();
        }
        for (i, c) in l.chars().enumerate() {
            if let Some(def) = font.get(&c) {
                for y in 0..7 {
                    if i & 1 != 0 {
                        v[(i / 2) * 7 + y] |= def[y]
                    } else {
                        v[(i / 2) * 7 + y] = def[y] << 4;
                    }
                }
            } else {
                return Err(anyhow!("Unknown character {c} on line {line}"));
            }
        }
        print!("const char line{ln}[{}] = {{\n\t", v.len());
        for i in 0..v.len() - 1 {
            print!("{}, ", v[i]);
            if ((i + 1) % 7) == 0 {
                print!("\n\t");
            }
        }
        println!("{}}};", v.last().unwrap());
    }
    Ok(())
}
