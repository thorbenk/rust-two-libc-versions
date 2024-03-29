extern crate libc;
extern crate mozjpeg_sys;
extern crate image;

use std::path::Path;
use std::ptr;

use image::GenericImage;
use image::DynamicImage;

use std::mem;

use mozjpeg_sys::*;
use std::ffi::CString;
use libc::funcs::c95::stdio::{fopen, fclose};
use libc::{malloc, free};

// https://github.com/mozilla/mozjpeg/blob/master/example.c

fn write_jpeg (input_image: &DynamicImage, output_path: &Path, quality: i32)
{
    let input_rgb8 = input_image.as_rgb8().unwrap();
    let (width, height) = input_rgb8.dimensions();
    let input_data = input_rgb8.as_ptr();

    unsafe {

        let mut err = mem::zeroed();
        jpeg_std_error(&mut err);

        let mut cinfo: jpeg_compress_struct = mem::zeroed();
        let size = mem::size_of_val(&cinfo) as size_t;
        cinfo.common.err = &mut err;
        if 0 == jpeg_c_bool_param_supported(&cinfo, JBOOLEAN_TRELLIS_QUANT) {
            panic!("Not linked to mozjpeg?");
        }

        jpeg_CreateCompress(&mut cinfo, JPEG_LIB_VERSION, size);

        let filename = CString::new(output_path.to_str().unwrap()).unwrap();
        let openmode = CString::new("wb").unwrap();
        let outfile = fopen (filename.as_ptr(), openmode.as_ptr());
        if outfile.is_null() {
            panic!("Could not write to file");
        }

        jpeg_stdio_dest(&mut cinfo, outfile);

        cinfo.image_width = width;
        cinfo.image_height = height;
        cinfo.input_components = 3;
        cinfo.in_color_space = JCS_RGB;

        jpeg_set_defaults(&mut cinfo);
        jpeg_set_quality(&mut cinfo, quality, true as i32);
        jpeg_start_compress(&mut cinfo, true as i32);
        let row_stride:isize = width as isize * 3;
        while cinfo.next_scanline < cinfo.image_height {
            let row_pointer = &input_data.offset(cinfo.next_scanline as isize * row_stride);
            jpeg_write_scanlines(&mut cinfo, row_pointer, 1);
        }
        jpeg_finish_compress(&mut cinfo);
        fclose(outfile);
        jpeg_destroy_compress(&mut cinfo);
    }

}

fn read_jpeg(input_path: &Path) -> DynamicImage
{
    unsafe {
        // open file
        let filename = CString::new(input_path.to_str().unwrap()).unwrap();
        let openmode = CString::new("rb").unwrap();
        let infile = fopen (filename.as_ptr(), openmode.as_ptr());
        if infile.is_null() {
            panic!("Could not read file");
        }

        let mut err = mem::zeroed();
        jpeg_std_error(&mut err);

        let mut cinfo: jpeg_decompress_struct = mem::zeroed();
        let size = mem::size_of_val(&cinfo) as size_t;
        cinfo.common.err = &mut err;

        jpeg_CreateDecompress(&mut cinfo, JPEG_LIB_VERSION, size);

        jpeg_stdio_src(&mut cinfo, infile);
        jpeg_read_header(&mut cinfo, true as i32);
        jpeg_start_decompress(&mut cinfo);

        let mut output_image = DynamicImage::new_rgb8 (cinfo.output_width, cinfo.output_height);
        let mut output_buffer = output_image.as_mut_rgb8().unwrap().as_mut_ptr();

        let row_stride:u64 = cinfo.output_width as u64 * cinfo.output_components as u64;
        let mut buffer = malloc(row_stride as u64) as *mut u8;

        while cinfo.output_scanline < cinfo.output_height {
            jpeg_read_scanlines(&mut cinfo, &mut buffer, 1);

            let mut output_row = &output_buffer.offset(cinfo.output_scanline as isize * row_stride as isize);
            ptr::copy(buffer, *output_row, row_stride as usize);
        }

        free(buffer as *mut c_void);

        jpeg_finish_decompress(&mut cinfo);
        jpeg_destroy_decompress(&mut cinfo);
        fclose(infile);

        output_image
    }
}

fn main() {
    let input_path = &Path::new("/tmp/input.jpg");
    let output_path = Path::new("/tmp/output.jpg");

    println!("read image");
    // let input_image = image::open(input_path).unwrap();
    let input_image = read_jpeg(&input_path);

    println!("write image");
    write_jpeg(&input_image, &output_path, 100);

    println!("done");
}
