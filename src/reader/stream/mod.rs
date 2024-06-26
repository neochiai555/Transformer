#[derive(Debug, Clone)]
pub struct Record{
    pub size: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct RecordList {
    list: Vec<Record>,
    current: usize
}

impl RecordList {
    pub fn new_instance() -> RecordList {
        let value = RecordList {list:vec![], current:0};
        value
    }

    pub fn add_record(self: &mut Self, a_size: usize, a_content: String) {
        let new_record = Record {size: a_size , content: a_content};
        self.list.push(new_record);
    }
}

impl Iterator for RecordList{
    type Item = Record;
    fn next(&mut self) -> Option<Record> {
        if (self.current < self.list.len()) {
            let return_value = &self.list[self.current];
            self.current += 1;
            return Some(return_value.clone())
        }
        None
    }
}