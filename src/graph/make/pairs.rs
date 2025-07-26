use std::collections::HashMap;

pub fn make_cyclical_pairs_map(vec: &Vec<Vec<usize>>) -> HashMap<(usize, usize), usize> {
	let mut pairs_map: HashMap<(usize, usize), usize> = HashMap::new();
	for (vec_index, inner_vec) in vec.iter().enumerate() {
		let length = inner_vec.len();
		for (index, &inner_vec_index) in inner_vec.iter().enumerate() {
			let pair = (inner_vec_index, inner_vec[(index + 1) % length]);
			pairs_map.insert(pair, vec_index);
		}
	}
	return pairs_map;
}