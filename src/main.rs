use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(640, 640)
        .run();
}

struct Model {
    texture: wgpu::Texture,
}

fn model(_app: &App) -> Model {
    let assets = _app.assets_path().unwrap();
    let img_path = assets.join("cat.png");
    let texture = wgpu::Texture::from_path(_app, img_path).unwrap();
    Model { texture }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let window = _app.main_window();
    let rect = window.rect();

    let draw = _app.draw();

    draw.background().color(BLACK);

    draw_bg(&draw, _app.time, &rect);
    draw_cat(&draw, _app.time, &rect, &_model.texture);

    draw.to_frame(_app, &frame).unwrap();
}

fn draw_bg(draw: &Draw, time: f32, rect: &Rect) {
    let step = f32::min(rect.w(), rect.h()) / 20.0;

    let step_by = || (0..).map(|i| i as f32 * step);

    let t_iter = step_by().take_while(|&f| f < rect.top()).map(|f| f + 0.5 * step);
    let b_iter = step_by().take_while(|&f| f < -rect.bottom()).map(|f| -f - 0.5 * step);
    let y_iter = t_iter.chain(b_iter);

    for y in y_iter {
        let r_iter = step_by().take_while(|&f| f < rect.right()).map(|f| f + 0.5 * step);
        let l_iter = step_by().take_while(|&f| f < -rect.left()).map(|f| -f - 0.5 * step);
        let x_iter = r_iter.chain(l_iter);

        for x in x_iter {
            let phase = vec2(x, y).magnitude();

            draw.rect()
                .color(cat_color(phase / step - 4.0 * PI * time))
                .x_y(x, y)
                .w_h(step, step);
        }
    }
}

fn draw_cat(draw: &Draw, time: f32, rect: &Rect, texture: &wgpu::Texture) {
    let size = f32::min(rect.w(), rect.h()) * (0.9 + 0.1 * (PI * time).sin());

    draw.texture(texture)
        .w_h(size, size);
}

fn cat_color(t: f32) -> Rgba {
    return rgba(
        0.5 + 0.5 * t.cos(),
        0.5 + 0.5 * (t + PI / 3.0 * 2.0).cos(),
        0.5 + 0.5 * (t + PI / 3.0 * 4.0).cos(),
        1.0
    );
}