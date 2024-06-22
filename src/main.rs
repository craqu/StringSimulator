use modules::corde::Corde;
use modules::screen_view::Framework;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
mod modules;
use modules::corde;
use modules::screen_view;
use sdl2::sys::SDL_Delay;

fn remap(value: f64, low1: f64, high1: f64, low2: f64, high2: f64) -> i32 {
    let res = low2 + (value - low1) * (high2 - low2) / (high1 - low1);
    return res as i32;
}
fn get_points(corde: &Corde, screen: &Framework) -> Box<Vec<Point>> {
    let mut points: Box<Vec<Point>> = Box::new(vec![Point::new(0, 0); corde.n as usize]);

    let xpadding = screen.screen_width / 12;
    let ypadding = screen.screen_height / 12;
    let ymax = 1.0 / (10_f64.powi(6));

    for i in 0..(corde.n - 1.0) as usize {
        let remapped_x = remap(
            i as f64,
            0.0,
            corde.n - 1.0,
            xpadding as f64,
            (screen.screen_width - xpadding) as f64,
        );
        let remapped_y = remap(
            corde.y[i],
            -ymax,
            ymax,
            ypadding as f64,
            (screen.screen_height - ypadding) as f64,
        );
        points[i] = Point::new(remapped_x, remapped_y);
    }
    return points;
}

fn main() {
    let mut corde = corde::build_corde();
    let mut screen = screen_view::build_framework();
    corde.pluck(5, 0.1);
    corde.replace();
    screen.canvas.set_draw_color(Color::RGB(0, 5, 2));

    screen.canvas.clear();

    while screen.running {
        for event in screen.event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    screen.running = false;
                }

                // Event::MouseMotion {
                //     x, y, xrel, yrel, .. } => {

                //         println!("Mouse x: {}, y: {}", x, y);
                //     },
                _ => {}
            }
        }
        screen.canvas.clear();
        screen.canvas.set_draw_color(Color::RGB(0, 5, 2));
        screen.canvas.fill_rect(screen.screen_area).ok();
        screen.canvas.set_draw_color(Color::RGB(255, 210, 0));

        corde.compute_single();
        let points = get_points(&corde, &screen);

        //screen.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..(points.len() - 2) {

            screen.canvas.draw_line(points[i], points[i + 1]).unwrap();
        }

        screen.canvas.present();
        unsafe { SDL_Delay(5) }
    }
}
