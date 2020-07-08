/*
This example hopefully gives an example of visualising a visibility graph. 
*/

extern crate visibility_graph;
use static_window::*;
use visibility_graph::*;

fn main(){
    
    let g = VisibilityGraph::random(800.0,800.0,30,30,0.01);

    //println!("{:?}",g);

    let mut w = StaticWindow::new("This is the Window Name", 800,800);
    w.items.push(GraphicsElement::GraphElement(g));
    w.draw();
}