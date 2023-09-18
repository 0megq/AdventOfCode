use std::fs;

#[derive(Debug)]
enum Node {
    File(usize),
    Dir(Vec<usize>, usize),
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut nodes: Vec<Node> = Vec::new();

    let mut directories: Vec<usize> = Vec::new();

    let mut current_dir_index = 0;

    //Creating all the nodes
    for line in input.lines() {
        if line[0..1].eq("$") {
            match &line[2..] {
                "cd /" => {
                    nodes.push(Node::Dir(Vec::new(), 0));
                    directories.push(0);
                }
                "cd .." => {
                    if let Node::Dir(_children, parent) = &nodes[current_dir_index] {
                        current_dir_index = *parent;
                    }
                }
                "ls" => (),
                _ => {
                    let new_dir_index = nodes.len();
                    if let Node::Dir(children, _parent) = &mut nodes[current_dir_index] {
                        children.push(new_dir_index);
                        nodes.push(Node::Dir(Vec::new(), current_dir_index));
                        directories.push(new_dir_index);
                        current_dir_index = new_dir_index;
                    }
                }
            }
            continue;
        } else if line[0..3].eq("dir") {
            ()
        } else {
            let file_size: usize = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let new_file_index = nodes.len();
            if let Node::Dir(children, _parent) = &mut nodes[current_dir_index] {
                children.push(new_file_index);
                nodes.push(Node::File(file_size));
            }
        }
    }

    let mut sum: usize = 0;

    for directory in directories {
        if get_node_size(&nodes[directory], &nodes) <= 100000 {
            sum += get_node_size(&nodes[directory], &nodes);
        }
    }

    println!("{}", sum);
}

fn get_node_size(node: &Node, nodes: &Vec<Node>) -> usize {
    match node {
        Node::File(size) => return *size,
        Node::Dir(children, _parent) => {
            let mut size: usize = 0;
            for child in children {
                size += get_node_size(&nodes[*child], nodes);
            }

            return size;
        }
    }
}
