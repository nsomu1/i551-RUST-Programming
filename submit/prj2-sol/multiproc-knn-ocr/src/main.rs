use nix::unistd::{fork, ForkResult};
use nix::unistd::getpid;
use nix::unistd::Pid;
use std::path::Path;
use std::env;
use std::error::Error;
use knn_ocr::{read_labeled_data, knn};
use nix::unistd::pipe;




const TEST_DATA: &str = "t10k-images-idx3-ubyte";
const TEST_LABELS: &str  = "t10k-labels-idx1-ubyte";
const TRAINING_DATA: &str  = "train-images-idx3-ubyte";
const TRAINING_LABELS: &str  = "train-labels-idx1-ubyte";

fn usage(prog_path: &str, err: &str) -> ! {
    let prog_name =
Path::new(prog_path).file_name().unwrap().to_str().unwrap();
    eprintln!("{}\nusage: {} DATA_DIR [N_TESTS [K]]", err, prog_name);
    std::process::exit(1);
}
#[derive(Debug)]
#[allow(dead_code)]
struct Pipeio {
    intopipe: [i32; 2],
    outtopipe: [i32; 2]
   
}

struct Args {
    data_dir: String,
    k: usize,
    n_test: i32,
    n_proc: usize,
}

impl Args {
    fn new(argv: &Vec<String>) -> Result<Args, Box<dyn Error>> {
let argc = argv.len();
if argc < 2 || argc > 5 {
   Err("must be called with 1 ... 4 args")?;
}
let data_dir = argv[1].clone();
let mut n_test: i32 = -1;
let mut k: usize = 3;
let mut n_proc: usize = 4;
if argc > 2 {
   n_test = argv[2].parse::<i32>()?;
}
if argc == 4 {
   k = argv[3].parse::<usize>()?;
   if k == 0 {
Err("k must be positive")?;
   }
}
if argc ==5 {
   n_proc = argv[4].parse::<usize>()?;
   if n_proc == 0 {
Err("n_proc must be positive")?;
   }
}
Ok(Args { data_dir, k, n_test,n_proc })
    }
   
}

#[allow(dead_code)]
fn main() {
    let argv: Vec<String> = env::args().collect();
    let args;
    match Args::new(&argv) {
Err(err) => usage(&argv[0], &err.to_string()),
Ok(a) => args = a,
    };
    
    let train_data =
read_labeled_data(&args.data_dir, TRAINING_DATA, TRAINING_LABELS);
    let test_data = read_labeled_data(&args.data_dir, TEST_DATA, TEST_LABELS);
    let n: usize = if args.n_test <= 0 {
test_data.len()
    } else {
args.n_test as usize
    };
    let mut ok = 0;
    
     let mut process_vec: Vec<Pipeio> =Vec::with_capacity(args.n_proc);
    
     let pid_c:Pid = getpid();
     for _i in 0..args.n_proc{
      if pid_c == getpid(){
             let (in1, out1) = pipe().expect("file descriptor error");
             let (in2,out2) = pipe().expect("file descriptor error");
             process_vec.push(Pipeio{intopipe:[in1,out1],outtopipe:[in2,out2]});
             match unsafe{fork()} {
                Ok(ForkResult::Parent { child: _, .. }) => {
  }
  Ok(ForkResult::Child) => {
  
   break;
  },
  Err(_) => println!("Fork failed"),
}

      }
     }
     
     if pid_c == getpid(){
      let mut l = 0;
      for _i in 0..n {
        if l == args.n_proc {
           l = 0;

        }
       
        
           
                 
        l = l + 1;

      }
     } else {
     

     }
   
    
    for i in 0..n {
let nearest_index = knn(&train_data, &test_data[i].features, args.k);
let predicted = train_data[nearest_index].label;
let expected = test_data[i].label;
if predicted == expected {
   ok += 1;
}
else {
   let digits = b"0123456789";
   println!("{}[{}] {}[{}]",
    char::from(digits[predicted as usize]), nearest_index,
    char::from(digits[expected as usize]), i);
}
    }

    println!("{}% success", (ok as f64)/(n as f64)*100.0);
   

}
