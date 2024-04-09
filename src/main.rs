use std::error::Error;
use image;

fn hilbert(n: u32, i: u32) -> (u32, u32) {
    let mut v = (0,0);
    let mut myi = i;
    for j in 0..n {
        let d = myi & 3;
        myi >>= 2;
        v = match d {
            0 => (v.1,v.0),
            1 => (v.0, v.1 + (1 << j)),
            2 => (v.0 + (1 << j), v.1 + (1 << j)),
            3 => ((1 << j+1) - 1 - v.1, (1 << j) - 1 - v.0),
            _ => panic!("Bitwise result should not be possible")
        }
    }
    v
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut img = image::RgbImage::new(256,256*256);

    for y in 0..256*256 {
        let h: (u32,u32) = hilbert(8, y);
        for x in 0..=255 {
            img.put_pixel(x.into(), y, image::Rgb([x, h.0.try_into().unwrap(), h.1.try_into().unwrap()]));
        }
    }

    img.save("test.png")?;

    let n = 2;
    let i = 12;
    let hil = hilbert(n,i);

    println!("Hilbert of ({},{}) is ({},{})", n, i, hil.0, hil.1);

    Ok(())
}
