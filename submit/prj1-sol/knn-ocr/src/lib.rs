//remove once project is completed 

#![allow(dead_code)] 

#![allow(unused_variables, unused_imports)] 


use std::fs; 

use std::fs::File; 

use byteorder::BigEndian; 

use std::io::BufReader; 

use byteorder::ByteOrder; 

use std::cmp::Ordering; 

use std::collections::HashMap; 
  

type Feature = u8; 

type Label = u8; 

type Index = usize; 

type AxType = usize; 

pub struct LabeledFeatures { 

    ///feature set 

    pub features: Vec<Feature>, 

  

    ///classification of feature set 

    pub label: Label, 

} 

pub struct AuxSt { 

    pub label: AxType, 

    pub dist: AxType, 

    pub index: AxType, 

} 

  

///magic number used at start of MNIST data file 

const DATA_MAGIC: u32 = 0x803; 

  

///magic number used at start of MNIST label file 

const LABEL_MAGIC: u32 = 0x801; 

  

///return labeled-features with features read from data_dir/data_file_name// 

///and labels read from data_dir/label_file_name 

pub fn read_labeled_data(data_dir: &str, 

			 data_file_name: &str, label_file_name: &str) 

			 -> Vec<LabeledFeatures> 

{ 

// following line will be replaced during your implementation 

//concatenate directory and filename 

    let d_path=format!("{}/{}",data_dir,data_file_name); 

    let l_path=format!("{}/{}",data_dir,label_file_name); 

//Debuging 

	//println!("{}",d_path); 

	//println!("{}",l_path); 

//create and read variables
//store data from data and label file in vectors			     

     let mut data_vec = Vec::new(); 

     let mut label_vec = Vec::new(); 

     data_vec=fs::read(&d_path).expect("unable to read data");	 

     label_vec=fs::read(&l_path).expect("unable to read label");	 

//Success behavior of len()

     //println!("{}",data_vec.len()); 

     //println!("{}",label_vec.len()); 


//Data_Magic Number Checked

    
//assert_eq!(DATA_MAGIC, big_data_magic_label(&data_vec,0,4)); 

//Label_Magic Number Checked  

//assert_eq!(LABEL_MAGIC, big_data_magic_label(&label_vec,0,4)); 

//No. of Images Checked

	let mut n: usize = big_data_magic_label(&data_vec,4,8).try_into().unwrap(); 

	//assert_eq!(60000, n); 

	//assert_eq!(10000, n);
	
	//println!("Checking #{} of Images Successfully!",n); 

//No. of Labels Checked

	n = big_data_magic_label(&label_vec,4,8).try_into().unwrap(); 

	//assert_eq!(60000, n);

	//assert_eq!(10000, n);

	//println!("Checking #{} of Labels Successfully!",n); 


//number of Data_rows checked

	let mut data_rows: usize = big_data_magic_label(&data_vec,8,12).try_into().unwrap(); 

 	//assert_eq!(28, data_rows); 

	//println!("Checking #{} of Data_rows Successfully!",data_rows); 

//number of Data_columns checked

	let mut data_columns: usize = big_data_magic_label(&data_vec,12,16).try_into().unwrap(); 

//assert_eq!(28, data_columns); 

//println!("Checking #{} of Data_columns Successfully!",data_columns); 


// declaring variable for storing n labeled images 

     let mut image_size=(&data_rows*&data_columns) as usize; 

     let mut final_v = Vec::with_capacity(n); 

  
     let mut data_image = &data_vec[16..]; 

     let mut label_image = &label_vec[8..]; 

  
        for i in 0..n as usize { 

     let mut start=i*image_size; 

     let mut finish = start+image_size; 

     let image_vec=  &data_image[start..finish];  

     let dummy = LabeledFeatures { 

	features : image_vec.to_vec(), 

	label : label_image[i], 

	}; 

    	final_v.push(dummy); 

    } 
//final vector returned
    final_v 

    } 

//Function which returns the bytes in BigEndian Format. 

pub fn big_data_magic_label(n: &[u8],mut initial_byte: usize,mut final_byte: usize )-> u32{ 

 return BigEndian::read_u32(&n[initial_byte.. final_byte]); 

} 

///Return the index of an image in training_set which is among the k 

///nearest neighbors of test and has the same label as the most 

///common label among the k nearest neigbors of test. 

impl Eq for AuxSt {} 

impl Ord for AuxSt { 

    fn cmp(&self, other: &Self) -> Ordering { 

        self.dist.cmp(&other.dist) 

    } 

} 

impl PartialOrd for AuxSt { 

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 

        Some(self.cmp(other)) 

    } 

} 
impl PartialEq for AuxSt { 

    fn eq(&self, other: &Self) -> bool { 

        self.dist == other.dist 

    } 

} 
pub fn distance_square(tr_image: &Vec<Feature>,tt_image: &Vec<Feature>) 

							-> AxType 
{ 
let mut sqof_dist : usize; 
sqof_dist = 0; 
for i in 0..784 as usize { 
sqof_dist =  sqof_dist + ((tr_image[i] as isize - tt_image[i] as isize) *
		 (tr_image[i] as isize - tt_image[i] as isize))as usize ; 
} 
return sqof_dist
} 


pub fn knn(training_set: &Vec<LabeledFeatures>, test: &Vec<Feature>, k: usize) 

       -> Index 

{ 

let mut d: usize; 

let mut vector_slice = Vec:: new(); 

   for i in 0..training_set.len() 

     { 

	d = distance_square(&training_set[i].features,test); 
	let dummy = AuxSt { 
	dist : d, 
	label : training_set[i].label as usize, 
	index : i, 
	}; 
	vector_slice.push(dummy); 

     } 

vector_slice.sort_by_key(|d| d.dist); 

let mut aucvec = &vector_slice[..k] ; 

return vector_slice[0].index;
} 


