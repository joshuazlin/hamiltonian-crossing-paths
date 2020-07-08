
/*
This example is a baseline to show how to get things showing on your screen, booyah!
*/


extern crate visibility_graph;

use static_window::*;

fn main(){

    let mut w = StaticWindow::new("This is the Window Name", 500,500);

    w.items.push(GraphicsElement::LineElement{x1 : 100.0,
                                                y1 : 2.0,
                                                x2 : 30.0,
                                                y2 : 40.0,
                                                radius:2.0,
                                                dashed:false,
                                                c:CommonColors::Blue});
    w.draw();

}