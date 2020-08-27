fn main() {
    let n = Node { cat: "shit", 
                   node: Option::Some(&Node {cat: "deep", 
                                             node: Option::None})};
    df_cats(&n);
}

struct Node<'a> {
    cat: &'a str,
    node: Option<&'a Node<'a>>,
    //right: &'a Option<Node<'a>>
}

fn df_cats(node: &Node) {
    let mut node = node;
    loop {
        println!("Category is {}", node.cat);
        if !has_child(node) {
            break;
        }
        node = node.node.unwrap();
    }
    println!("that's it!");
}

fn has_child(node: &Node) -> bool {
    match node.node {
        Some(_) => true,
        None => false
    }
}

