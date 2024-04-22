mod graph;
mod pagerank;
use crate::graph::Graph;
use crate::pagerank::computing_pagerank;

use std::fs::File;
use std::io::prelude::*;



fn main() {
    let vector=read_file("data.txt");
    let pagerank = computing_pagerank(&vector, 100); 
    println!("Top 5 vertices by approximate PageRank:");
    let mut vector_pagerank: Vec<(usize, usize)> = pagerank.into_iter().collect();
    vector_pagerank.sort_by_key(|&x| std::cmp::Reverse(x.1));
    let mut i=0;
    for (k,v) in vector_pagerank{
        if i<5{
            let calculating=(v as f64) /(100 as f64*vector.edges.len() as f64);
            println!("For vertex {}, the pagerank is: {:?}",k,calculating);
        }
        else{
            break;
        }
        i=i+1;

    
}
}

fn read_file(path: &str) -> Graph {
    let file = File::open(path).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    let first_line = buf_reader.next().expect("File is empty").unwrap();
    let num_vertices = first_line.trim().parse::<usize>().expect("Invalid number of vertices");
    let mut graph=Graph::new_graph(num_vertices);

    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
       
        graph.add_edges(x as usize,y as usize);
     }
     graph
       
    }

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn correct_graph_edges(){
        let mut graph1=Graph::new_graph(4);
        graph1.add_edges(1,2);
        graph1.add_edges(2,3);
        graph1.add_edges(3,4);
        graph1.add_edges(1,4);
        assert_eq!(graph1.edges[1],vec![2,4]);
        assert_eq!(graph1.edges[2],vec![3]);
        assert_eq!(graph1.edges[3],vec![4]);
    }
    #[test]
    fn add_to_one(){
        let mut graph2=Graph::new_graph(5);
        graph2.add_edges(0,3);
        graph2.add_edges(1,3);
        graph2.add_edges(2,3);
        graph2.add_edges(3,4);
        graph2.add_edges(4,3);
        graph2.add_edges(4,0);
        let pagerank1 = computing_pagerank(&graph2, 100); 
        let mut i:f64=0.0;
        let mut calculating:f64=0.0;
        for (_k,v) in pagerank1{
                calculating=(v as f64) /(100 as f64*graph2.number_of_vertices() as f64);
                i=i+calculating;
            }
        
        assert_eq!(i,1.0);
    
    }

}



