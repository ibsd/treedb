use trees::{tr,Node};
use std::fmt::Display;

fn tree_to_string<T:Display>( node: &Node<T> ) -> String {
    if node.is_leaf() {
        node.data.to_string()
    } else {
        format!( "{}( {})", node.data, 
            node.iter().fold( String::new(),
                |s,c| s + &tree_to_string(c) + &" " ))
    }
}

fn main() {
    let tree = tr("wireless")
        /( tr("wifi-device") /tr("wifi0")/tr("wifi1") )
        /( tr("wifi-iface") /tr("2.3G")/tr("5G") );

    println!("{}", tree_to_string( &tree ));
}
