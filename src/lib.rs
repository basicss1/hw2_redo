use std::{env, process};
use std::fs::{self,File};
use std::vec::Vec;
use std::collections::HashMap;
use std::io::{self, Write, prelude::*};

pub struct IntGraph {
    //use the mappins to number to form the graph
    //equivalent to my adjacent_matrix
    graph: Vec<Vec<usize>>,
}
impl IntGraph{
    pub fn get_len(&self) -> usize{
        self.graph.len()
    }

    pub fn get_contents(&self) -> &Vec<Vec<usize>>{
        &self.graph
    }
    pub fn new() -> IntGraph {
        IntGraph{
            graph: Vec::new()
        }
    }

    pub fn set_graph(&mut self, length: usize){
        let mut zero_vec: Vec<usize> = Vec::with_capacity(length);
        let mut vecofvec: Vec<Vec<usize>> = Vec::new();
        for i in 0..length{
            zero_vec.push(0 as usize);
        }
        for i in 0..length{
            let mut add_one = zero_vec.clone();
            add_one[i] = 1;
            vecofvec.push(add_one);
        }
        self.graph = vecofvec.clone();
    }
   
    pub fn set_1_for_neighbor(&mut self, vertex: usize, neighbor: usize){
        //if it is not 0, set it to 0
        if self.graph[vertex][neighbor] != 1{
            self.graph[vertex][neighbor] = 1;
            self.graph[neighbor][vertex] = 1;
        }
    }
}
pub struct LabelMap {
    forward: HashMap<String, usize>,
    backward: Vec<String>
}
impl LabelMap{
    pub fn new() -> LabelMap{
        LabelMap{
            forward: HashMap::new(),
            backward: Vec::new(),
        }
    }
   
    pub fn get_len(&self) -> usize{
        self.backward.len()
    }
    pub fn add_word(&mut self, word: &str){
        if(!self.backward.contains(&word.to_string())){
            self.forward.insert(word.to_string().clone(), self.get_len());
            self.backward.push(word.to_string().clone());
        }
    }
}

pub struct LabelGraph {
    intgraph: IntGraph,
    labelmap: LabelMap,
   
}

impl LabelGraph{
    pub fn new() -> LabelGraph{
        LabelGraph{
            intgraph: IntGraph::new(),
            labelmap: LabelMap::new(),
           
        }
    }
    pub fn set_graph(&mut self, intgraph: IntGraph, labelmap: LabelMap){
        self.intgraph = intgraph;
        self.labelmap = labelmap;
    }
    //rewrite my helper functions
    //original helper(&Labelmap, begin_word: String, end_word: String)
    
    //woohoo! updated version for helper
    pub fn helper(&self, answer: &mut Vec<String>, begin_values: usize, end_values: usize, mut found: &mut i32){
        
        if self.intgraph.graph[begin_values][end_values] == 1{
            *found = 1;
            answer.push(self.labelmap.backward[begin_values].clone());
            answer.push(self.labelmap.backward[end_values].clone());
            // if answer.contains(&self.labelmap.backward[end_values]){
            println!("this is the path {:?}\n", answer);
            return;
          
        }
       
        else{
            for i in 0..self.labelmap.get_len(){
                
                if i == begin_values || answer.contains(&self.labelmap.backward[i]){
                    continue;
                }
                if self.intgraph.graph[begin_values][i] == 1{
                    answer.push(self.labelmap.backward[begin_values].clone());
                    self.helper(answer, i, end_values, found);
                }
            }
        }
    
    }
}

pub fn read_string(mut label_graph: &mut LabelGraph) -> io::Result<()>{
    //from rust doc
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)?;
    //get rid of empty lines from both sides
    contents.trim_left().trim_right();
    //count how many lines
    let vlen: usize = contents.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1 as usize;
    label_graph.intgraph.set_graph(vlen);
    for ele in contents.lines(){
        
        if ele.is_empty(){
            continue;
        }
        else{
            let a: Vec<&str> = ele.split_whitespace().collect();
            let new_word = a[0].clone();
            label_graph.labelmap.add_word(new_word);
            if a.len() > 1{
                for word_ in &a[1..]{
                    let new_word_ = word_.clone();
                    label_graph.labelmap.add_word(word_);
                    //SAFE for unwrap because both words are added
                    let first_: usize = *(label_graph.labelmap).forward.get(new_word).unwrap();
                    let second_: usize = *(label_graph.labelmap).forward.get(new_word_).unwrap();
                    label_graph.intgraph.set_1_for_neighbor(first_, second_);
                }
                
            }
        }
    }
    Ok(())

}

