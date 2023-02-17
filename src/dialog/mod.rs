pub mod ask_about;
pub use ask_about::*;
pub mod body;
pub use body::*;
pub mod interchange;
pub use interchange::*;
pub mod reply_function;
pub use reply_function::*;
pub mod reply;
pub use reply::*;
pub mod text_command;
pub use text_command::*;

use crate::*;
use serde::{Serialize, ser::SerializeStruct, Serializer, Deserialize, Deserializer, de::Visitor, de::SeqAccess, de};
use std::fs::*;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Dialog {
    interchanges: Vec<Interchange>,
    askabouts: Vec<AskAbout>,
    askabout_top_text: String
}

impl Dialog {
    pub fn new(interchanges: Vec<Interchange>, askabouts: Vec<AskAbout>, askabout_top_text: String) -> Self {
        Dialog {
            interchanges,
            askabouts,
            askabout_top_text
        }
    }

    pub fn interchange_count(&self) -> usize {
        self.interchanges.len()
    }

    pub fn find_and_replace(&mut self, target: &str, replacement: &str) {
        for interchange in &mut self.interchanges {
            interchange.find_and_replace(target, replacement);
        }
        for askabout in &mut self.askabouts {
            askabout.find_and_replace(target, replacement);
        }
        self.askabout_top_text = self.askabout_top_text.replace(target, replacement);
    }

    pub fn add_interchange(&mut self, interchange: Interchange) {
        self.interchanges.push(interchange);
    }

    pub fn add_empty_interchange(&mut self) {
        self.add_interchange(Interchange::default());
    }

    pub fn add_item_consumption_sequence(&mut self, mut count: usize, interchange_item_check: Interchange, interchange_item_missing: Interchange) {
        while count != 0 {
            let mut new_interchange = interchange_item_check.clone();
            new_interchange.find_and_replace("{}", &count.to_string());
            self.interchanges.push(new_interchange);

            let mut new_interchange = interchange_item_missing.clone();
            new_interchange.find_and_replace("{}", &count.to_string());
            self.interchanges.push(new_interchange);

            count -= 1;
        }
    }

    pub fn add_password_entry_sequence(&mut self, body: Body, options: &[&str], correct_sequence: &[usize]) {
        for i in 0..correct_sequence.len() {
            if i != 0 {
                // Sad path
                let mut replies = Vec::with_capacity(options.len());
                for j in 0..options.len() {
                    let destination_interchange = self.interchange_count() as i32 + 2;
                    replies.push(Reply::continue_to(destination_interchange, options[j].to_owned(), Cmd::none()));
                }
                self.interchanges.push(Interchange::new(body.clone(), replies));
            }

            // Happy path
            let mut replies = Vec::with_capacity(options.len());
            for j in 0..options.len() {
                let destination_interchange = self.interchange_count() as i32 + if i == j { 2 } else { 1 };
                replies.push(Reply::continue_to(destination_interchange, options[j].to_owned(), Cmd::none()));
            }
            self.interchanges.push(Interchange::new(body.clone(), replies));
        }
        self.interchanges.push(Interchange::placeholder(String::from("FAILURE")));
        self.interchanges.push(Interchange::placeholder(String::from("SUCCESS")));
    }

    // TODO: Remove this crap.
    // pub fn to_dia(&self, filename: &str) -> Result<(), Error> {
    //     let file = File::create(format!("{}.dia", filename)).map_err(Error::InputOutput)?;
    //     serde_blitz3d::to_writer(file, self).map_err(Error::Serde)
    // }

    pub fn to_dia(&mut self, filename: &str) -> Result<(), Error> {
        let file = File::create(format!("{}.dia", filename)).map_err(Error::InputOutput)?;
        //file.write();
        let mut writer = BlitzWriter::new(file);
        writer.read_or_write(self);
        Ok(())
    }

    pub fn from_dia<P: AsRef<Path>>(path: P) -> Result<Object, Error> {
        let file = File::open(path).map_err(Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Error::Serde)
    }
}

impl Serialize for Dialog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut dialog = serializer.serialize_struct("Dialog", 3)?;
        //dialog.serialize_i32(0);
        dialog.serialize_field("interchanges", &self.interchanges)?;
        dialog.end()
    }
}

