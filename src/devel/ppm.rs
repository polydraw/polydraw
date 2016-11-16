use std::io;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::slice;


pub fn write_ppm(filename: &str, width: usize, height: usize, mut buffer: *const u8) -> io::Result<()> {
   let path = Path::new(filename);
   let mut file = try!(File::create(&path));

   let header = format!("P6 {} {} 255\n", width, height);

   try!(file.write(header.as_bytes()));

   for _ in 0..width * height {
      let data = unsafe { slice::from_raw_parts(buffer, 3) };

      try!(file.write(data));

      unsafe {
         buffer = buffer.offset(4);
      }
   }

   Ok(())
}
