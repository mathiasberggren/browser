use html5ever::{namespace_url, ns, LocalName, QualName};
use markup5ever_rcdom::{Handle, NodeData};

// Right now, let's just print to terminal.
// We will change this once we have a layout engine.
pub fn print_to_terminal(dom: &Handle) -> () {
    let node = dom;

    let link_name = QualName::new(None, ns!(html), LocalName::from("href"));
    match node.data {
        NodeData::Document => println!("Document yo"),
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if name.local == LocalName::from("a") {
                println!("name: {}", name.local);
            }
            for attr in attrs.borrow().iter() {
                if attr.name.local == link_name.local {
                    println!("Name: {}", attr.name.local);
                    println!("Link: {}", attr.value);
                }
            }
        },
        _ => {}
    }
    for child in node.children.borrow().iter() {
        print_to_terminal(child);
    }
}
