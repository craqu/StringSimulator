// use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Framework {
    pub screen_width: u32,
    pub screen_height: u32,
    pub sdl_context: sdl2::Sdl,
    pub screen_area: Rect,
    pub clear_color: Color,
    pub running: bool,
    pub event_queue: sdl2::EventPump,
    pub canvas: Canvas<Window>,
}
pub fn build_framework() -> Framework {
    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video();
    let window = video_subsystem
        .expect("fucked up in video subsystem")
        .window("Design II en Rust!", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let screen_area: Rect = Rect::new(0, 0, screen_width, screen_height);
    let clear_color: Color = Color::RGB(64, 192, 200);
    canvas.set_draw_color(clear_color);

    let running: bool = true;
    let event_queue: sdl2::EventPump = sdl_context.event_pump().unwrap();

    Framework {
        screen_area,
        screen_height,
        screen_width,
        sdl_context,
        canvas,
        clear_color,
        event_queue,
        running,
    }
}
