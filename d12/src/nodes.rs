use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
pub struct Connection {
    pub weight: i64,
    pub from_node: usize,
    pub to_node: usize,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub connections: VecDeque<Connection>,
    pub visited: bool,
    pub distance_to_start: i64,
    pub index: usize,
    pub orig_char: char,
}

impl Node {
    pub fn new(index: usize, orig_char: char) -> Node {
        Node {
            connections: VecDeque::new(),
            visited: false,
            distance_to_start: i64::MAX,
            index: index,
            orig_char: orig_char,
        }
    }
}

fn get_reopresented_heigth(letter: char) -> i64 {
    if letter == 'S' {
        return get_reopresented_heigth('a');
    } else if letter == 'E' {
        return get_reopresented_heigth('z');
    }

    letter as i64 - 97
}

fn get_connection(from: char, from_i: usize, to: char, to_i: usize) -> Connection {
    Connection {
        weight: 1,
        from_node: from_i,
        to_node: to_i,
    }
}

fn add_connection(node: &mut Node, from: char, from_i: usize, to: char, to_i: usize) {
    node.connections
        .push_back(get_connection(from, from_i, to, to_i));
}

fn get_resulting_index(i: usize, j: usize, len_of_row: usize) -> usize {
    i * len_of_row + j
}

