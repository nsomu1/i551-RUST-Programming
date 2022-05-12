use std::env;
use std::io::{Write, BufReader};
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::str;
use std::sync::Arc;
use std::fs;

mod request;
#[allow(unused_imports)]
use request::{Request, parse_request};
#[allow(unused_imports)]
use knn_ocr::{LabeledFeatures, read_labeled_data, knn};

const TRAINING_DATA: &str  = "train-images-idx3-ubyte";
const TRAINING_LABELS: &str  = "train-labels-idx1-ubyte";


fn main() {
    let argv: Vec<String> = env::args().collect();
    let args;
    match Args::new(&argv) {
	Err(err) => usage(&argv[0], &err.to_string()),
	Ok(a) => args = Arc::new(a),
    };
    //TODO: spin up TCP/IP server
    let training_set = read_labeled_data(&args.data_dir, TRAINING_DATA, TRAINING_LABELS);
    println!("length of training data : {}",training_set.len());
    host_connect(args);
}

//TODO: add functions as necessary

/*********************** Command-Line Arguments ************************/

fn usage(prog_path: &str, err: &str) -> ! {
    let prog_name =
	Path::new(prog_path).file_name().unwrap().to_str().unwrap();
    eprintln!("{}\nusage: {} PORT DATA_DIR ROOT_HTML_PATH [K]", err, prog_name);
    std::process::exit(1);
}

#[derive(Debug)]
struct Args {
    port: u16,
    data_dir: String,
    index_path: String,
    k: usize,
}
fn host_connect(args: Arc<Args>) {
    let host = format!("127.0.0.1:{}", args.port);
    let binding = TcpListener::bind(&host).unwrap();
    loop {
	match binding.accept() {
	    Ok((client, host)) => {
		println!("connection successful : {:?}", host);
		handler_fn(client, &args);
	    },
	    Err(e) => eprintln!("connection error: {:?}", e),
	}
    }
}
impl Args {
    fn new(argv: &Vec<String>) -> Result<Args, Box<dyn Error>> {
	let argc = argv.len();
	if argc < 4 || argc > 5 {
	    Err("must be called with 3 ... 4 args")?;
	}
	let port = argv[1].parse::<u16>()?;
	if port < 1024 	{
	    Err("port must be greater than 1023")?;
	}
	let data_dir = argv[2].clone();
	let index_path = argv[3].clone();
	let mut k: usize = 3;
	if argc == 5 {
	    k = argv[4].parse::<usize>()?;
	    if k == 0 {
		Err("k must be positive")?;
	    }
	}
	Ok(Args { port, data_dir, index_path, k })
    }
    
}
fn handler_fn(mut stream: TcpStream, args: &Arc<Args>) {
    loop {

    	let clone = stream.try_clone().expect("unexpected error while cloning...");
    	let bufreader = BufReader::new(clone);
    	match parse_request(bufreader) {
	    Ok(request) => {
	    	
	    	if request.method == "GET" && request.path == "/"
    		{
        		let index = fs::read_to_string(&args.index_path).unwrap();

        		let ack = format!(
            			"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            			index.len(),
            			index
        		);

        		stream.write(ack.as_bytes()).unwrap();
        		stream.flush().unwrap();
    		}
    		else if request.method == "POST" && request.path == "/ocr" 
    		{
    		 	let testing_data_set = request.body;
			let training_data_set = read_labeled_data(&args.data_dir, TRAINING_DATA, TRAINING_LABELS);
			    		 	
    		 	
			let index_knn = knn(&training_data_set, &testing_data_set, args.k);
			let predicted_label = training_data_set[index_knn].label;
			
			
			
        		let ack = format!(
            			"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 3\r\n\r\n{}\r\n",
            			predicted_label
        		);

        		stream.write(ack.as_bytes()).unwrap();
        		stream.flush().unwrap();
    		}
    		else
    		{
    		  	let not_found = "HTTP/1.1 404 NOT FOUND";

        		let ack = format!(
            			"{}\r\n",
            			not_found
        		);

        		stream.write(ack.as_bytes()).unwrap();
        		stream.flush().unwrap();
    		}
	    }
	    Err(e) => panic!("{}", e),
	}
    }
}
