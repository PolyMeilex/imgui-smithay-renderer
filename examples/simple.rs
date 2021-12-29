use smithay::backend::{
    renderer::Transform,
    winit::{self, WinitEvent},
};

pub fn main() {
    let (mut renderer, mut input) = winit::init(None).unwrap();

    let size = renderer.window_size().physical_size;

    let mut imgui = imgui::Context::create();
    {
        imgui.set_ini_filename(None);
        let io = imgui.io_mut();
        io.display_framebuffer_scale = [1.0f32, 1.0f32];
        io.display_size = [size.w as f32, size.h as f32];
    }

    let imgui_pipeline = renderer
        .renderer()
        .with_context(|_, gles| imgui_smithay_renderer::Renderer::new(gles, &mut imgui))
        .unwrap();

    loop {
        let res = input.dispatch_new_events(|event| match event {
            WinitEvent::Resized { size, scale_factor } => {
                let io = imgui.io_mut();
                io.display_framebuffer_scale = [scale_factor as f32, scale_factor as f32];
                io.display_size = [size.w as f32, size.h as f32];
            }
            WinitEvent::Input(_event) => {}
            _ => {}
        });

        match res {
            Ok(()) => {
                let ui = imgui.frame();

                imgui::Window::new("Root")
                    .size([size.w as f32, size.h as f32], imgui::Condition::Always)
                    .position([0.0, 0.0], imgui::Condition::Always)
                    .title_bar(false)
                    .build(&ui, || {
                        ui.text(&format!("Workspace: {}", 1));
                        ui.text(&format!("Geo: {:#?}", 0));
                    });

                renderer
                    .render(|renderer, _| {
                        let draw_data = ui.render();

                        renderer
                            .with_context(|_renderer, gles| {
                                imgui_pipeline.render(Transform::Normal, gles, draw_data);
                            })
                            .unwrap();
                    })
                    .unwrap();
            }
            Err(winit::WinitError::WindowClosed) => {
                break;
            }
        }
    }
}