pub struct Nodes {
    pub nodes: VecDeque<Node>,
    pub start: usize,
    pub end: usize,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            nodes: VecDeque::new(),
            start: 0,
            end: 0,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push_back(node);
    }

    pub fn from_AoC_string(aoc_string: &str) -> Nodes {
        let mut lines: Vec<Vec<char>> = Vec::new();
        for line in aoc_string.lines() {
            lines.push(line.chars().collect());
        }

        let mut nodes = Nodes::new();

        let num_of_lines = lines.len();
        let num_of_chars = lines[0].len();

        for i in 0..num_of_lines {
            for j in 0..num_of_chars {
                let res_index = get_resulting_index(i, j, num_of_chars);
                let from = lines[i][j];
                let mut node = Node::new(res_index, from);
                if from == 'S' {
                    nodes.start = res_index;
                } else if from == 'E' {
                    nodes.end = res_index;
                }

                if i > 0 {
                    let top_i = i - 1;
                    let res_ind = get_resulting_index(top_i, j, num_of_chars);
                    let to = lines[top_i][j];
                    add_connection(&mut node, from, res_index, to, res_ind);
                }
                if i < lines.len() - 1 {
                    let bottom_i = i + 1;
                    let res_ind = get_resulting_index(bottom_i, j, num_of_chars);
                    let to = lines[bottom_i][j];
                    add_connection(&mut node, from, res_index, to, res_ind);
                }

                if j > 0 {
                    let left_j = j - 1;
                    let res_ind = get_resulting_index(i, left_j, num_of_chars);
                    let to = lines[i][left_j];
                    add_connection(&mut node, from, res_index, to, res_ind);
                }

                if j < lines[i].len() - 1 {
                    let right_j = j + 1;
                    let res_ind = get_resulting_index(i, right_j, num_of_chars);
                    let to = lines[i][right_j];
                    add_connection(&mut node, from, res_index, to, res_ind);
                }

                nodes.add_node(node);
            }
        }

        nodes
    }

    pub fn get_node_with_shortest_dist_to_start(&self, node_indexes: &HashSet<usize>) -> usize {
        let mut min = None;
        let mut index = 0;
        for ind in node_indexes {
            if let Some(m) = min {
                if self.nodes[*ind].distance_to_start < m {
                    index = *ind;
                    min = Some(self.nodes[*ind].distance_to_start);
                }
            }

            if let None = min {
                min = Some(self.nodes[*ind].distance_to_start);
                index = *ind;
            }
        }
        // if let Some(m) = min {
        //     if m == i64::MAX {
        //         panic!("Min should never max out");
        //     }
        // }
        index
    }

    pub fn get_shortest_path_length(&mut self) -> i64 {
        for node in &mut self.nodes {
            node.visited = false;
            node.distance_to_start = i64::MAX;
        }

        self.nodes[self.start].distance_to_start = 0;

        let mut unknown_nodes: HashSet<usize> = (0..self.nodes.len()).into_iter().collect();
        loop {
            if unknown_nodes.is_empty() {
                break;
            }

            let ind = self.get_node_with_shortest_dist_to_start(&unknown_nodes);
            // println!("{},{}", ind, self.nodes[ind].distance_to_start);
            unknown_nodes.remove(&ind);
            self.nodes[ind].visited = true;
            if ind == self.end {
                break;
            }

            let from_node_dist = self.nodes[ind].distance_to_start;
            let from_node_heigth = get_reopresented_heigth(self.nodes[ind].orig_char);
            let connections = self.nodes[ind].connections.clone();
            for connection in &connections {
                let mut node = &mut self.nodes[connection.to_node];
                if node.visited {
                    continue;
                }
                if get_reopresented_heigth(node.orig_char) - from_node_heigth > 1 {
                    continue;
                }

                let tmp = from_node_dist + connection.weight;
                if tmp < node.distance_to_start {
                    node.distance_to_start = tmp;
                }
            }
        }

        self.nodes[self.end].distance_to_start
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::Nodes;
    use indoc::indoc;

    #[test]
    fn from_AoC_string_and_shortest_path_test() {
        let input = indoc! {"Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"};

        let mut nodes = Nodes::from_AoC_string(input);

        assert_eq!(nodes.nodes.len(), 40);
        assert_eq!(nodes.start, 0);
        assert_eq!(nodes.end, 21);

        let node0 = &nodes.nodes[0];
        assert_eq!(node0.connections.len(), 2);
        let node0_conn0 = &node0.connections[0];
        assert_eq!(node0_conn0.weight, 1);
        assert_eq!(node0_conn0.from_node, 0);
        assert_eq!(node0_conn0.to_node, 8);

        let node0_conn1 = &node0.connections[1];
        assert_eq!(node0_conn1.weight, 1);
        assert_eq!(node0_conn1.from_node, 0);
        assert_eq!(node0_conn1.to_node, 1);

        let node39 = &nodes.nodes[39];
        assert_eq!(node39.connections.len(), 2);

        let node39_conn0 = &node39.connections[0];
        assert_eq!(node39_conn0.weight, 1);
        assert_eq!(node39_conn0.from_node, 39);
        assert_eq!(node39_conn0.to_node, 31);

        let node39_conn1 = &node39.connections[1];
        assert_eq!(node39_conn1.weight, 1);
        assert_eq!(node39_conn1.from_node, 39);
        assert_eq!(node39_conn1.to_node, 38);

        let node12 = &nodes.nodes[12];
        assert_eq!(node12.connections.len(), 4);

        let node12_conn0 = &node12.connections[0];
        assert_eq!(node12_conn0.weight, 1);
        assert_eq!(node12_conn0.from_node, 12);
        assert_eq!(node12_conn0.to_node, 12 - 8);

        let node12_conn1 = &node12.connections[1];
        assert_eq!(node12_conn1.weight, 1);
        assert_eq!(node12_conn1.from_node, 12);
        assert_eq!(node12_conn1.to_node, 12 + 8);

        let node12_conn2 = &node12.connections[2];
        assert_eq!(node12_conn2.weight, 1);
        assert_eq!(node12_conn2.from_node, 12);
        assert_eq!(node12_conn2.to_node, 12 - 1);

        let node12_conn3 = &node12.connections[3];
        assert_eq!(node12_conn3.weight, 1);
        assert_eq!(node12_conn3.from_node, 12);
        assert_eq!(node12_conn3.to_node, 12 + 1);

        assert_eq!(nodes.get_shortest_path_length(), 31);

        assert_eq!(nodes.nodes[8].distance_to_start, 1);
    }
}
