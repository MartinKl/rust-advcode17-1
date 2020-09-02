#[cfg(test)]
mod tests {
    use super::*;

    fn assert_none(value: Option<&Node>) {
        match value {
            Some(_) => panic!("Value {:?} is not None!", value),
            None => {}
        };
    }

    #[test]
    fn parse_binary_1() {
        let spec = "(CU (SUBJ (I)) (VP (V (am)) (ADJ (stupid))))";
        let node = match parse(spec) {
            None => panic!("Test failed!"),
            Some(n) => n
        };
        assert_eq!("CU", node.get_category());
        let l = node.get_left().unwrap();
        {
            assert_eq!("SUBJ", l.get_category());
            let ll = l.get_left().unwrap();
            {
                assert_eq!("I", ll.get_category());
                assert_none(ll.get_left());
                assert_none(ll.get_right());
            }
        }
        let r = node.get_right().unwrap();
        {
            assert_eq!("VP", r.get_category());
            let rl = r.get_left().unwrap();
            {
                assert_eq!("V", rl.get_category());
                let rll = rl.get_left().unwrap();
                {
                    assert_eq!("am", rll.get_category());
                }
                assert_none(rl.get_right());
            }
            let rr = r.get_right().unwrap();
            {
                assert_eq!("ADJ", rr.get_category());
                let rrl = rr.get_left().unwrap();
                {
                    assert_eq!("stupid", rrl.get_category());
                }
                assert_none(rrl.get_right());
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_trinary() {
        let spec = "(CU (SUBJ (I)) (V (am)) (ADJ (stupid)))";
        parse(spec);
    }
}

fn main() {
    parse("(A (B) (C))");
}

struct Node {
    cat: String,  // TODO change to &'a str
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl<'a> Node {
    fn get_left(&'a self) -> Option<&'a Node> {
        match &self.left {
            None => None,
            Some(b) => Some(&(*b))
        }
    }

    fn get_right(&'a self) -> Option<&'a Node> {
        match &self.right {
            None => None,
            Some(b) => Some(&(*b))
        }
    }

    fn get_category(&'a self) -> &'a str {
        &self.cat
    }
}

use std::collections::HashMap;
use std::collections::HashSet;

fn parse(spec: &str) -> Option<Node> {
    const NODE_START: char = '(';
    const NODE_END: char = ')';
    const ERR_STATE: &'static str = "Invalid parser state: Tree might not be binary.";
    let mut stack = Vec::new();
    let mut queue = HashSet::new();
    let mut chars = String::new();
    let mut o: isize = 0;
    for c in spec.chars() {
        if c == NODE_START || c == NODE_END {
            if !stack.is_empty() && !chars.trim().is_empty() {
                let mut pt: (isize, isize, String) = stack.pop().unwrap();
                pt.2.push_str(&chars.trim());
                chars.clear();
                stack.push(pt);
            }
            o += 1;
            if c == NODE_START {
                stack.push((o, -1, String::new()));
            } else {
                let mut nt = stack.pop().unwrap();
                nt.1 = o;
                queue.insert(nt);
            }
        } else {
            chars.push(c);
        };
    }
    let mut pre = HashMap::<&isize, (&isize, &String)>::new();
    let mut post = HashMap::<&isize, &isize>::new();
    let mut build_order = Vec::<&isize>::new();
    let mut built = HashSet::<&isize>::new();
    for (p, p_, cat) in &queue {
        pre.insert(p, (p_, cat));
        post.insert(p_, p);
        if p_ - p == 1 {
            build_order.push(p);
            built.insert(p);
        };
    }
    build_order.sort();
    build_order.reverse();
    let mut candidates;
    while built.len() < queue.len() {
        candidates = Vec::new();
        for order_val in pre.keys() {
            if !built.contains(order_val) {
                if built.contains(&(*order_val + 1)) {
                    candidates.push(order_val);
                };
            };
        }
        candidates.sort();
        // nodes need to be built in descending order (highest pre-order first)
        // to guarantee that all children already exist, when the parent is constructed
        candidates.reverse();
        for c in candidates {
            build_order.push(c);
            built.insert(c);
        }
    };
    let mut pre_to_node = HashMap::new();
    let mut root = None;
    for p in build_order {
        let (p_, cat) = pre.remove(p).unwrap();
        let left: Option<Box<Node>> = match (&pre_to_node).contains_key(&(*p + 1)) {
            false => None,
            true => Some(Box::new(pre_to_node.remove(&(p + 1)).unwrap()))
        };
        let rp_opt = post.get(&(p_ - 1));
        let right: Option<Box<Node>> = match rp_opt {
            None => None,
            Some(rp) => match *rp - &1 - p { 
                0 => None,  // node will only have one child
                _ => Some(Box::new(pre_to_node.remove(rp).unwrap()))
            }
        };
        let node = Node {cat: cat.to_string(), left: left, right: right};
        if p > &1 {
            pre_to_node.insert(p, node);
        } else {
            root = Some(node);
        }
    }
    if !pre_to_node.is_empty() {
        panic!(ERR_STATE);
    }
    root
}   