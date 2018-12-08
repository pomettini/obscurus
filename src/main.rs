use std::env;
use std::fs::File;

const FIRST_PHOTO_POSITION: usize = 0x2000;
const PHOTO_OFFSET: usize = 0x1000;
const PHOTO_TILE_WIDTH: usize = 16;
const PHOTO_TILE_HEIGHT: usize = 14;
const TILE_SIDES: usize = 8;
const MAX_FILE_NAME_LENGTH: usize = 262;
const IMAGE_RASTER_SIZE: usize = PHOTO_TILE_WIDTH * PHOTO_TILE_HEIGHT * TILE_SIDES * TILE_SIDES;

// Returns image raster index given a tile index and x and y coordinates.
fn image_raster_pixel_index_from_tile(tile_index: usize, x: usize, y: usize) -> usize {
    let image_x = x + (tile_index % PHOTO_TILE_WIDTH) * TILE_SIDES;
    let image_y = y + (tile_index / PHOTO_TILE_WIDTH) * TILE_SIDES;
    PHOTO_TILE_WIDTH * TILE_SIDES * image_y + image_x
}

// Takes a Game Boy Camera save RAM file and photo index and populates the
// provided image raster with pixels. Valid index is between 0 and 29.
fn image_raster_from_game_boy_save_ram(save_file: &mut File, image_raster: &[u8], photo_index: u8) {
    //   char tile[16];

    //   fseek(save_file, FIRST_PHOTO_POSITION + (PHOTO_OFFSET * photo_index), 0);

    //   for (size_t i = 0; i < PHOTO_TILE_WIDTH * PHOTO_TILE_HEIGHT * 2; i += 2) {
    //     fread(tile, 1, sizeof tile, save_file);

    //     uint8_t pixel_value;

    //     for (size_t j = 0, y = 0; j < 16; j += 2, y++) {
    //       for (size_t k = 0, x = 7; k < 8; k++, x--) {
    //         pixel_value = ((tile[j] >> k) & 0x01) + (((tile[j + 1] >> k) & 0x01) << 1);

    //         pixel_value = pixel_value ^ 3;

    //         image_raster[image_raster_pixel_index_from_tile(i / 2, x, y)] = pixel_value;
    //       }
    //     }
    //   }
}

// Creates and initializes a PGM file for writing, indicated by filename and
// returns a pointer to the file stream.
// filename and postfix can be at most 256 characters long together.
fn pgm_open_and_initialize(filename: &str, postfix: u8) -> Vec<u8> {
    //   char full_name[MAX_FILE_NAME_LENGTH];
    //   sprintf(full_name, "%s-%d.pgm", filename, postfix);

    //   FILE* image = fopen(full_name, "w+");

    //   fputs("P5\n", image);
    //   fprintf(image, "%d %d\n", PHOTO_TILE_WIDTH * TILE_SIDES, PHOTO_TILE_HEIGHT * TILE_SIDES);
    //   fputs("255\n", image);

    //   return image;
    Vec::new()
}

// Writes an image ("image-<photo_index>.pgm") to disk base on the provided
// image raster.
fn pgm_from_image_raster(image_raster: &[u8], photo_index: u8) {
    //   FILE* image = pgm_open_and_initialize("image", photo_index + 1);

    //   for (size_t i = 0; i < IMAGE_RASTER_SIZE; i++) {
    //     fputc(image_raster[i] * 85, image);
    //   }

    //   fclose(image);
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
                image_raster_from_game_boy_save_ram(&mut save_file, &image_raster, i);
                pgm_from_image_raster(&image_raster, i);
            }
        }
    };
}
