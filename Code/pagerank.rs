use crate::graph::Graph;
use std::collections::HashMap;
use rand::{Rng, thread_rng};

pub fn computing_pagerank(graph:&Graph,walks:usize)->HashMap<usize,usize>{
    let mut rng=thread_rng();
    let num_vertices=graph.number_of_vertices();
    let mut termination_of_walks =vec![0;num_vertices];
    let mut pagerank= HashMap::<usize,usize>::new();
    for i in 0..num_vertices{
        for _j in 0..walks{
            let mut new_vertex=i;
            for _steps in 0..100{
                if graph.edges[new_vertex].is_empty(){
                    new_vertex=rng.gen_range(0..num_vertices);
                    
                    
                }
                else {
                    let random_prob: f64=rng.gen();
                    if random_prob<0.1{
                        new_vertex=rng.gen_range(0..num_vertices);
                        
                    }
                    else{
                        let adjacent=&graph.edges[new_vertex];
                        new_vertex=adjacent[rng.gen_range(0..adjacent.len())];

                    }
                }
            }
            termination_of_walks[new_vertex]+=1;
        }
    }
    
    
    for i in 0..num_vertices{
        pagerank.insert(i,*termination_of_walks.get(i).unwrap());
    }



    pagerank
}