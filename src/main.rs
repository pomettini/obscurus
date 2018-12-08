use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

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
    save_file.seek(SeekFrom::Start(pos as u64)).unwrap();

    for i in (0..PHOTO_TILE_WIDTH * PHOTO_TILE_HEIGHT * 2).step_by(2) {
        save_file.read_exact(&mut tile).unwrap();

        let mut j = 0;
        let mut y = 0;

        while j < 16 {
            let mut k = 0;
            let mut x = 8;

            while k < 8 {
                let mut pixel_value: u8;

                pixel_value = ((tile[j] >> k) & 0x01) + (((tile[j + 1] >> k) & 0x01) << 1);

                pixel_value = pixel_value ^ 3;

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
    let mut image = File::create(full_name).unwrap();
    let mut pgm: String = String::new();

    pgm.push_str("P5\n");
    pgm.push_str(&format!(
        "{} {}\n",
        PHOTO_TILE_WIDTH * TILE_SIDES,
        PHOTO_TILE_HEIGHT * TILE_SIDES
    ));
    pgm.push_str("255\n");

    image.write_all(&pgm.as_bytes()).unwrap();

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

    image.write_all(&pgm).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} <file>\n", &args[0]);
    }

    match File::open(&args[1]) {
        Err(_) => {
            panic!("Error: could not open file '{}'.\n", &args[1]);
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
