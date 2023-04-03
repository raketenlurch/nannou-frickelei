// Anatomy of a Nannou App
/*use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        //.event(event)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

// fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, _frame: Frame) {}
*/

// Draw a Sketch
/*use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}*/

// Basics - sketch vs app (Sketch)
/*use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}*/

// Basics - sketch vs app (App)
/*use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}*/

// Basics - Window Coordinates
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    // 100-step and 10-step grids
    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);

    // Crosshair
    let crosshair_color = gray(0.5);
    let ends = [
        win.mid_top(),
        win.mid_right(),
        win.mid_bottom(),
        win.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }

    // Crosshair text
    let top = format!("{:.1}", win.top());
    let bottom = format!("{:.1}", win.bottom());
    let left = format!("{:.1}", win.left());
    let right = format!("{:.1}", win.right());
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);
    draw.text(&top)
        .h(win.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);

    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);
    draw.rect().x_y(100.0, 100.0).w_h(100.0, 100.0).color(BLUE);
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0)
        .z_degrees(45.0)
        .color(BLUEVIOLET);
    draw.rect()
        .x_y(100.0, 100.0)
        .w_h(100.0, 100.0)
        .z_degrees(45.0)
        .color(BLUEVIOLET);
    draw.rect().x_y(-50.0, -50.0).w_h(-100.0, -100.0).color(RED);

    let side = 100.0;
    let top_left = pt2(-300.0, 200.0);
    let offset = vec2(side / 2.0, -side / 2.0);
    let xy = top_left + offset;
    draw.rect().xy(xy).w_h(side, side).color(PLUM);

    let r = Rect::from_w_h(100.0f32, 100.0f32).top_left_of(win);
    draw.rect().xy(r.xy()).wh(r.wh()).color(PLUM);

    let win_p = win.pad(25.0);
    draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.5));
    let r = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);
    draw.rect().xy(r.xy()).wh(r.wh()).color(PALEGOLDENROD);

    let pad = 25.0;
    let win_p = win.pad(25.0);
    let square = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);
    draw.rect().xy(square.xy()).wh(square.wh()).color(PLUM);

    let circle = square.below(square).shift_y(-pad);
    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);

    // Window and monitor details
    if let Some(monitor) = window.current_monitor() {
        let w_scale_factor = window.scale_factor();
        let m_scale_factor = monitor.scale_factor();
        let mon_phys = monitor.size();
        let mon = mon_phys.to_logical(w_scale_factor as f64);
        let mon_w: f32 = mon.width;
        let mon_h: f32 = mon.height;
        let text = format!(
            "
            Window size: [{:.0}, {:.0}]
            Window ratio: {:.2}
            Window scale factor: {:.2}
            Monitor size: [{:.0}, {:.0}]
            Monitor ratio: {:.2}
            Monitor scale factor: {:.2}
            ",
            win.w(),
            win.h(),
            win.w() / win.h(),
            w_scale_factor,
            mon_w,
            mon_h,
            mon_w / mon_h,
            m_scale_factor,
        );
        let pad = 6.0;
        draw.text(&text)
            .h(win.pad(pad).h())
            .w(win.pad(pad).w())
            .line_spacing(pad)
            .font_size(14)
            .align_text_bottom()
            .color(crosshair_color)
            .left_justify();

        // Ellipse at mouse
        draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

        // Mouse position text
        let mouse = app.mouse.position();
        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}
