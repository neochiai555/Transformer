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
pub struct TextRecordListIterator<'a> {
    records: &'a TextRecordList,
    current: usize
}
#[derive(Debug)]
pub struct TextRecordListIntoIterator {
    records: TextRecordList,
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
    pub fn iter(&self) -> TextRecordListIterator {
        TextRecordListIterator {
            records: self,
            current: 0,
        }
    }
}

impl<'a> TextRecordListIterator<'a> {
    pub fn reset(self: &mut Self) {
        self.current = 0;
    }
}

impl <'a>Iterator for TextRecordListIterator<'a> {
    type Item = &'a TextRecord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.records.list.len() {
            let return_value = Some(&self.records.list[self.current]);
            self.current += 1;
            return return_value
        }
        return None
    }
}

impl Iterator for TextRecordListIntoIterator {
    type Item = TextRecord;

    fn next(&mut self) -> Option<TextRecord> {
        if self.records.list.len() == 0 {
            return None;
        }
        let result = self.records.list.remove(0);
        Some(result)
    }
}

impl IntoIterator for TextRecordList {
    type Item = TextRecord;
    type IntoIter = TextRecordListIntoIterator;

    fn into_iter(self) -> TextRecordListIntoIterator {
        TextRecordListIntoIterator{records: self}
    }
}
