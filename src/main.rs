
use std::env;
use std::fs;
use byteorder::{ReadBytesExt, LittleEndian};

mod static_window;

fn read_file(file_name : &str) -> Vec<u16>{
    //Tries to reads in a file, filename.b08 or filename.b16
    //regardless of if it is stored in u8 or u16, 
    //and returns a u16 vec.

    let file = fs::File::open(String::from(file_name) + ".b08");
    
    if let Ok(_f) = file{
        return fs::read(String::from(file_name) + ".b08").unwrap().iter().map(|x| *x as u16).collect();
    }
    else{
        let mut f = fs::File::open(String::from(file_name) + ".b16").unwrap();
        let len = fs::metadata(String::from(file_name) + ".b16").unwrap().len();
        let mut buf = vec![0 as u16;(len/2) as usize];
        f.read_u16_into::<LittleEndian>(buf.as_mut_slice()).unwrap();
        return buf.to_vec();
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();
    let point_sets = read_file(&format!("data/otypes0{}",n));
    let hamiltonian = read_file(&format!("data/hamilt0{}",n));

    let mut max = 0;
    let mut max_i : u32 = 0;
    let mut multiple_max = false;
    for (i,h) in hamiltonian.iter().enumerate(){
        if *h == max{
            multiple_max = true;
        }
        if *h > max{
            max = *h;
            max_i = i as u32;
            multiple_max = false;
        }
    }
    println!("Are there multiple max? : {}",multiple_max);
    println!("the max is :{}",max);


    let size = 500;
    let border = 0.1;

    let max_set = &point_sets[(n*2*max_i) as usize..(n*2*(max_i+1)) as usize];
    let x_set_uncast : Vec<&u16> = max_set.iter().step_by(2).collect();
    let y_set_uncast : Vec<&u16> = max_set.iter().skip(1).step_by(2).collect();

    let mut x_min = **x_set_uncast.iter().min().unwrap() as f64;// - border*(size as f64); 
    let mut x_max = **x_set_uncast.iter().max().unwrap() as f64;// + border*(size as f64); 
    let mut y_min = **y_set_uncast.iter().min().unwrap() as f64;// - border*(size as f64); 
    let mut y_max = **y_set_uncast.iter().max().unwrap() as f64;// + border*(size as f64); 
    
    let delta_x = x_max - x_min;
    let delta_y = y_max - y_min;

    x_min = x_min - delta_x*border;
    x_max = x_max + delta_x*border;
    y_min = y_min - delta_y*border;
    y_max = y_max + delta_y*border;


    let x_set : Vec<f64> = x_set_uncast.iter().map(|x| **x as f64).collect();
    let y_set : Vec<f64> = y_set_uncast.iter().map(|y| **y as f64).collect();

    let mut my_window = static_window::window::new(&format!("window{}",n),size,size);
    for i in 0..n{
        my_window.items.push(static_window::GraphicsElement::CircleElement{
            c : static_window::CommonColors::Red,
            radius : 5.0,
            x : (size as f64)*(x_set[i as usize] - x_min)/(x_max-x_min),
            y : (size as f64)*(y_set[i as usize] - y_min)/(y_max-y_min),
            centered: true,
        });
        for j in 0..n{
            if i == j{continue;};
            my_window.items.push(static_window::GraphicsElement::LineElement{
                c : static_window::CommonColors::Black,
                dashed : false,
                radius : 1.0,
                x1 : (size as f64)*(x_set[i as usize] - x_min)/(x_max-x_min),
                y1 : (size as f64)*(y_set[i as usize] - y_min)/(y_max-y_min),
                x2 : (size as f64)*(x_set[j as usize] - x_min)/(x_max-x_min),
                y2 : (size as f64)*(y_set[j as usize] - y_min)/(y_max-y_min),
            })
        }
    }
    my_window.draw();
    
}
