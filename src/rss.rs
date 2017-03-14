use std::io::{Cursor};

use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};
use xml_ext::{write_simple_xml};

/// A trait allowing arbitrary items can be serialised into the feed
pub trait RSSChannelItem {
    /// Write the element into the buffer
    fn write_xml(&self, w: &mut EventWriter<&mut Vec<u8>>);
}

/// A channel of news items that the feed should contain
pub struct RSSChannel {
    /// The name of the feed or site
    pub title: String,
    /// A short tagline describing the feed/site
    pub description: String,
    /// The homepage URL of the feed/site
    pub link: String,
    /// Time in seconds the client should wait to refresh items
    pub ttl: u32,
    /// A list of news items to show in the feed
    pub items: Vec<Box<RSSChannelItem>>,
}

/// The root object of an RSS feed
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
