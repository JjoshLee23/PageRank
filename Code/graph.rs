#[derive(Debug)]
pub struct Graph{
    pub edges:Vec<Vec<usize>>,
}

impl Graph{
    pub fn new_graph(num_vertices: usize)->Self{
        Self{
            edges:vec![Vec::new();num_vertices],
        }
    }
    pub fn add_edges(&mut self, vertex:usize,next_edge:usize){
        if vertex<self.edges.len(){
            self.edges[vertex].push(next_edge);
        }
    }
    pub fn number_of_vertices(&self)->usize{
        self.edges.len()
    }

}
