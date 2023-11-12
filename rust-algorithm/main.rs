use std::fs::File;
use std::io::{BufRead, BufReader, self};
use std::collections::{VecDeque, LinkedList, HashSet};
use std::process::exit;

struct Node {
    name: String,
    children: Vec<usize>,
    height: u32,
    parent: i32,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Tree {
        let nodes: Vec<Node> = Vec::new();
        Tree { nodes }
    }

    fn read_tree(&mut self, filename: &str) {
        let file = File::open(filename).expect("File not found");
        let reader = BufReader::new(file);

        let mut cur: i32 = -1;
        let mut names: HashSet<String> = HashSet::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.len() > 1 && line.trim().starts_with("<") && line.trim().ends_with(">") {
                    if line.contains("</") {
                        cur = self.nodes[cur as usize].parent;
                    } else if cur == -1 {
                        let name = line[1..line.len() - 1].to_string();
                        let node = Node {
                            name: name.clone(),
                            children: Vec::new(),
                            height: 1,
                            parent: -1,
                        };
                        names.insert(name);
                        self.nodes.push(node);
                        cur = 0;
                    } else {
                        let name = line[1..line.len() - 1].to_string();
                        let node = Node {
                            name: name.clone(),
                            children: Vec::new(),
                            height: self.nodes[cur as usize].height + 1,
                            parent: cur as i32,
                        };

                        if names.contains(&name) {
                            print!("Tag {} appers more than one time\n", name);
                            exit(1)
                        }
                        names.insert(name);

                        let new_node_index = self.nodes.len();
                        
                        self.nodes[cur as usize].children.push(new_node_index);
                        self.nodes.push(node);
                        
                        cur = new_node_index as i32;
                    }
                }
            }
        }
    }

    fn find_path(&self, a_tag: &str, b_tag: &str) -> LinkedList<String> {
        let mut res_a: LinkedList<String> = LinkedList::new();
        let mut res_b: LinkedList<String> = LinkedList::new();

        if self.nodes.len() == 0 {
            return res_a;
        }
        let mut que: VecDeque<usize> = VecDeque::new();
        que.push_back(0);

        let (mut a, mut b): (Option<usize>, Option<usize>) = (None, None);

        while let Some(cur) = que.pop_front() {
            if self.nodes[cur].name == a_tag {
                a = Some(cur);
            } else if self.nodes[cur].name == b_tag {
                b = Some(cur);
            }

            if a.is_some() && b.is_some() {
                break;
            }

            for child in self.nodes[cur].children.iter() {
                que.push_back(*child);
            }
        }

        if let (Some(mut a), Some(mut b)) = (a, b) {
            while self.nodes[a].height > self.nodes[b].height {
                res_a.push_back(self.nodes[a].name.clone());
                a = self.nodes[a].parent as usize;
            }

            while self.nodes[b].height > self.nodes[a].height {
                res_b.push_front(self.nodes[b].name.clone());
                b = self.nodes[b].parent as usize;
            }

            while self.nodes[a].name != self.nodes[b].name {
                res_a.push_back(self.nodes[a].name.clone());
                res_b.push_front(self.nodes[b].name.clone());
                a = self.nodes[a].parent as usize;
                b = self.nodes[b].parent as usize;
            }

            res_a.push_back(self.nodes[a].name.clone());

            res_a.append(&mut res_b);
        }

        res_a
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.read_tree("src/xml.txt");
    let mut a_tag = String::new();
    let mut b_tag = String::new();
    print!("Type first tag:\n");
    let _ = io::stdin().read_line(&mut a_tag);
    print!("Type second tag:\n");
    let _ = io::stdin().read_line(&mut b_tag);
    while a_tag.trim() != "0" && b_tag.trim() != "0" {
        let path = tree.find_path(&a_tag.trim(), &b_tag.trim());

        print!("Path:\n");
        print!("{}\n", path.iter().map(|item| item.to_string()).collect::<Vec<String>>().join("->"));

        a_tag = "".to_string();
        b_tag = "".to_string();
        print!("Type first tag:\n");
        let _ = io::stdin().read_line(&mut a_tag);
        print!("Type second tag:\n");
        let _ = io::stdin().read_line(&mut b_tag);
    }
}