pub fn read_input(label_graph: &mut LabelGraph) -> io::Result<()> {
    loop{
        println!("Start and end to choose from {:?}", label_graph.labelmap.backward);
        print!(">> ");
        io::stdout().flush()?;
        let mut contents = String::new();
        let g = match io::stdin().read_line(&mut contents) {
            Ok(s) => s,
            Err(e) => return Err(e)
        };
        let user_inputs: Vec<&str> = contents.split_whitespace().collect();
        if user_inputs.len() != 2{
            
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "wrong number of inputs\n"))
        }
        let new_word = user_inputs[0];
        let new_word_ = user_inputs[1];
       
        if !label_graph.labelmap.backward.contains(&new_word.to_string()) || !label_graph.labelmap.backward.contains(&new_word_.to_string()){
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "string not existing\n"))
            
        }
        let first_: usize = *(label_graph.labelmap).forward.get(new_word).unwrap();
        let second_: usize = *(label_graph.labelmap).forward.get(new_word_).unwrap();
        let mut found: &mut i32 = &mut 0;
       
        let mut ans: &mut Vec<String> = &mut Vec::new();
        label_graph.helper(ans, first_, second_, found);
        
        if (*found == 0){
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "path not found\n"))
        }
    }
}

#[cfg(test)]
    //first write tests for the intgraph
    #[test]
    fn intgraph_set_graph(){
        let mut intgraph = IntGraph::new();

        intgraph.set_graph(2 as usize);
        assert_eq!(intgraph.graph[0], vec![1 as usize, 0 as usize]);
        assert_eq!(intgraph.graph[1], vec![0 as usize, 1 as usize]);
        
    }

    fn intgraph_set_1_for_neighbor(){
        let mut intgraph = IntGraph::new();

        intgraph.set_graph(3 as usize);
        intgraph.set_1_for_neighbor(0 as usize, 1 as usize);
        assert_eq!(intgraph.graph[0], vec![1 as usize, 1 as usize, 0 as usize]);
        assert_eq!(intgraph.graph[1], vec![1 as usize, 1 as usize, 0 as usize]);
        assert_eq!(intgraph.graph[2], vec![0 as usize, 0 as usize, 1 as usize]);
       
    }
    //second write test for labelmap
    #[test]
    fn add_word_test(){
        let mut labelmap = LabelMap::new();
        labelmap.add_word("hehe");
        assert_eq!(*(labelmap).forward.get(&"hehe".to_string()).unwrap(), 0 as usize);
        assert_eq!(labelmap.backward[0], "hehe".to_string());
    }
    //third write test for labelgraph
    #[test]
    fn labelgraph_set_graph(){
        let mut labelgraph = LabelGraph::new();
        let mut labelmap = LabelMap::new();
        labelmap.add_word("hehe");
        let mut intgraph = IntGraph::new();
        intgraph.set_graph(2 as usize);
        labelgraph.set_graph(intgraph, labelmap);

        assert_eq!(labelgraph.intgraph.graph[0], vec![1 as usize, 0 as usize]);
        assert_eq!(labelgraph.intgraph.graph[1], vec![0 as usize, 1 as usize]);

        assert_eq!(*(labelgraph.labelmap).forward.get(&"hehe".to_string()).unwrap(), 0 as usize);
        assert_eq!(labelgraph.labelmap.backward[0], "hehe".to_string());
    }

    fn label_graph_helper(){
        let mut labelgraph = LabelGraph::new();
        let mut labelmap = LabelMap::new();
        let mut intgraph = IntGraph::new();
        labelmap.add_word("foo");
        labelmap.add_word("bar");
        labelmap.add_word("qux");
        labelmap.add_word("baz");

        intgraph.set_graph(4);
        intgraph.set_1_for_neighbor(0 as usize,1 as usize);
        intgraph.set_1_for_neighbor(0 as usize, 2 as usize);
        intgraph.set_1_for_neighbor(1 as usize, 3 as usize);

        labelgraph.set_graph(intgraph, labelmap);
        let mut ans: &mut Vec<String> = &mut Vec::new();
        let mut found: &mut i32 = &mut 0;
        //first test on connected vertexes
        labelgraph.helper(ans, 0, 1, found);
        assert_eq!(ans[0], "foo".to_string());
        assert_eq!(ans[1], "bar".to_string());

        let mut ans1: &mut Vec<String> = &mut Vec::new();
        let mut found1: &mut i32 = &mut 0;
        labelgraph.helper(ans1, 0, 3, found1);

        assert_eq!(ans.contains(&"foo".to_string()), true);
        assert_eq!(ans.contains(&"bar".to_string()), true);
        assert_eq!(ans.contains(&"baz".to_string()), true);
        //make sure the length also matches
        assert_eq!(*found1, 1);
        assert_eq!(ans.len(), 3 as usize);

        let mut ans2: &mut Vec<String> = &mut Vec::new();
        let mut found2: &mut i32 = &mut 0;
        labelgraph.helper(ans1, 2, 3, found2);
        assert_eq!(ans2.len(), 0 as usize);
        //make sure found value is right to print the error message
        assert_eq!(*found2, 0);
    }

    



