use fold::fold::Fold;

use std::{fs, path::PathBuf};
use std::io::BufReader;

pub fn validate_file(file_str: &str) -> bool {
	let file_path = PathBuf::from(file_str);
	let mut file = fs::File::open(file_path).unwrap();
	let mut file_reader = BufReader::new(file);

	let fold: Fold = serde_json::from_reader(file_reader).unwrap();
	//println!("{:?}", fold.key_frame.graph.vertices_coordinates);
	//println!("{:?}", serde_json::to_string(&fold));
	println!("{:?}", fold.validate());
	return fold.validate().is_ok();
}

macro_rules! test_valid {
	($name: ident, $file: expr) => {
		#[test]
		fn $name() { assert!(validate_file($file)); }
    };
}

macro_rules! test_invalid {
	($name: ident, $file: expr) => {
		#[test]
		fn $name() { assert!(!validate_file($file)); }
    };
}

test_valid!(abstract_graph, "tests/abstract-graph.fold");
test_valid!(bad_edges, "tests/bad-edges.fold");
test_valid!(bird_base_3d_cp, "tests/bird-base-3d-cp.fold");
test_valid!(bird_base_3d, "tests/bird-base-3d.fold");
test_valid!(bird_disjoint_edges, "tests/bird-disjoint-edges.fold");
test_valid!(blintz_frames, "tests/blintz-frames.fold");
test_valid!(bowtie, "tests/bowtie.fold");
test_valid!(command_strip, "tests/command-strip.fold");
test_valid!(command_strip_with_back, "tests/command-strip-with-back.fold");
test_valid!(crane_cp_bmvfcj, "tests/crane-cp-bmvfcj.fold");
test_valid!(crane_cp_bmvfcj_simple, "tests/crane-cp-bmvfcj-simple.fold");
test_valid!(crane_cp, "tests/crane-cp.fold");
test_valid!(crane, "tests/crane.fold");
test_valid!(crane_step, "tests/crane-step.fold");
test_valid!(cube_octagon, "tests/cube-octagon.fold");
test_valid!(cycles_3d, "tests/cycles-3d.fold");
test_valid!(disjoint_cps, "tests/disjoint-cps.fold");
test_valid!(disjoint_triangles_3d, "tests/disjoint-triangles-3d.fold");
test_valid!(fan_cp, "tests/fan-cp.fold");
test_valid!(fan_flat_cp, "tests/fan-flat-cp.fold");
test_valid!(fan_folded_through_cp, "tests/fan-folded-through-cp.fold");
test_valid!(fish_cp_3d, "tests/fish-cp-3d.fold");
test_valid!(flat_pleat_fish, "tests/flat-pleat-fish.fold");
test_valid!(invalid_box_pleat_3d, "tests/invalid-box-pleat-3d.fold");
test_valid!(invalid_key_names, "tests/invalid-key-names.fold");
test_valid!(invalid_mismatch_length, "tests/invalid-mismatch-length.fold");
test_valid!(invalid_mismatch_references, "tests/invalid-mismatch-references.fold");
test_valid!(invalid_self_intersect, "tests/invalid-self-intersect.fold");
test_valid!(invalid_single_vertex_2d, "tests/invalid-single-vertex-2d.fold");
test_valid!(invalid_single_vertex_3d, "tests/invalid-single-vertex-3d.fold");
test_valid!(isolated_line_in_face, "tests/isolated-line-in-face.fold");
test_valid!(kabuto, "tests/kabuto.fold");
test_valid!(kissing_squares, "tests/kissing-squares.fold");
test_valid!(kraft_bird_base, "tests/kraft-bird-base.fold");
test_valid!(layer_4_flaps, "tests/layer-4-flaps.fold");
test_valid!(layers_3d_edge_edge, "tests/layers-3d-edge-edge.fold");
test_valid!(layers_3d_edge_face, "tests/layers-3d-edge-face.fold");
test_valid!(layers_cycle_nonconvex, "tests/layers-cycle-nonconvex.fold");
test_valid!(layers_flat_grid, "tests/layers-flat-grid.fold");
test_valid!(layer_solver_conflict, "tests/layer-solver-conflict.fold");
test_valid!(layers_zipper, "tests/layers-zipper.fold");
test_valid!(maze_8x8, "tests/maze-8x8.fold");
test_valid!(maze_s, "tests/maze-s.fold");
test_valid!(maze_u, "tests/maze-u.fold");
test_valid!(moosers_train_carriage, "tests/moosers-train-carriage.fold");
test_valid!(moosers_train_carriage_fourth, "tests/moosers-train-carriage-fourth.fold");
test_valid!(moosers_train_engine, "tests/moosers-train-engine.fold");
test_valid!(moosers_train, "tests/moosers-train.fold");
test_valid!(nested_frames, "tests/nested-frames.fold");
test_valid!(no_faces, "tests/no-faces.fold");
test_valid!(non_flat_paper, "tests/non-flat-paper.fold");
test_valid!(non_planar_100_chaotic, "tests/non-planar-100-chaotic.fold");
test_valid!(non_planar_100_lines, "tests/non-planar-100-lines.fold");
test_valid!(non_planar_25_lines, "tests/non-planar-25-lines.fold");
test_valid!(non_planar_500_chaotic, "tests/non-planar-500-chaotic.fold");
test_valid!(non_planar_50_chaotic, "tests/non-planar-50-chaotic.fold");
test_valid!(non_planar_50_lines, "tests/non-planar-50-lines.fold");
test_valid!(non_planar_75_lines, "tests/non-planar-75-lines.fold");
test_valid!(non_planar_bird_base, "tests/non-planar-bird-base.fold");
test_valid!(non_planar_nonconvex, "tests/non-planar-nonconvex.fold");
test_valid!(non_planar_polygons, "tests/non-planar-polygons.fold");
test_valid!(non_planar_square_fish, "tests/non-planar-square-fish.fold");
test_valid!(overlapping_assignments, "tests/overlapping-assignments.fold");
test_valid!(panels_3x3, "tests/panels-3x3.fold");
test_valid!(panels_3x3_invalid, "tests/panels-3x3-invalid.fold");
test_valid!(panels_4x2, "tests/panels-4x2.fold");
test_valid!(panels_5, "tests/panels-5.fold");
test_valid!(panels_6x2_90deg, "tests/panels-6x2-90deg.fold");
test_valid!(panels_simple, "tests/panels-simple.fold");
test_valid!(panels_zig_zag, "tests/panels-zig-zag.fold");
test_valid!(pleats_angle_3d, "tests/pleats-angle-3d.fold");
test_valid!(preliminary_offset_cp, "tests/preliminary-offset-cp.fold");
test_valid!(randlett_flapping_bird, "tests/randlett-flapping-bird.fold");
test_valid!(random_triangles_3d, "tests/random-triangles-3d.fold");
test_valid!(resch_tess, "tests/resch-tess.fold");
test_valid!(separated_parallel_edges, "tests/separated-parallel-edges.fold");
test_valid!(square_fish_3d, "tests/square-fish-3d.fold");
test_valid!(square_tube_with_overlap, "tests/square-tube-with-overlap.fold");
test_valid!(square_twist, "tests/square-twist.fold");
test_valid!(strip_weave_concave, "tests/strip-weave-concave.fold");
test_valid!(strip_weave, "tests/strip-weave.fold");
test_valid!(strip_with_angle, "tests/strip-with-angle.fold");
test_valid!(surrounded_square, "tests/surrounded-square.fold");
test_valid!(triangle_strip_2, "tests/triangle-strip-2.fold");
test_valid!(triangle_strip, "tests/triangle-strip.fold");
test_valid!(two_bird_cp, "tests/two-bird-cp.fold");
test_valid!(wavy_miura_no_faces, "tests/wavy-miura-no-faces.fold");
test_valid!(windmill, "tests/windmill.fold");
test_valid!(windmill_no_edges, "tests/windmill-no-edges.fold");
test_valid!(windmill_variations, "tests/windmill-variations.fold");
