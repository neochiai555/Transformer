use transformer_lib::read_lines;
fn main() {
    let result_lines = read_lines("/tmp/test.txt");
    match result_lines {
        Ok(lines) => {
            let mut iterator = lines.into_iter();
            for line in iterator.by_ref() {
                println!("{:?}", line);
            }
            iterator.reset();
            for line in iterator.by_ref() {
                println!("{:?}", line);
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}