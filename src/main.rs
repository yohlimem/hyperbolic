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


    // circle_color: Rgba8,
    // line_color: Rgba8,
    // node_color: Rgba8,
    // background_color: Rgba8,

    // circle_color_c32: Color32,
    // line_color_c32: Color32,
    // node_color_c32: Color32,
    // background_color_c32: Color32,
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


        // circle_color: BLANCHEDALMOND.into(),
        // line_color: Rgba8::new(100, 100, 100, 255),
        // node_color: Rgba8::new(255, 255, 255, 0),
        // background_color: Rgba8::new(14, 48, 87, 255),

        // color 32circle_color
        // circle_color_c32: Color32::from_rgba_premultiplied(255, 255, 255, 255),
        // line_color_c32: Color32::from_rgba_premultiplied(100, 100, 100, 255),
        // node_color_c32: Color32::from_rgba_premultiplied(255, 255, 255, 255),
        // background_color_c32: Color32::from_rgba_premultiplied(14, 48, 87, 255),
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
            
            // egui::widgets::color_picker::color_edit_button_srgba(ui, &mut model.line_color_c32, Alpha::BlendOrAdditive);
            // egui::widgets::color_picker::color_edit_button_srgba(ui, &mut model.node_color_c32, Alpha::BlendOrAdditive);
            // egui::widgets::color_picker::color_edit_button_srgba(ui, &mut model.background_color_c32, Alpha::BlendOrAdditive);
            // egui::widgets::color_picker::color_edit_button_srgba(ui, &mut model.circle_color_c32, Alpha::BlendOrAdditive);
            
    });}
    // model.circle_color = convert_nannou_color_to_egui_color_mut(model.circle_color_c32);
    // model.line_color = convert_nannou_color_to_egui_color_mut(model.line_color_c32);
    // model.node_color = convert_nannou_color_to_egui_color_mut(model.node_color_c32);
    // model.background_color = convert_nannou_color_to_egui_color_mut(model.background_color_c32);
        
        // println!(Some"{}Some", model.num);
        // model.lines[0].start.pos = app.mouse.position();
        
        // model.points.clear();
        draw_lines(&app, &app.draw(), model, 255);
    // middle of circle = 0.0, 0.0
    // TODO: https://math.stackexchange.com/a/1322475
    // https://www.youtube.com/watch?v=xHCEDMQDbzk
    // https://www.ms.uky.edu/~droyster/courses/spring08/math6118/classnotes/chapter09.pdf PAGE 4 CASE 3
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
    draw.background().color(Rgb8::new	(13,9,40));

    draw.ellipse().radius(model.circle_radius).color(Rgb8::new(45,39,102)).stroke_color(Rgb8::new(86,82,171)).stroke_weight(1.0);

    
    for line in &model.lines {
        // line.draw(&draw);
        let p1 = line.start;
        let p2 = line.end;
        p1.draw(&draw, model.node_size, None);
        p2.draw(&draw, model.node_size, None);
    }
    for circle in &model.circles {
        // draw.ellipse().radius(circle.0).xy(circle.1.pos).color(Rgb8::newa8::new(255, 255, 255, 0)).stroke_weight(1.0).stroke_color(BLACK);
        let segments = generate_segment(circle.0, circle.1, circle.2.start, circle.2.end);
        // circle.2.start.draw(&draw, Some("a"));
        // circle.2.end.draw(&draw, Some("b"));
        // Line::from_points(Point{pos: Vec2::ZERO},circle.2.start).draw(&draw);
        // Line::from_points(Point{pos: Vec2::ZERO},circle.2.end).draw(&draw);
        // draw.line().end(vec2(-1000.0, 0.0)).start(vec2(1000.0, 0.0));
        // println!("{:?}", segments);

        draw.polyline().points(segments).color(Rgb8::new(97,189,172));
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
    let mut touching_points: Vec<(&Point, usize)> = Vec::new();
    let mut length:usize = 0;

    for l in 0..model.lines.len() {

        if model.lines[l].start.pos.distance(app.mouse.position()) < model.node_size {
            touching_points.push((&model.lines[l].start, l));
            length+=1;

        }
        if model.lines[l].end.pos.distance(app.mouse.position()) < model.node_size {
            touching_points.push((&model.lines[l].end, l));
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

        // println!("connecting, end");
        model.create_line.1 = None;
        model.create_line.0 = None;

    }
    else if app.mouse.buttons.left().is_down() && model.mouse_down && model.create_line.0.is_none(){
        model.create_line.0 = Some(app.mouse.position());
        model.mouse_down = false;
        model.lines.push(Line::from_points(Point::from(model.create_line.0.unwrap()), Point::from(app.mouse.position())));

        // println!(" to mouse, start");
    }
    else if app.mouse.buttons.left().is_up() && !model.mouse_down && model.create_line.1.is_none(){
        model.create_line.1 = Some(app.mouse.position());

        // model.points.push(Point::from(model.create_line.0.unwrap().x, model.create_line.0.unwrap().y));
        // model.points.push(Point::from(model.create_line.1.unwrap().x, model.create_line.1.unwrap().y));
        model.lines.last_mut().unwrap().end.pos = model.create_line.1.unwrap();

        model.mouse_down = true;
        // println!("to mouse, end");
        model.create_line.0 = None;
        model.create_line.1 = None;

        
    }

    // draw line while creating
    if app.mouse.buttons.left().is_down() && model.create_line.0.is_some() {
        model.lines.last_mut().unwrap().end.pos = app.mouse.position();
    }

    if app.mouse.buttons.right().is_down() && model.lines.len() > 0 && length > 0 && !app.mouse.buttons.left().is_down() {
        model.lines.pop();
        
    }
}

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
    // println!("angle: {:?}", a.angle().rad_to_deg());
    // println!("angle: {:?}", a.angle_between(b).rad_to_deg());

    let angle_between = a.angle_between(b);
    
    for i in 0..101 {

        let t = i as f32 / 100.0;
        // let angle = b.angle() + angle_between * t;
        let angle = lerp(b.angle()..=b.angle()+angle_between, t)-angle_between;

        let x = center.pos.x + radius * angle.cos();
        let y = center.pos.y + radius * angle.sin();
        points.push(vec2(x, y));
    }
    points
    

}

fn convert_nannou_color_to_egui_color_mut(color: Color32) -> nannou::color::Rgba8 {
    nannou::color::Rgba8::new(color.r(), color.g(), color.b(), color.a())
}