impl<'de> Deserialize<'de> for Dialog {
    fn deserialize<D>(deserializer: D) -> Result<Dialog, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_struct("Dialog", &["interchanges", "askabouts", "askabout_top_text"], DialogVisitor)
    }
}

pub struct DialogVisitor;

impl<'de> Visitor<'de> for DialogVisitor {
    type Value = Dialog;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "struct Dialog")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>
    {
        let interchange_count: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let mut interchanges = vec![];
        for _ in 0..interchange_count {
            let interchange = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            interchanges.push(interchange);
        }
        let askabout_count: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let askabout_top_text: String = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let mut askabouts = vec![];
        for _ in 0..askabout_count {
            let askabout = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            askabouts.push(askabout);
        }
        Ok(Dialog::new(interchanges, askabouts, askabout_top_text))
    }
}

// NEW writer implementation:

pub trait ReadOrWrite<T>
where
    T: ?Sized
{
    fn read_or_write(&mut self, object: &mut T);
}

pub trait ReadOrWriteCollection<T> {
    fn read_or_write_len(&mut self, object: &mut T);
    fn read_or_write_contents(&mut self, object: &mut T);
}

// pub struct BlitzReader<R> {
//     reader: R
// }

pub struct BlitzWriter<W> {
    writer: W
}

impl<W> BlitzWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer
        }
    }
}

impl<W> ReadOrWrite<i32> for BlitzWriter<W>
where
    W: Write
{
    fn read_or_write(&mut self, object: &mut i32) {
        self.writer.write(&object.to_le_bytes()).unwrap();
    }
}

impl<W> ReadOrWrite<usize> for BlitzWriter<W>
where
    W: Write
{
    fn read_or_write(&mut self, object: &mut usize) {
        self.writer.write(&(*object as i32).to_le_bytes()).unwrap();
    }
}

impl<W> ReadOrWrite<String> for BlitzWriter<W>
where
    W: Write
{
    fn read_or_write(&mut self, object: &mut String) {
        self.read_or_write(&mut object.len());
        self.writer.write(&object.as_bytes()).unwrap();
    }
}

impl<T, W> ReadOrWrite<[T]> for BlitzWriter<W>
where
    W: Write,
    Self: ReadOrWrite<T> + ReadOrWrite<usize>
{
    fn read_or_write(&mut self, object: &mut [T]) {
        self.read_or_write(&mut object.len());
        for element in object {
            self.read_or_write(element);
        }
        //self.writer.write(&object.as_bytes()).unwrap();
    }
}

// impl<T, W> ReadOrWriteCollection<Vec<T>> for BlitzWriter<W>
// where
//     W: Write
// {

// }

impl<W> ReadOrWrite<Dialog> for W
where
    W: ReadOrWrite<i32> + ReadOrWrite<usize> + ReadOrWrite<String> + ReadOrWrite<[String]>
{
    // Only built for writing at the moment...
    fn read_or_write(&mut self, dialog: &mut Dialog) {
        self.read_or_write(&mut dialog.interchanges.len());
        for interchange in &mut dialog.interchanges {
            //self.read_or_write(&mut interchange.body_text_lines().len());
            self.read_or_write(interchange.body_text_lines_mut());

            let commands = interchange.body_text_commands();
            self.read_or_write(&mut commands.len());
            for command in commands {
                self.read_or_write(&mut command.name().to_owned());
                self.read_or_write(&mut command.position());
            }

            let replies = interchange.replies();
            self.read_or_write(&mut replies.len());
            for reply in replies {
                self.read_or_write(&mut reply.text().to_owned());
                let reply_fnc = reply.fnc();
                self.read_or_write(&mut reply_fnc.id());
                self.read_or_write(&mut reply_fnc.data());
                let mut cmd = reply.cmd().to_owned();
                self.read_or_write(&mut cmd.id);
                self.read_or_write(&mut cmd.data1);
                self.read_or_write(&mut cmd.data2);
                self.read_or_write(&mut cmd.data3);
                self.read_or_write(&mut cmd.data4);
            }
        }

        let askabouts = &mut dialog.askabouts;
        self.read_or_write(&mut askabouts.len());
        self.read_or_write(&mut dialog.askabout_top_text);
        for askabout in askabouts {
            self.read_or_write(&mut askabout.text);
            self.read_or_write(&mut askabout.active);
            self.read_or_write(&mut askabout.interchange);
            self.read_or_write(&mut askabout.repeat);
        }
    }
}