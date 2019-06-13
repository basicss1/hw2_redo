mod lib;
use std::io;
fn main() -> io::Result<()> {
    
    let mut label_graph:&mut lib::LabelGraph = &mut lib::LabelGraph::new();
    let a = lib::read_string(label_graph)?;

    let g = lib::read_input(label_graph)?;
    Ok(())
}
   
