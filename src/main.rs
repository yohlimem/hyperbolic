use std::vec;

use lines::Line;
use nannou::{ease::circ, geom::point, math::ConvertAngle, prelude::*};
use nannou_egui::{self, egui::{self, lerp}, Egui};

mod points;
mod lines;
use points::Point;

struct Model {
    // window: Window,
    egui: Egui,
    circle_radius: f32,
    points: Vec<Point>,
    mouse_down: bool,
    create_line: (Option<Vec2>, Option<Vec2>),
    circles: Vec<(f32, Point, Line)>,
    lines: Vec<Line>,
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let circle_radius = 300.0;
    Model {
        egui,
        circle_radius,
        points: vec![],
        circles: vec![],
        create_line: (None, None),
        mouse_down: true,
        lines: vec![]
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    {let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum windowSome").show(&ctx, |ui| {
        ui.label("radius");
        ui.add(egui::Slider::new(&mut model.circle_radius, 50.0..=2000.0));
    });}

    // println!(Some"{}Some", model.num);
    // model.lines[0].start.pos = app.mouse.position();

    // model.points.clear();
    draw_lines(&app, &app.draw(), model, 255);
    // middle of circle = 0.0, 0.0
    // TODO: https://math.stackexchange.com/a/1322475
    // https://www.youtube.com/watch?v=xHCEDMQDbzk
    model.circles.clear();
    for l in 0..model.lines.len() {
        let p1 = model.lines[l].start;
        let p2 = model.lines[l].end;

        let inverse_p1 = p1.find_inverse(model.circle_radius).unwrap_or(Point::from(vec2(0.10, 0.10)));
        let inverse_p2 = p2.find_inverse(model.circle_radius).unwrap_or(Point::from(vec2(0.10, 0.10)));


        let middle_p1_to_inverse = (p1.pos + inverse_p1.pos)/2.0;
        let middle_p2_to_inverse = (p2.pos + inverse_p2.pos)/2.0;


        let a_to_inverse_perpendicular_bisector_algebraic = Line::from_points(p1, inverse_p1).tangent_function_from(middle_p1_to_inverse);
        let b_to_inverse_perpendicular_bisector_algebraic = Line::from_points(p2, inverse_p2).tangent_function_from(middle_p2_to_inverse);



        let a_to_inverse_perpendicular_bisector = Line::function_to_line(&a_to_inverse_perpendicular_bisector_algebraic, -1000.0, 1000.0);
        let b_to_inverse_perpendicular_bisector = Line::function_to_line(&b_to_inverse_perpendicular_bisector_algebraic, -1000.0, 1000.0);

        let intersection = a_to_inverse_perpendicular_bisector.algebraic_intersection(&b_to_inverse_perpendicular_bisector).unwrap_or( Point::from(vec2(0.0, 0.0)));
        let radius = (p1.pos - intersection.pos).length();

        model.circles.push((radius, intersection, model.lines[l]));
    }

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.ellipse().radius(model.circle_radius);
    for p in &model.points {
        p.draw(&draw, None);
        // p.to_hyperbolic_point().draw(&draw);
    }

    for circle in &model.circles {
        // draw.ellipse().radius(circle.0).xy(circle.1.pos).color(Rgba8::new(255, 255, 255, 0)).stroke_weight(1.0).stroke_color(BLACK);
        let segments = generate_segment(circle.0, circle.1, circle.2.start, circle.2.end);

        // println!("{:?}", segments);

        draw.polyline().points(segments).color(Rgba8::new(100, 100, 100, 255));
    }

    for line in &model.lines {
        // line.draw(&draw);
        let p1 = line.start;
        let p2 = line.end;
        p1.draw(&draw, None);
        p2.draw(&draw, None);
    }
    {
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

fn draw_lines<'a>(app: &App, draw: &Draw, model: &mut Model, sections: u32){
    let mut touching_points: Vec<&Point> = Vec::new();

    for l in &model.lines {

        if l.start.pos.distance(app.mouse.position()) < 10.0 {
            touching_points.push(&l.start);
        }
        if l.end.pos.distance(app.mouse.position()) < 10.0 {
            touching_points.push(&l.end);
        }
    }


    if touching_points.len() > 0 && app.mouse.buttons.left().is_down() && model.mouse_down{
        model.create_line.0 = Some(touching_points[0].pos);
        model.mouse_down = false;
        // println!("connecting, start");
    }
    else if touching_points.len() > 0 && app.mouse.buttons.left().is_up() && !model.mouse_down{
        model.create_line.1 = Some(touching_points[0].pos);
        model.mouse_down = true;
        model.lines.push(Line::from_points(Point::from(model.create_line.0.unwrap()), Point::from(model.create_line.1.unwrap())));

        // println!("connecting, end");
        model.create_line.1 = None;
        model.create_line.0 = None;

    }
    else if app.mouse.buttons.left().is_down() && model.mouse_down && model.create_line.0.is_none(){
        model.create_line.0 = Some(app.mouse.position());
        model.mouse_down = false;
        // println!(" to mouse, start");
    }
    else if app.mouse.buttons.left().is_up() && !model.mouse_down && model.create_line.1.is_none(){
        model.create_line.1 = Some(app.mouse.position());

        // model.points.push(Point::from(model.create_line.0.unwrap().x, model.create_line.0.unwrap().y));
        // model.points.push(Point::from(model.create_line.1.unwrap().x, model.create_line.1.unwrap().y));
        model.lines.push(Line::from_points(Point::from(model.create_line.0.unwrap()), Point::from(model.create_line.1.unwrap())));
        // for t in 0..sections {
        //     let x = lerp(model.create_line.0.unwrap().x..=app.mouse.x, t as f32/sections as f32);
        //     let y = line_to_algebra(model.create_line.0.unwrap(), model.create_line.1.unwrap(), x);
        //     model.points.push(Point::from(x, y));
        // }
        // model.shapes.push(Shape::Line(Line::from(model.create_line.0.unwrap(), model.create_line.1.unwrap(), 1.0, Rgba8::new(0, 0, 0, 255))));
        model.mouse_down = true;
        // println!("to mouse, end");
        model.create_line.0 = None;
        model.create_line.1 = None;

        
    }
    // println!("{:?}", model.create_line);
}

pub fn generate_segment(radius: f32, center: Point, point_a: Point, point_b: Point) -> Vec<Vec2> {
    let mut points = vec![];
    let mut a = point_a.pos - center.pos;
    let mut b = point_b.pos - center.pos;


    for i in 0..101 {

        let t = i as f32 / 100.0;
        let angle = lerp(a.angle()..=b.angle(), t);
        // println!("angle: {:?}", angle);

        let x = center.pos.x + radius * angle.cos();
        let y = center.pos.y + radius * angle.sin();
        points.push(vec2(x, y));
    }
    points
    

}



