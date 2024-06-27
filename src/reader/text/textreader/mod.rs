macro_rules! return_next_item {
    ($arg: ident) => {
        if $arg.current < $arg.records.list.len() {
            let return_value = &$arg.records.list[$arg.current];
            $arg.current += 1;
            return Some(return_value.clone())
        }
        return None
    };
}

#[derive(Debug, Clone)]
pub struct TextRecord {
    pub size: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct TextRecordList {
    list: Vec<TextRecord>
}

#[derive(Debug)]
pub struct TextRecordListIterator {
    records: TextRecordList,
    current: usize
}
#[derive(Debug)]
pub struct TextRecordListIntoIterator {
    records: TextRecordList,
    current: usize
}

impl TextRecordListIntoIterator {
    pub fn reset(self: &mut Self) {
        self.current = 0
    }
}
impl TextRecordList {
    pub fn new_instance() -> TextRecordList {
        let value = TextRecordList {list:vec![]};
        value
    }
    pub fn add_record(self: &mut Self, a_record: &TextRecord) {
        let new_record = TextRecord {size: a_record.size , content: a_record.content.clone()};
        self.list.push(new_record);
    }
}

impl Iterator for TextRecordListIterator {
    type Item = TextRecord;
    fn next(&mut self) -> Option<TextRecord> {
        return_next_item!(self);
    }
}

impl Iterator for TextRecordListIntoIterator {
    type Item = TextRecord;

    fn next(&mut self) -> Option<TextRecord> {
        return_next_item!(self);
    }
}

impl IntoIterator for TextRecordList {
    type Item = TextRecord;
    type IntoIter = TextRecordListIntoIterator;

    fn into_iter(self) -> TextRecordListIntoIterator {
        TextRecordListIntoIterator{records: self, current: 0}
    }
}
