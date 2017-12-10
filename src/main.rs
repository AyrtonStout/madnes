fn main() {
	println!("Hello World!");
	dude();
}

fn dude() {
	println!("Wat");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
