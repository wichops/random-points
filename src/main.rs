use core::time::Duration;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

fn main() {
    nannou::app(model).update(update).run();
}

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const SIZE: f32 = 4.0;
const PADDING: f32 = -10.0;

#[derive(Default, Debug, Clone)]
struct Point {
    speed_x: f32,
    speed_y: f32,
    x: f32,
    y: f32,
}
struct Model {
    points: Vec<Point>,
    random_range: StdRng,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut points: Vec<Point> = vec![Default::default(); 400];
    for (index, p) in points.iter_mut().enumerate() {
        p.x = (index as f32 * 1.8) - 380.0;
    }

    let random_seed = random_range(0, 100000);
    let random_range = StdRng::seed_from_u64(random_seed);
    Model {
        points,
        random_range,
    }
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }

        _other_key => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut speed_x_sum = 0.0;
    let mut speed_y_sum = 0.0;

    for (index, mut point) in model.points.iter_mut().enumerate() {
        let range_x = model.random_range.gen_range(-0.5..0.5);
        let range_y = model.random_range.gen_range(-1.0..1.0);

        point.speed_x += range_x;
        point.speed_y += range_y;
        speed_x_sum += point.speed_x;
        speed_y_sum += point.speed_y;

        let special_speed_x = speed_x_sum / (index + 1) as f32;
        let special_speed_y = speed_y_sum / (index + 1) as f32;

        if point.y > -300.0 + PADDING && point.y < 300.0 - PADDING {
            point.y += special_speed_y;
            point.x += special_speed_x;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.duration.since_start < Duration::from_millis(10) {
        draw.background().color(rgba(1.0, 1.0, 1.0, 1.0));
    }

    for p in &model.points {
        draw.ellipse()
            .y(p.y)
            .x(p.x)
            .w_h(SIZE, SIZE)
            .color(rgba(0.0, 0.0, 0.0, 0.2));
    }

    draw.to_frame(app, &frame).unwrap();
}
