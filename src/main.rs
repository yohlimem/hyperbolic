use std::vec;

use lines::Line;
use nannou::{draw::mesh::vertex::Color, ease::circ, geom::point, math::ConvertAngle, prelude::*};
use nannou_egui::{self, color_picker::Alpha, egui::{self, lerp, Color32}, Egui};

mod points;
mod lines;
use points::Point;

struct Model {
    // window: Window,
    egui: Egui,
    circle_radius: f32,
    mouse_down: bool,
    create_line: (Option<Vec2>, Option<Vec2>),
    circles: Vec<(f32, Point, Line)>,
    lines: Vec<Line>,
    node_size: f32,

    segment_or_circle: bool,
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).fullscreen().raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let circle_radius = 700.0;
    Model {
        egui,
        circle_radius,
        circles: vec![],
        create_line: (None, None),
        mouse_down: true,
        lines: vec![],
        node_size: 5.0,
        segment_or_circle: false,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    
    
    {let egui = &mut model.egui;
        egui.set_elapsed_time(update.since_start);
        
        let ctx = egui.begin_frame();
        
        egui::Window::new("Rum windowSome").show(&ctx, |ui| {
            ui.label("radius");
            ui.add(egui::Slider::new(&mut model.circle_radius, 50.0..=2000.0));
            ui.add(egui::Slider::new(&mut model.node_size, 0.1..=10.0));            
            ui.add(egui::Checkbox::new(&mut model.segment_or_circle, "Show full circle"));            
    });}
    // draw lines with the mouse
    draw_lines(&app, &app.draw(), model, 255);

    // calculate the perpendicular circles to the disk
    // TODO: https://math.stackexchange.com/a/1322475
    // https://www.youtube.com/watch?v=xHCEDMQDbzk
    // https://www.ms.uky.edu/~droyster/courses/spring08/math6118/classnotes/chapter09.pdf PAGE 4 CASE 3
    model.circles.clear();
    for l in 0..model.lines.len() {
        let p1 = model.lines[l].start;
        let p2 = model.lines[l].end;

        // the center of a circle that is perpendicular to the disk and touches one point on said disk is the inverse of the point
        // the center of a cricle that is perpendicular to the disk and touches two points on said disk is the intersection of the perpendicular bisectors of the lines between the points and they're inverses

        // in both cases the radius is just the distance from one of the points.

        // find the inverse of the points
        let inverse_p1 = p1.find_inverse(model.circle_radius).unwrap_or(Point::from(vec2(0.10, 0.10)));
        let inverse_p2 = p2.find_inverse(model.circle_radius).unwrap_or(Point::from(vec2(0.10, 0.10)));

        // find the middle of the line between the points and they're inverses
        let middle_p1_to_inverse = (p1.pos + inverse_p1.pos)/2.0;
        let middle_p2_to_inverse = (p2.pos + inverse_p2.pos)/2.0;

        // find the perpendicular bisector of the line between the points and they're inverses as an algebraic function
        let a_to_inverse_perpendicular_bisector_algebraic = Line::from_points(p1, inverse_p1).tangent_function_from(middle_p1_to_inverse);
        let b_to_inverse_perpendicular_bisector_algebraic = Line::from_points(p2, inverse_p2).tangent_function_from(middle_p2_to_inverse);

        // convert the algebraic function to a line
        let a_to_inverse_perpendicular_bisector = Line::function_to_line(&a_to_inverse_perpendicular_bisector_algebraic, -1000.0, 1000.0);
        let b_to_inverse_perpendicular_bisector = Line::function_to_line(&b_to_inverse_perpendicular_bisector_algebraic, -1000.0, 1000.0);

        // find the intersection of the two lines
        let intersection = a_to_inverse_perpendicular_bisector.algebraic_intersection(&b_to_inverse_perpendicular_bisector).unwrap_or( Point::from(vec2(0.0, 0.0)));
        
        // find the radius
        let radius = (p1.pos - intersection.pos).length();


        // add the circle to the model
        model.circles.push((radius, intersection, model.lines[l]));
    }

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(Rgb8::new	(13,9,40));

    draw.ellipse().radius(model.circle_radius).color(Rgb8::new(45,39,102)).stroke_color(Rgb8::new(86,82,171)).stroke_weight(1.0);

    // draw the lines' points
    for line in &model.lines {
        let p1 = line.start;
        let p2 = line.end;
        p1.draw(&draw, model.node_size, None);
        p2.draw(&draw, model.node_size, None);
    }

    // draw segments of the perpendicular circles that range from one point_a to point_b
    for circle in &model.circles {

        if !model.segment_or_circle{
            let segments = generate_segment(circle.0, circle.1, circle.2.start, circle.2.end);
            draw.polyline().points(segments).color(Rgb8::new(97,189,172));
        } else {
            draw.ellipse().radius(circle.0).xy(circle.1.pos).no_fill().stroke_color(Rgb8::new(86,82,171)).stroke_weight(1.0);
        }
    }


    { // debug
    // test
    // let p1 = model.lines[0].start;
    // let p2 = model.lines[0].end;

    // let inverse_p1 = find_inverse(p1, model.circle_radius).unwrap_or(Point::from(vec2(0.10, 0.10)));
    // let inverse_p2 = find_inverse(p2, model.circle_radius).unwrap_or(Point::from(vec2(0.10, 0.10)));

    // inverse_p1.draw(&draw,  Some("inverse_p1"));
    // inverse_p2.draw(&draw, Some("inverse_p2"));

    // let middle_p1_to_inverse = (p1.pos + inverse_p1.pos)/2.0;
    // let middle_p2_to_inverse = (p2.pos + inverse_p2.pos)/2.0;

    // Point::from(middle_p1_to_inverse).draw(&draw, Some("middle_p1_to_inverse"));
    // Point::from(middle_p2_to_inverse).draw(&draw, Some("middle_p2_to_inverse"));

    // let a_to_inverse_perpendicular_bisector_algebraic = Line::from_points(p1, inverse_p1).tangent_function_from(middle_p1_to_inverse);
    // let b_to_inverse_perpendicular_bisector_algebraic = Line::from_points(p2, inverse_p2).tangent_function_from(middle_p2_to_inverse);



    // let a_to_inverse_perpendicular_bisector = Line::function_to_line(&a_to_inverse_perpendicular_bisector_algebraic, -1000.0, 1000.0);
    // let b_to_inverse_perpendicular_bisector = Line::function_to_line(&b_to_inverse_perpendicular_bisector_algebraic, -1000.0, 1000.0);

    // a_to_inverse_perpendicular_bisector.draw(&draw);
    // b_to_inverse_perpendicular_bisector.draw(&draw);

    // let intersection = a_to_inverse_perpendicular_bisector.algebraic_intersection(&b_to_inverse_perpendicular_bisector).unwrap_or( Point::from(vec2(0.0, 0.0)));
    // let radius = (p1.pos - intersection.pos).length();

    // draw.ellipse().radius(radius).xy(intersection.pos).color(Rgba8::new(255, 255, 255, 0)).stroke_weight(1.0).stroke_color(BLACK);
    }
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

// draw lines function
fn draw_lines<'a>(app: &App, draw: &Draw, model: &mut Model, sections: u32){


    let mut touching_points: Vec<(Point, usize)> = Vec::new();
    let mut length:usize = 0;
    let clone_help = model.lines.clone();
    for l in 0..clone_help.len() {

        if model.lines[l].start.pos.distance(app.mouse.position()) < model.node_size {
            touching_points.push((clone_help[l].start, l));
            length+=1;
        }
        if model.lines[l].end.pos.distance(app.mouse.position()) < model.node_size {
            touching_points.push((clone_help[l].end, l));
            length+=1;
        }
    }


    if touching_points.len() > 0 && app.mouse.buttons.left().is_down() && model.mouse_down{
        model.create_line.0 = Some(touching_points[0].0.pos);
        model.mouse_down = false;
        model.lines.push(Line::from_points(Point::from(model.create_line.0.unwrap()), Point::from(app.mouse.position())));

    }
    else if touching_points.len() > 0 && app.mouse.buttons.left().is_up() && !model.mouse_down{
        model.create_line.1 = Some(touching_points[0].0.pos);
        model.mouse_down = true;
        model.lines.last_mut().unwrap().end.pos = model.create_line.1.unwrap();

        model.create_line.1 = None;
        model.create_line.0 = None;

    }
    else if app.mouse.buttons.left().is_down() && model.mouse_down && model.create_line.0.is_none(){
        model.create_line.0 = Some(app.mouse.position());
        model.mouse_down = false;
        model.lines.push(Line::from_points(Point::from(model.create_line.0.unwrap()), Point::from(app.mouse.position())));

    }
    else if app.mouse.buttons.left().is_up() && !model.mouse_down && model.create_line.1.is_none(){
        model.create_line.1 = Some(app.mouse.position());

        model.lines.last_mut().unwrap().end.pos = model.create_line.1.unwrap();

        model.mouse_down = true;
        model.create_line.0 = None;
        model.create_line.1 = None;

        
    }

    
    // show line while creating
    if app.mouse.buttons.left().is_down() && model.create_line.0.is_some() {
        model.lines.last_mut().unwrap().end.pos = app.mouse.position();
    }

    // remove line thats touching
    if app.mouse.buttons.right().is_down() && model.lines.len() > 0 && length > 0 && !app.mouse.buttons.left().is_down() {
        model.lines.remove(touching_points[0].1);
        
    }
}

// idk how this works but it works...
pub fn generate_segment(radius: f32, center: Point, point_a: Point, point_b: Point) -> Vec<Vec2> {
    let mut points = vec![];
    let mut a = point_a.pos - center.pos;
    let mut b = point_b.pos - center.pos;
    // be will always be the smaller angle
    if a.angle() < b.angle() {
        let temp = b;
        b = a;
        a = temp;
        
    }

    let angle_between = a.angle_between(b);
    
    for i in 0..101 {

        let t = i as f32 / 100.0;
        let angle = lerp(b.angle()..=b.angle()+angle_between, t)-angle_between;

        let x = center.pos.x + radius * angle.cos();
        let y = center.pos.y + radius * angle.sin();
        points.push(vec2(x, y));
    }
    points
    

}




