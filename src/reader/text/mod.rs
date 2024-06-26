use std::io::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::reader::stream::RecordList;

pub fn read_lines<P>(filename: P) -> Result<RecordList , Error>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut result = RecordList::new_instance();
    for line in lines.flatten() {
        result.add_record(1, line);
    }
    Ok(result)
}