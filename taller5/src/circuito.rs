struct Node {
    name: String,
    next: usize,
    prev: usize,
}

pub struct Circuito {
    pub name: String,
    nodes: Vec<Node>,
}

impl Circuito {
    pub fn new(name: &str, stations: &[&str]) -> Self {
        let n = stations.len();
        let nodes = stations
            .iter()
            .enumerate()
            .map(|(i, e)| Node {
                name: e.to_string(),
                next: (i + 1) % n,
                prev: (i + n - 1) % n,
            })
            .collect();
        Circuito {
            name: name.to_string(),
            nodes,
        }
    }

    pub fn find_all(&self, name: &str) -> Vec<usize> {
        let buscado = name.trim().to_lowercase();
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.name.to_lowercase() == buscado)
            .map(|(i, _)| i)
            .collect()
    }

    // Camino mas corto entre dos indices del mismo circuito
    pub fn route(&self, a: usize, b: usize) -> Vec<String> {
        let mut front = vec![self.nodes[a].name.clone()];
        let mut i = a;
        while i != b {
            i = self.nodes[i].next;
            front.push(self.nodes[i].name.clone());
        }

        let mut back = vec![self.nodes[a].name.clone()];
        let mut i = a;
        while i != b {
            i = self.nodes[i].prev;
            back.push(self.nodes[i].name.clone());
        }

        if front.len() <= back.len() {
            front
        } else {
            back
        }
    }
}
