use std::io::{Cursor};
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub fn write_simple_xml(w: &mut EventWriter<&mut Vec<u8>>, tag: &str, body: &str) {
    w.write(XmlEvent::start_element(tag)).unwrap();
    w.write(body).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
}

pub trait RSSChannelItem {
    fn write_xml(&self, w: &mut EventWriter<&mut Vec<u8>>);
}

pub struct RSSChannel {
    pub title: String,
    pub description: String,
    pub link: String,
    pub ttl: u32,
    pub items: Vec<Box<RSSChannelItem>>,
}

pub struct RSS {
    pub channel: RSSChannel,
}

impl RSS {
    fn build_body(&self) -> String {
        let mut target: Vec<u8> = Vec::new();
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&mut target);

        writer.write(XmlEvent::start_element("rss").attr("version", "2.0")).unwrap();
        self.write_channel(&mut writer);
        writer.write(XmlEvent::end_element()).unwrap();

        String::from_utf8(writer.into_inner().to_vec()).unwrap()
    }

    fn write_channel(&self, w: &mut EventWriter<&mut Vec<u8>>) {
        w.write(XmlEvent::start_element("channel")).unwrap();

        write_simple_xml(w, "title", self.channel.title.as_str());
        write_simple_xml(w, "description", self.channel.description.as_str());
        write_simple_xml(w, "link", self.channel.link.as_str());
        write_simple_xml(w, "ttl", self.channel.ttl.to_string().as_str());

        for item in self.channel.items.iter() {
            w.write(XmlEvent::start_element("item")).unwrap();
            item.write_xml(w);
            w.write(XmlEvent::end_element()).unwrap();
        }

        w.write(XmlEvent::end_element()).unwrap();
    }
}


impl<'r> Responder<'r> for RSS {
    fn respond(self) -> Result<Response<'r>, Status> {
        Response::build()
            .header(ContentType::new("text", "rss+xml"))
            .sized_body(Cursor::new(self.build_body()))
            .ok()
    }
}
