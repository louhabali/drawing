use raster::{Image,Color};
use rand::Rng;

pub trait Drawable {
    fn draw(&self,img: &mut Image);
    fn color() -> Color {
        let mut rng = rand::thread_rng();
        return Color::rgb(
            rng.gen_range(0..255), 
            rng.gen_range(0..255),
            rng.gen_range(0..255)
        );
    }
}

pub trait Displayable {
     fn display(&mut self, x: i32, y: i32, color: Color);
}

// point implementation
#[derive(Clone)]
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
impl  Drawable for Point {
    fn draw(&self, img : &mut Image){
       
        img.display(self.x, self.y,Point::color());
    }
}

// line implementation
pub struct Line {
    pub p1: Point,
    pub p2: Point,
    color : Color,
}

impl Line {
    pub fn new(p1: Point, p2: Point,color : Color) -> Self {
        Line { 
            p1, 
            p2,
            color
         }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            Point::color(),
    )

    }
}
impl Drawable for Line {
    fn draw(&self, img : &mut Image){
    let dx = (self.p2.x - self.p1.x) as f64;
    let dy = (self.p2.y - self.p1.y) as f64;
   
    let steps = dx.abs().max(dy.abs());
    for i in 0..steps as i32  {
        let t = i as f64 / steps; 
        let mut newx = self.p1.x as f64 + dx * t; 
        let mut newy = self.p1.y as f64 + dy * t;
        newx = newx.round();
        newy = newy.round();
        if newx >= 0.0 && newx < img.width.into() && newy >= 0.0 && newy < img.height.into() {
             img.display(newx as i32, newy as i32, self.color.clone());
        }
    }
    }
}

// triangle implementation
pub struct Triangle {
    pub p1 :Point,
    pub p2 :Point,
    pub p3 :Point,
}
 impl Triangle{
    pub fn new(p1 :&Point,p2 : &Point,p3: &Point) -> Self {
        Triangle {
            p1:p1.clone(),
            p2:p2.clone(),
            p3:p3.clone(),
        }

    }
}
impl  Drawable for Triangle{
    fn draw(&self, img : &mut Image){
        let c = Point::color();
       let l1 = Line {p1 : self.p1.clone(),p2: self.p2.clone(),color:c.clone()};
       let l2 = Line {p1 : self.p2.clone(),p2: self.p3.clone(),color:c.clone()};
       let l3 = Line {p1 : self.p3.clone(),p2: self.p1.clone(),color:c.clone()};
       l1.draw(img);
       l2.draw(img);
       l3.draw(img);
    }
}


// rectangle implementation
pub struct Rectangle  {
    pub p1 :  Point,
    pub p2 :  Point,
}
 impl  Rectangle {
    pub fn new(p1 : &Point,p2 : &Point) -> Self {
        Rectangle {
            p1 : p1.clone(),
            p2 : p2.clone(),
        }
        
    }
}
impl  Drawable for Rectangle{
    fn draw(&self, img : &mut Image){
        let p3 = Point::new(self.p1.x , self.p2.y);
        let p4 = Point::new(self.p2.x , self.p1.y);
        let c = Point::color();
       let l1 = Line {p1 : self.p2.clone(),p2 : p3.clone(),color:c.clone()};
       let l2 = Line {p1 : p3.clone(), p2 : self.p1.clone(),color:c.clone()};
       let l3 = Line {p1:self.p1.clone(),p2: p4.clone(),color:c.clone()};
       let l4 = Line {p1: p4.clone() ,p2 : self.p2.clone(),color:c.clone()};
       l1.draw(img);
       l2.draw(img);
       l3.draw(img);
       l4.draw(img);
    }
}

// circle implementation
pub struct Circle{
    pub center : Point,
    pub radius : i32,
}
 impl Circle {
    pub fn new(center : &Point , radius : i32)-> Self {
        Circle {
           center: center.clone(),
        radius,
        }
    }
    pub fn random(width:i32, height:i32)-> Circle {
            let mut rng = rand::thread_rng();
            Self::new( 
                &Point::random(width,height),
                rng.gen_range(0..height)
            )
    }
}
fn calculate_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx.powi(2) + dy.powi(2)).sqrt()
}
impl Drawable for Circle{
    fn draw(&self, img : &mut Image){
        let cx= self.center.x;
        let cy= self.center.y;
        let r= self.radius as f64;
        let mycolor = Self::color();
        let mut x=cx;
        let mut y=cy-r as i32;

       while y<=cy {
        let a=(calculate_distance(cx as f64,cy as f64,(x+1) as f64,y as f64) -r).abs();
        let b=(calculate_distance(cx as f64,cy as f64,x as f64,(y+1) as f64) -r).abs();
        let c=(calculate_distance(cx as f64,cy as f64,(x+1) as f64,(y+1) as f64) -r).abs();
        let min= a.min(b).min(c);

        if a==min{
            x+=1;
        }else if b==min{
            y+=1;
        }else if c==min{
            x+=1;
            y+=1;
        }
        img.display(x, y, mycolor.clone());
        img.display(x, cy + (cy - y), mycolor.clone());
        img.display(2 * cx - x, y,  mycolor.clone());
        img.display(2 * cx - x, cy + (cy - y), mycolor.clone());
       }


    }
}




