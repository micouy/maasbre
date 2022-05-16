use super::{WIDTH};

pub struct Canvas<'a> {
    pub buf: &'a mut [u8],
	pub z_buf: &'a mut [f64],
}

impl<'a> Canvas<'a> {
    pub fn new(buf: &'a mut [u8], z_buf: &'a mut [f64]) -> Self {
        Self { buf, z_buf }
    }

    pub fn clear(&mut self) {
        for pixel in self.buf.chunks_exact_mut(4) {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 255;
        }

		self.z_buf.fill(f64::NEG_INFINITY);
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, z: f64, color: [u8; 3]) {
        let buf_ix = (x + y * (WIDTH as i32)) * 4;
		let z_buf_ix = (x + y * WIDTH as i32);

        if let (Ok(buf_ix), Ok(z_buf_ix)) = (usize::try_from(buf_ix), usize::try_from(z_buf_ix)) {
			if z_buf_ix < self.z_buf.len() && buf_ix + 4 < self.buf.len() {
    			if z > self.z_buf[z_buf_ix] {
					self.z_buf[z_buf_ix] = z;
                    self.buf[buf_ix + 0] = color[0];
                    self.buf[buf_ix + 1] = color[1];
                    self.buf[buf_ix + 2] = color[2];

					/*
					if 0.0 < z && z < 1.0 {
    					let shade = (z * 255.0).round() as u8;

                        self.buf[buf_ix + 0] = shade;
                        self.buf[buf_ix + 1] = shade;
                        self.buf[buf_ix + 2] = shade;
					}
					*/
				}
            }
        }
    }
}
