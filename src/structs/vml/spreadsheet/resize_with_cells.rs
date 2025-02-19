use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::TrueFalseBlankValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ResizeWithCells {
    value: TrueFalseBlankValue,
}
impl ResizeWithCells {
    pub fn get_value(&self) -> &Option<bool> {
        self.value.get_value()
    }

    pub fn set_value(&mut self, value: bool) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        empty_flag: bool,
    ) {
        if empty_flag {
            return;
        }
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Text(e)) => {
                    self.value
                        .set_value_string(e.unescape_and_decode(reader).unwrap());
                }
                Ok(Event::End(ref e)) => match e.name() {
                    b"x:SizeWithCells" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "x:SizeWithCells"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:SizeWithCells
        if self.value.has_value() {
            write_start_tag(writer, "x:SizeWithCells", vec![], false);
            write_text_node(writer, self.value.get_value_string2());
            write_end_tag(writer, "x:SizeWithCells");
        } else {
            write_start_tag(writer, "x:SizeWithCells", vec![], true);
        }
    }
}
