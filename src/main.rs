extern crate libc;
extern crate mozjpeg_sys;

use std::mem;

use mozjpeg_sys::*;
use std::ffi::CString;
use libc::funcs::c95::stdio::fopen;

fn write_jpeg()
{
    let mut err = mem::zeroed();
    jpeg_std_error(&mut err);
    let mut cinfo: jpeg_compress_struct = mem::zeroed();
    let size = mem::size_of_val(&cinfo) as size_t;
    cinfo.common.err = &mut err;
    if 0 == jpeg_c_bool_param_supported(&cinfo, JBOOLEAN_TRELLIS_QUANT) {
        panic!("Not linked to mozjpeg?");
    }
    jpeg_CreateCompress(&mut cinfo, JPEG_LIB_VERSION, size);

    unsafe {
        let filename = CString::new("/tmp/test.jpg").unwrap();
        let openmode = CString::new("wb").unwrap();
        let mut outfile = fopen (filename.as_ptr(), openmode.as_ptr());
        jpeg_stdio_dest(&mut cinfo, outfile);
    }

    jpeg_destroy_compress(&mut cinfo);
}

fn main() {
    write_jpeg();
}
