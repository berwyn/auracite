use xml::writer::{EventWriter, XmlEvent};

/// Writes a simple XML element to the provided buffer
///
/// # Examples
///
/// ```rust
/// write_simple_xml(w, "title", "My Cool News Feed");
/// //=> <title>My Cool News Feed</title>
/// ```
pub fn write_simple_xml(w: &mut EventWriter<&mut Vec<u8>>, tag: &str, body: &str) {
    w.write(XmlEvent::start_element(tag)).unwrap();
    w.write(body).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
}
