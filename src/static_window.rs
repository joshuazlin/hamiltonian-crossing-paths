/*
The idea behind this .rs file is that this will parse some
list of primitives and show them on a window,
and *thats it*, doesn't do anything else. 
*/

extern crate piston_window;
use piston_window::*;

pub enum CommonColors{
    White,
    Black,
    Red,
    Green,
    Blue,
}

//Quite disgusting, I know. 
//Surely not the right way to do things
//but in this town, we are cowboys
impl CommonColors{
    fn to_rgb(&self) -> [f32;4] {
        match self{
            CommonColors::Black => [0.0,0.0,0.0,1.0],
            CommonColors::White => [1.0,1.0,1.0,1.0],
            CommonColors::Red   => [1.0,0.0,0.0,1.0],
            CommonColors::Green => [0.0,1.0,0.0,1.0],
            CommonColors::Blue  => [0.0,0.0,1.0,1.0],
        }
    }
}

/*
I'm definitely not doing this part right either....
*/
pub enum GraphicsElement{
    LineElement {x1:f64, y1:f64, x2:f64, y2:f64, radius: f64, dashed:bool, c:CommonColors,},
    CircleElement {x:f64, y:f64, radius:f64,c:CommonColors,centered:bool,},
    //GraphElement(VisibilityGraph),
}


impl GraphicsElement{
    fn draw<G : Graphics>(&self, t : [[f64;3];2], g: &mut G){
        match self{
            GraphicsElement::LineElement{x1,y1,x2,y2,radius,dashed,c} => line(c.to_rgb(),*radius,[*x1,*y1,*x2,*y2],t, g),
            GraphicsElement::CircleElement{x,y,radius,c,centered:false} => ellipse(c.to_rgb(),
                                                                            [*x,*y,2.0*(*radius),2.0*(*radius)],t,g),
            GraphicsElement::CircleElement{x,y,radius,c,centered:true} => ellipse(c.to_rgb(),
                                                                            [*x-*radius,*y-*radius,2.0*(*radius),2.0*(*radius)],t,g),
        }

    }
}

pub struct window {
    opengl: OpenGL, // OpenGL drawing backend.
    window:  PistonWindow,
    pub items: Vec<GraphicsElement>,
}

impl window {

    pub fn new(name : &str, width: u32, height:u32) -> window {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new(name, [width,height])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();
        window.set_lazy(true);

        window{
            opengl:opengl,
            window:window,
            items:vec![],
        }
    }

    pub fn draw(&mut self){
        while let Some(e) = self.window.next() {

            self.window.draw_2d(&e, |c,g,_| {clear([1.0;4],g)});
            
            for graphic_element in &self.items{
                self.window.draw_2d(&e, |c,g,_| {
                    graphic_element.draw(c.transform, g);
                });
            }
        };
    }
}

#[cfg(test)]
mod test_draw {
    use super::*;

    //#[test]
}