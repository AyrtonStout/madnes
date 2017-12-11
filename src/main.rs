use std::io::prelude::*;
use std::fs::File;

fn main() {
	println!("Hello World!");
	read_file();
}

fn read_file() {
	println!("Wat");
    let mut buffer = vec![0; 10];
//    let mut file = File::open("Super Mario Bros 3 (E).nes").expect("Bad things");
    let mut file = File::open("Contra (USA).nes").expect("Bad things");

    file.read_to_end(&mut buffer).expect("More bad things");

    print!("{}", buffer[10]); // This is the first 'N' in 'N' 'E' 'S' 'EOF'
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
