
use raster::{Image,Color};
use rand::Rng;

pub trait Drawable {
     fn draw(&self,img: &mut Image);
     fn color() -> Color {
        return Color::rgb(51, 189, 231);
    }
}

pub trait Displayable {
     fn display(&mut self, x: i32, y: i32, color: Color);
}

pub struct Point {
    pub x : i32,
    pub y : i32,
}
 impl Point {
    pub fn new(x: i32,y:i32) -> Self {
      Point {
        x,
        y,
      }  
    }

    pub fn random(width:i32, height:i32)-> Self {
            let mut rng = rand::thread_rng();
            Point {
                x : rng.gen_range(0..width),
                y : rng.gen_range(0..height),
            }
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Line {
            p1: Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            p2: Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
        }
    }
}

// pub struct Triangle<'a> {
//     pub p1 :&'a Point,
//     pub p2 :&'a Point,
//     pub p3 :&'a Point,
// }
//  impl <'a>Triangle<'a>{
//     pub fn new(p1 : &'a Point,p2 : &'a Point,p3:&'a Point) -> Self {
//         Triangle {
//             p1,
//             p2,
//             p3,
//         }

//     }
// }
// pub struct Rectangle  {
//     pub p1 :  Point,
//     pub p2 :  Point,
// }
//  impl  Rectangle {
//     pub fn new(p1 : Point,p2 : Point) -> Self {
//         Rectangle {
//             p1,
//             p2,
//         }
        
//     }
// }
// pub struct Circle<'a>{
//     pub center : &'a Point,
//     pub radius : i32,
// }
//  impl <'a>Circle<'a> {
//     pub fn new(center : &'a Point , radius : i32)-> Self {
//         Circle {
//             center,
//             radius,
//         }
//     }
//     pub fn random(width:i32, height:i32)-> Point {
//             let mut rng = rand::thread_rng();
//             Point {
//                 x : rng.gen_range(0..width),
//                 y : rng.gen_range(0..height),
//             }
//     }
// }
// impl Drawable for Line {

// }
impl  Drawable for Point {
    fn draw(&self, img : &mut Image){
        img.set_pixel(self.x, self.y, Point::color());
    }
}
impl Drawable for Line {
    fn draw(&self, img : &mut Image){
    //    img.set_pixel(self.p1.x, self.p1.y, Point::color());
    //    img.set_pixel(self.p2.x, self.p2.y, Point::color());
    let dx = (self.p1.x - self.p2.x) as f64;
    let dy = (self.p1.y - self.p2.y) as f64;
    let mut newx :f64 =0.0; 
    let mut newy :f64 =0.0;
    let steps = dx.abs().max(dy.abs());
    for i in 0..steps as i32  {
        //println!("{}",i);
        let t = i as f64 / steps; 
        newx = self.p1.x as f64 + dx * t;
        newy = self.p1.y as f64 + dy * t;
        println!("{}---{}",newx,newy);
        newx = newx.round();
        newy = newy.round();
        if newx >= 0.0 && newx < img.width.into() && newy >= 0.0 && newy < img.height.into() {
             img.set_pixel(newx as i32, newy as i32, Point::color()).unwrap();
        }
    }
    
    }
}
// impl  Drawable for Rectangle{
//     fn draw(&self, img : &mut Image){
//        //img.
//     }
// }
// impl <'a> Drawable for Triangle <'a>{
//     fn draw(&self, img : &mut Image){
//        //img.
//     }
// }
