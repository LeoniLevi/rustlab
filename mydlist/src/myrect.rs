pub fn draw_rect(sz: u32) {
	println!();

	let pnt : [f32; 3] = [1.0, 2.0, 3.0] ;
	let magn = magnitude(&pnt);
	println!("pnt: {:?}, magn={}", pnt, magn);
	let normalized = normalize(&pnt);
	println!("normalized: {:?}", normalized);

	

	let mx : [[f32; 3]; 3] = [
		[1.0, 1.0, 1.0],
		[2.0, 2.0, 2.0],
		[3.0, 3.0, 3.0]
	];
	let rev_mx = transpose(&mx);
	println!("matrix: {:?}", mx);
	println!("transposed: {:?}", rev_mx);

	//let mut arr  = [5,2,9,1,6,3, 57, -23, 45, 22, 3, -7, 99];
	let mut arr : [i32; 13] = [5,2,9,1,6,3, 57, -23, 45, 22, 3, -7, 99];
	println!("array: {:?}", arr);
	bubble_sort(&mut arr);
	println!("array after bubble_sort: {:?}", arr);

	let mut arr1 : [i32; 6] = [15,23,91,13,16,3];
	println!("array: {:?}", arr1);
	selection_sort(&mut arr1);
	println!("array after bubble_sort: {:?}", arr1);




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

pub fn magnitude(pnt: &[f32; 3]) -> f32 {
	let sq_sum = pnt[0] * pnt[0] + pnt[1] * pnt[1] + pnt[2] * pnt[2];
	//let res = sq_sum.sqrt();
	let res = f32::sqrt(sq_sum);
	res
}

pub fn normalize(pt: &[f32; 3]) -> [f32; 3] {
	let m = magnitude(&pt);
	[pt[0] / m, pt[1] / m, pt[2] / m]
}

pub fn transpose(mx: &[[f32; 3]; 3]) -> [[f32; 3]; 3] {
	let mut m : [[f32; 3]; 3] = [[0.0; 3]; 3];
	for i in 0..3 {
		for j in 0..3 {
			m[i][j] = mx[j][i];
		}
	}
	m
}

//------------------

pub fn bubble_sort(arr: &mut[i32]) {
	let mut len = arr.len();
	while len > 1 {
		for i in 0..len-1 {
			if arr[i] > arr[i+1] {
				(arr[i], arr[i+1]) = (arr[i+1], arr[i]);
			}
		}
		len -= 1;
	}	
}

pub fn selection_sort(arr: &mut[i32]) {
	let len = arr.len();
	for i0 in 0..len {
		//let mut val = arr[i0];
		for i in i0+1..len {
			if arr[i] < arr[i0] {
				(arr[i0], arr[i]) = (arr[i], arr[i0])
			}
		}
	}
}