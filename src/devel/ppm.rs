use std::io;
use std::io::Write;
use std::path::Path;
use std::fs::File;

use draw::RGB;


pub fn write_ppm(filename: &str, width: usize, height: usize, buffer: &Vec<RGB>) -> io::Result<()> {
   let path = Path::new(filename);
   let mut file = try!(File::create(&path));

   let header = format!("P6 {} {} 255\n", width, height);

   try!(file.write(header.as_bytes()));

   let mut data = Vec::with_capacity(width * height);

   for y in 0..height {
      for x in 0..width {
         let i = (y * width + x) as usize;
         data.push(buffer[i].r);
         data.push(buffer[i].g);
         data.push(buffer[i].b);
      }
   }

   try!(file.write(&data));

   Ok(())
}
