extern crate assert_cmd;
extern crate clap;

use clap::*;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

const FIRST_PHOTO_POSITION: usize = 0x2000;
const PHOTO_OFFSET: usize = 0x1000;
const PHOTO_TILE_WIDTH: usize = 16;
const PHOTO_TILE_HEIGHT: usize = 14;
const TILE_SIDES: usize = 8;
const IMAGE_RASTER_SIZE: usize = PHOTO_TILE_WIDTH * PHOTO_TILE_HEIGHT * TILE_SIDES * TILE_SIDES;

// Returns image raster index given a tile index and x and y coordinates.
fn image_raster_pixel_index_from_tile(tile_index: usize, x: usize, y: usize) -> usize {
    let image_x = x + (tile_index % PHOTO_TILE_WIDTH) * TILE_SIDES;
    let image_y = y + (tile_index / PHOTO_TILE_WIDTH) * TILE_SIDES;
    PHOTO_TILE_WIDTH * TILE_SIDES * image_y + image_x
}

// Takes a Game Boy Camera save RAM file and photo index and populates the
// provided image raster with pixels. Valid index is between 0 and 29.
fn image_raster_from_game_boy_save_ram(
    save_file: &mut File,
    image_raster: &mut [u8; IMAGE_RASTER_SIZE],
    photo_index: usize,
) {
    let mut tile: [u8; 16] = [0; 16];

    let pos = FIRST_PHOTO_POSITION + (PHOTO_OFFSET * photo_index);
    save_file
        .seek(SeekFrom::Start(pos as u64))
        .expect("Cannot write to image file");

    for i in (0..PHOTO_TILE_WIDTH * PHOTO_TILE_HEIGHT * 2).step_by(2) {
        save_file
            .read_exact(&mut tile)
            .expect("Cannot read from .sav file");

        let mut j = 0;
        let mut y = 0;

        while j < 16 {
            let mut k = 0;
            let mut x = 8;

            while k < 8 {
                let mut pixel_value = ((tile[j] >> k) & 0x01) + (((tile[j + 1] >> k) & 0x01) << 1);

                pixel_value ^= 3;

                image_raster[image_raster_pixel_index_from_tile(i / 2, x - 1, y)] = pixel_value;

                k += 1;
                x -= 1;
            }

            j += 2;
            y += 1;
        }
    }
}

// Creates and initializes a PGM file for writing, indicated by filename and
// returns a pointer to the file stream.
// filename and postfix can be at most 256 characters long together.
fn pgm_open_and_initialize(filename: &str, postfix: usize) -> File {
    let full_name = format!("{}-{}.pgm", filename, postfix);
    let mut image = File::create(full_name).expect("Cannot create image file");
    let mut pgm: String = String::new();

    pgm.push_str("P5\n");
    pgm.push_str(&format!(
        "{} {}\n",
        PHOTO_TILE_WIDTH * TILE_SIDES,
        PHOTO_TILE_HEIGHT * TILE_SIDES
    ));
    pgm.push_str("255\n");

    image
        .write_all(&pgm.as_bytes())
        .expect("Cannot write to image file");

    image
}

// Writes an image ("image-<photo_index>.pgm") to disk base on the provided
// image raster.
fn pgm_from_image_raster(image_raster: &[u8], photo_index: usize) {
    let mut image = pgm_open_and_initialize("image", photo_index + 1);
    let mut pgm: Vec<u8> = Vec::new();

    for i in 0..IMAGE_RASTER_SIZE {
        pgm.push(image_raster[i] * 85);
    }

    image.write_all(&pgm).expect("Cannot write to image file");
}

fn main() {
    let matches = App::new("obscurus")
        .version("0.1")
        .author("Giorgio Pomettini <giorgio.pomettini@gmail.com>")
        .arg(Arg::with_name("file").index(1).required(true))
        .get_matches();

    let file_name = matches.value_of("file").unwrap();
    let path = Path::new(file_name);

    match File::open(&path) {
        Err(_e) => {
            panic!("Error: could not open file '{}'.\n", &file_name);
        }
        Ok(mut save_file) => {
            let mut image_raster: [u8; IMAGE_RASTER_SIZE] = [0; IMAGE_RASTER_SIZE];

            for i in 0..30 {
                image_raster_from_game_boy_save_ram(&mut save_file, &mut image_raster, i);
                pgm_from_image_raster(&image_raster, i);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_image_raster_pixel_index_from_tile() {
        assert_eq!(image_raster_pixel_index_from_tile(8, 8, 8), 1096);
    }

    // Add all the other tests

    #[test]
    fn test_functional_file_green() {
        Command::cargo_bin("obscurus")
            .unwrap()
            .arg("gbc.sav")
            .assert()
            .success();

        let output = Command::new("shasum")
            .arg("image-1.pgm")
            .output()
            .expect("Cannot find pgm file");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "8de7e105a13eeb6a9f8a4529c86037c25dfa47cc  image-1.pgm\n"
        );

        clean();
    }

    #[test]
    #[should_panic]
    fn test_functional_file_red() {
        Command::cargo_bin("obscurus")
            .unwrap()
            .arg("gbc.sav")
            .assert()
            .success();

        let output = Command::new("shasum")
            .arg("image-1.pgm")
            .output()
            .expect("Cannot find pgm file");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "8de7e105a13eeb6a9f8a4529c86037c25dfa47ff  image-1.pgm\n"
        );

        clean();
    }

    fn clean() {
        Command::new("find")
            .arg(".")
            .arg("-name")
            .arg("*.pgm")
            .arg("-delete")
            .output()
            .unwrap();
    }
}
