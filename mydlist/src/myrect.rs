pub fn draw_rect(sz: u32) {
	println!();
	for i in 0 .. sz {
		for j in 0 .. sz {
			let mut c = '+';
			if (i + j) % 2 == 1 {
				c = '-';
			}
			print!(" {}", c);
		}
		println!();
	}
}
