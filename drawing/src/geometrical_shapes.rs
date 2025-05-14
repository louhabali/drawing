use raster::{Image,Color};
use rand::Rng;

pub trait Drawable {
     fn draw(&self,img: &mut Image);
     fn color() -> Color {
        let mut rng = rand::thread_rng();
        return Color::rgb(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
    }
}

pub trait Displayable {
     fn display(&mut self, x: i32, y: i32, color: Color);
}
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

pub struct Triangle {
    pub p1 : Point,
    pub p2 : Point,
    pub p3 : Point,
}
 impl Triangle{
    pub fn new(p1 : &Point,p2 : &Point,p3:&Point) -> Self {
        Triangle {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
        }

    }
}
pub struct Rectangle  {
    pub p1 :  Point,
    pub p2 :  Point,
}
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {  
        Rectangle {
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }
}
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let radius = rng.gen_range(10..100);
        Circle::new(&center, radius)
    }
}


impl  Drawable for Point {
    fn draw(&self, img : &mut Image){
        img.set_pixel(self.x, self.y, Point::color());
    }
}
impl Drawable for Line {
    fn draw(&self, img : &mut Image){
    let dx = (self.p2.x - self.p1.x) as f64;
    let dy = (self.p2.y - self.p1.y) as f64;
    let mut newx :f64 =0.0; 
    let mut newy :f64 =0.0;
    let steps = dx.abs().max(dy.abs());
    for i in 0..steps as i32  {
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


impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        let p1 = self.p1.clone();
        let p2 = self.p2.clone();
        let p3 = Point::new(p1.x, p2.y);
        let p4 = Point::new(p2.x, p1.y);

        Line::new(p2.clone(), p3.clone()).draw(img); 
        Line::new(p3.clone(), p1.clone()).draw(img); 
        Line::new(p1.clone(), p4.clone()).draw(img); 
        Line::new(p4.clone(), p2.clone()).draw(img);
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        Line::new(self.p1.clone(), self.p2.clone()).draw(img);
        Line::new(self.p2.clone(), self.p3.clone()).draw(img);
        Line::new(self.p3.clone(),self.p1.clone()).draw(img);
    }
}


impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;
        let color = Point::color();

        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            let points = [
                (cx + x, cy + y),
                (cx + y, cy + x),
                (cx - y, cy + x),
                (cx - x, cy + y),
                (cx - x, cy - y),
                (cx - y, cy - x),
                (cx + y, cy - x),
                (cx + x, cy - y),
            ];

            for (px, py) in points {
                if px >= 0 && py >= 0 && px < img.width as i32 && py < img.height as i32 {
                    img.set_pixel(px, py, color.clone()).unwrap();
                }
            }

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }
}
