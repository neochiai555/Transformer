use transformer_lib::read_lines;
fn main() {
    let result_lines = read_lines("/tmp/test.txt");
    if let Ok(lines) = result_lines {
        for line in lines {
            println!("{:?}", line);
        }
    }
    /*
    match result_lines {
        Ok(lines) => {
            for line in lines {
                println!("{:?}", line);
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    } */
}