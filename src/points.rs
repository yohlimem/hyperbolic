use nannou::prelude::*;
use crate::lines::Line;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
}


impl Point {
    pub fn from(pos: Vec2) -> Self {
        Point {
            pos: pos,
        }
    }
}

impl Point {
    pub fn angle_from_origin(&self) -> f32 {
        self.pos.y.atan2(self.pos.x)
    }

    pub fn distance_from_origin(&self) -> f32 {
        self.pos.length()
    }
    pub fn hyperbolic_distance(&self) -> f32 {
        let d= self.distance_from_origin();
        0.5*((1.0+d)/(1.0-d)).ln()
    }

    pub fn to_hyperbolic_point(&self) -> Point {
        let d = self.hyperbolic_distance();
        let a = self.angle_from_origin();
        println!("{:?}", vec2(d*self.pos.x.tanh(),d* self.pos.y.tanh()));
        Point {
            pos: vec2(d*self.pos.x.tanh(),d* self.pos.y.tanh()),
        }
    }

    pub fn draw(&self, draw: &Draw, string: Option<&str>) {
        draw.ellipse()
            .xy(self.pos)
            .radius(10.0)
            .color(GRAY);
        if let Some(str) = string {
            
            draw.text(str).color(RED).xy(self.pos).font_size(20);
        }
    }

    pub fn find_inverse(&self, radius: f32) -> Option<Point>{
        let p1 = self.pos;
        let center_to_a = Line::from_points(Point{pos: Vec2::ZERO}, Point{pos: p1});
        let perpendicular_to_ACenter = center_to_a.tangent_function();
        let intersection_disk_to_tangent = Line::function_to_line(&perpendicular_to_ACenter, -1000.0, 1000.0).circle_to_line_intersection(Vec2::ZERO, radius);
        
        
        if intersection_disk_to_tangent.is_none() {
            // println!("no intersectionSome");
            return None;
        }
        
        let X = intersection_disk_to_tangent.unwrap().0;
        let Y = intersection_disk_to_tangent.unwrap().1;
        
        let tan_x = Line::function_to_line(&Line::from_points(Point::from(vec2(0.0,0.0)), X).tangent_function(), -1000.0, 1000.0);
        let tan_y =  Line::function_to_line(&Line::from_points(Point::from(vec2(0.0,0.0)), Y).tangent_function(), -1000.0, 1000.0);
        
        // println!(Some"{:?}Some", X.pos);
        // tan_x.draw(&app.draw());
        // tan_y.draw(&app.draw());
        let intersection = tan_x.algebraic_intersection(&tan_y);
        
        if intersection.is_none() {
            // println!(Some"{:?}Some", intersection.unwrap().pos);
            return None;
        }
        
        let intersection = intersection.unwrap();
        return Some(intersection);
    }
}
