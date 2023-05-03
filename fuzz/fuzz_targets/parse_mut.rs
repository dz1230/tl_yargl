#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate tl_yargl;

fuzz_target!(|data: &str| {
    let mut dom = tl_yargl::parse(data, tl_yargl::ParserOptions::default()).unwrap();

    // ... some random DOM mutations ...
    for node in dom.nodes_mut() {
        if let Some(tag) = node.as_tag_mut() {
            tag.attributes_mut()
                .insert_attribute("test", Some("testing"));

            tag.inner_html_mut()
                .set("<b>Hello World</b>".as_bytes())
                .unwrap();
        }
    }
});
