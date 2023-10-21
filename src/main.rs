use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

#[derive(Debug)]
struct Files {
    path: String,
    agency: String,
    routes: String,
    shapes: String,
    stops: String,
    stop_times: String,
    trips: String,
}
#[derive(Debug)]
struct Columns {
    agency: Vec<String>,
    routes: Vec<String>,
    shapes: Vec<String>,
    stops: Vec<String>,
    stop_times: Vec<String>,
    trips: Vec<String>,
}

impl Files {
    fn new() -> Files {
        print!("Enter the path to the directory where your GTFS files are stored: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        println!("You entered: {}", input);

        Files {
            path: input.to_string(),
            agency: format!("{}/agency.txt", input),
            routes: format!("{}/routes.txt", input),
            shapes: format!("{}/shapes.txt", input),
            stops: format!("{}/stops.txt", input),
            stop_times: format!("{}/stop_times.txt", input),
            trips: format!("{}/trips.txt", input),
        }
    }

    fn get_columns(&self) -> Columns {
        Columns {
            agency: get_first_lines(&self.agency),
            routes: get_first_lines(&self.routes),
            shapes: get_first_lines(&self.shapes),
            stops: get_first_lines(&self.stops),
            stop_times: get_first_lines(&self.stop_times),
            trips: get_first_lines(&self.trips),
        }
    }
}

fn main() {
    let files = Files::new();
    let columns = files.get_columns();
    println!("{:#?}", columns);
}

fn get_first_lines(path: &String) -> Vec<String> {
    let file = File::open(path);

    let reader = match file {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Could not locate the file! Error: {:?}", e),
    };

    let split: Vec<String> = match reader.lines().next() {
        Some(Ok(content)) => content
            .split(',')
            .map(|s| {
                let mut new_string = s.to_string();
                new_string.retain(|f| f != '"' && f != '\\');
                new_string
            })
            .collect(),
        Some(Err(e)) => panic!("Error reading line: {}", e),
        None => panic!("File is empty or couldn't read the first line."),
    };

    split
}
