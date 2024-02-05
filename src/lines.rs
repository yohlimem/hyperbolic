use std::string;

use nannou::{math::Vec2Angle, prelude::{Vec2, vec2, Draw, BLUE}};
// mod Line {};
use crate::points::Point;


#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn from_points(start: Point, end: Point) -> Self {
        Line {start, end}
    }

    pub fn length(&self) -> f32 {
        (self.end.pos - self.start.pos).length()
    }

    pub fn angle(&self) -> f32 {
        (self.end.pos - self.start.pos).angle()
    }

    pub fn midpoint(&self) -> Point {
        Point::from(vec2((self.start.pos.x + self.end.pos.x) / 2.0, (self.start.pos.y + self.end.pos.y) / 2.0))
    }

    pub fn function(&self) -> Box<dyn Fn(f32) -> f32> {
        let m = (self.end.pos.y - self.start.pos.y) / (self.end.pos.x - self.start.pos.x);
        let b = self.start.pos.y - m * self.start.pos.x;
        
        Box::new(move |x| m * x + b) as Box<dyn Fn(f32) -> f32>

    }
    pub fn function_to_line(fx: &Box<dyn Fn(f32) -> f32>, x1: f32, x2: f32) -> Line {
        // convert a function (mx+b) to a line object
        let start = Point::from(vec2(x1, fx(x1)));
        let end = Point::from(vec2(x2, fx(x2)));
        Line::from_points(start, end)

    }

    pub fn tangent_function(&self) -> Box<dyn Fn(f32) -> f32> {
        let m = -(self.end.pos.x - self.start.pos.x) / (self.end.pos.y - self.start.pos.y); // inverse of the slope
        let b = self.end.pos.y - m * self.end.pos.x; // y = mx + b -> y - mx = b -> 
        
        Box::new(move |x| m * x + b) as Box<dyn Fn(f32) -> f32>
    }
    pub fn tangent_function_from(&self, point: Vec2) -> Box<dyn Fn(f32) -> f32> {
        let m = -(self.end.pos.x - self.start.pos.x) / (self.end.pos.y - self.start.pos.y); // inverse of the slope
        let b = point.y - m * (point.x); // y = mx + b -> y - mx = b -> 
        
        Box::new(move |x| m * x + b) as Box<dyn Fn(f32) -> f32>
    }



    pub fn algebraic_intersection(&self, other: &Line) -> Option<Point> {
        let x = (other.start.pos.y - self.start.pos.y + (self.start.pos.x * (self.end.pos.y - self.start.pos.y) / (self.end.pos.x - self.start.pos.x)) - other.start.pos.x * (other.end.pos.y - other.start.pos.y) / (other.end.pos.x - other.start.pos.x)) / ((self.end.pos.y - self.start.pos.y) / (self.end.pos.x - self.start.pos.x) - (other.end.pos.y - other.start.pos.y) / (other.end.pos.x - other.start.pos.x));
        let y = self.start.pos.y + (self.end.pos.y - self.start.pos.y) / (self.end.pos.x - self.start.pos.x) * (x - self.start.pos.x);
        if x.is_nan() || y.is_nan() {
            return None
        }
        Some(Point::from(vec2(x, y)))
        
    }

    pub fn circle_to_line_intersection(&self, center: Vec2, radius: f32) -> Option<(Point, Point)> {
        let dx = self.end.pos.x - self.start.pos.x;
        let dy = self.end.pos.y - self.start.pos.y;
        let dr = (dx.powi(2) + dy.powi(2)).sqrt();
        let D = self.start.pos.x * self.end.pos.y - self.end.pos.x * self.start.pos.y;
        let discriminant = radius.powi(2) * dr.powi(2) - D.powi(2);
        if discriminant < 0.0 {
            return None
        }
        let sign = if dy < 0.0 { -1.0 } else { 1.0 };
        let x1 = (D * dy + sign * dx * discriminant.sqrt()) / dr.powi(2);
        let y1 = (-D * dx + dy.abs() * discriminant.sqrt()) / dr.powi(2);
        let x2 = (D * dy - sign * dx * discriminant.sqrt()) / dr.powi(2);
        let y2 = (-D * dx - dy.abs() * discriminant.sqrt()) / dr.powi(2);
        Some((Point::from(vec2(x1, y1)), Point::from(vec2(x2, y2))))
    }

    pub fn draw(&self, draw: &Draw) {
        draw.line()
            .start(self.start.pos)
            .end(self.end.pos)
            .color(BLUE);


    }
}