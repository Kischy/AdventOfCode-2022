use std::collections::VecDeque;

pub struct Connection {
    pub weight: i8,
    pub from_node: usize,
    pub to_node: usize,
}

pub struct Node {
    connections: VecDeque<Connection>,
}

fn get_reopresented_heigth(letter: char) -> i8 {
    if letter == 'S' {
        return get_reopresented_heigth('a');
    } else if letter == 'E' {
        return get_reopresented_heigth('z');
    }

    letter as i8 - 97
}

fn get_connection(from: char, from_i: usize, to: char, to_i: usize) -> Connection {
    Connection {
        weight: (get_reopresented_heigth(from) - get_reopresented_heigth(to)).abs() + 1,
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
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            nodes: VecDeque::new(),
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
                let mut node = Node {
                    connections: VecDeque::new(),
                };
                let from = lines[i][j];
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
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::Nodes;
    use indoc::indoc;

    #[test]
    fn from_AoC_string_test() {
        let input = indoc! {"Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"};

        let mut nodes = Nodes::from_AoC_string(input);

        assert_eq!(nodes.nodes.len(), 40);

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
        assert_eq!(node39_conn0.weight, 2);
        assert_eq!(node39_conn0.from_node, 39);
        assert_eq!(node39_conn0.to_node, 31);

        let node39_conn1 = &node39.connections[1];
        assert_eq!(node39_conn1.weight, 2);
        assert_eq!(node39_conn1.from_node, 39);
        assert_eq!(node39_conn1.to_node, 38);

        let node12 = &nodes.nodes[12];
        assert_eq!(node12.connections.len(), 4);

        let node12_conn0 = &node12.connections[0];
        assert_eq!(node12_conn0.weight, 10);
        assert_eq!(node12_conn0.from_node, 12);
        assert_eq!(node12_conn0.to_node, 12 - 8);

        let node12_conn1 = &node12.connections[1];
        assert_eq!(node12_conn1.weight, 2);
        assert_eq!(node12_conn1.from_node, 12);
        assert_eq!(node12_conn1.to_node, 12 + 8);

        let node12_conn2 = &node12.connections[2];
        assert_eq!(node12_conn2.weight, 8);
        assert_eq!(node12_conn2.from_node, 12);
        assert_eq!(node12_conn2.to_node, 12 - 1);

        let node12_conn3 = &node12.connections[3];
        assert_eq!(node12_conn3.weight, 2);
        assert_eq!(node12_conn3.from_node, 12);
        assert_eq!(node12_conn3.to_node, 12 + 1);
    }
}
