//#![allow(dead_code)]
//#![allow(unused_variables)]


use std::fs;

//this may result in non-deterministic behavior
//use std::collections::HashMap;

//use this for deterministic behavior
use hash_hasher::HashedMap;

type Feature = u8;
type Label = u8;
type Dist = i32;
type Index = usize;

fn be_bytes_to_u32(bytes: &[u8]) -> u32 {
    ((bytes[0] as u32) << 24) |
    ((bytes[1] as u32) << 16) |
    ((bytes[2] as u32) << 8) |
    ((bytes[3] as u32) << 0)
}

pub struct LabeledFeatures {
    ///feature set
    pub features: Vec<Feature>,

    ///classification of feature set
    pub label: Label,
}

///magic number used at start of MNIST data file
const DATA_MAGIC: u32 = 0x803;

///magic number used at start of MNIST label file
const LABEL_MAGIC: u32 = 0x801;

///return labeled-features with features read from data_dir/data_file_name
///and labels read from data_dir/label_file_name
pub fn read_labeled_data(data_dir: &str,
			 data_file_name: &str, label_file_name: &str)
			 -> Vec<LabeledFeatures>
{
    let data_path = format!("{}/{}", data_dir, data_file_name);
    let label_path = format!("{}/{}", data_dir, label_file_name);
    let data_bytes = fs::read(&data_path)
	.expect(&format!("unable to read {}", data_path));
    let label_bytes = fs::read(&label_path)
	.expect(&format!("unable to read {}", label_path));
    assert_eq!(be_bytes_to_u32(&data_bytes), DATA_MAGIC);
    assert_eq!(be_bytes_to_u32(&label_bytes), LABEL_MAGIC);
    let n_images = be_bytes_to_u32(&data_bytes[4..8]) as usize;
    let n_labels = be_bytes_to_u32(&label_bytes[4..8]) as usize;
    assert_eq!(n_images, n_labels);
    let n = n_images;
    let n_rows = be_bytes_to_u32(&data_bytes[8..12]) as usize;
    let n_cols = be_bytes_to_u32(&data_bytes[12..16]) as usize;
    let n_pixels = n_rows * n_cols;
    let data_start: usize = 16;
    let label_start: usize = 8;
    let mut results = Vec::with_capacity(n as usize);
    for i in 0..n {
	let start = data_start + i*n_pixels;
	let end = start + n_pixels;
	let pixels_slice = &data_bytes[start..end];
	let mut features = Vec::with_capacity(n_pixels);
	features.extend_from_slice(pixels_slice);
	let label = label_bytes[label_start + i];
	results.push(LabeledFeatures { features, label });
    }
    return results;
}

///return square of cartesian distance between coord1 and coord2.
fn distance2(coords1: &[Feature], coords2: &[Feature]) -> Dist {
    assert_eq!(coords1.len(), coords2.len());
    let mut dist2 = 0 as Dist;
    for i in 0..coords1.len() {
	let x1 = coords1[i] as Dist;
	let x2 = coords2[i] as Dist;
	dist2 += (x1 - x2) * (x1 - x2);
    }
    dist2
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct DistLabel {
    /// square of distance of this data point from test data point
    dist_sq: Dist,
    /// label for this data point
    label: Label,
    /// index of this data point
    index: Index,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct LabelIndex(Label, Index);

fn mode(labels_with_index: &[LabelIndex]) -> &LabelIndex {
    //let mut counts = HashMap::new();
    let mut counts = HashedMap::default();
    for &label_index in labels_with_index {
	*counts.entry(label_index.0).or_insert(0) += 1;
    }
    let (label, _) = counts.into_iter().max_by_key(|&(_, count)| count)
	.expect("must have at least one label for k > 0");
    labels_with_index.iter().find(|&x| x.0 == label).expect("label must exist")
}

///Return the index of an image in training_set which is among the k
///nearest neighbors of test and has the same label as the most
///common label among the k nearest neigbors of test.
pub fn knn(training_set: &Vec<LabeledFeatures>, test: &Vec<Feature>, k: usize)
       -> Index
{
    let n = training_set.len();
    let mut dists: Vec<DistLabel> = Vec::with_capacity(n);
    for index in 0..n {
	let dist_sq = distance2(&training_set[index].features, test);
	let label = training_set[index].label;
	dists.push(DistLabel { dist_sq, label, index });
    }
    dists.sort();
    let label_indexes = dists[0..k]
	.iter()
	.map(|x| LabelIndex(x.label, x.index))
	.collect::<Vec<LabelIndex>>();
    mode(&label_indexes).1
}

