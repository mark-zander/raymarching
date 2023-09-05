use std::iter;
use winit::window::Window;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
// use multiarray::*;

pub mod cli;
pub mod display;
pub mod pipeline;
pub mod div_size;
pub mod texture;

struct State {
    display: display::Display,
    div_size: div_size::Data,
    cube_texture: texture::Texture,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    async fn new(window: Window, args: &cli::Args) -> Self {
        let display = display::Display::new(window).await.unwrap();
        let div_size = div_size::Data::new(display.size(), display.device());
        let color_cube = texture::ColorCube::new();
        let cube_texture = texture::Texture::mk_cube(
            display.device(), display.queue(),
            color_cube, "Color Cube"
        );
        // let cube_bind_group_layout = cube_texture.bind_group_layout;
        // let cube_bind_group = cube_texture.bind_group;

        let render_pipeline = pipeline::make(&display, args, &[
            &div_size.layout,
            &cube_texture.bind_layout,
        ]);
        Self {
            display,
            div_size,
            cube_texture,
            render_pipeline,
        }
    }
    fn display(&self) -> &display::Display { &self.display }

    pub fn surface(&self) -> &wgpu::Surface { &self.display.surface() }

    pub fn window(&self) -> &Window { &self.display.window }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> { self.display.size }

    pub fn device(&self) -> &wgpu::Device { &self.display.device }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.display.config
    }

    pub fn queue(&self) -> &wgpu::Queue { &self.display.queue }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.display.resize(new_size);
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.display.input(event)
    }

    pub fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.display.surface().get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.div_size.bind, &[]);
            render_pass.set_bind_group(1, &self.cube_texture.bind, &[]);
            render_pass.draw(0..6, 0..1);
        }

        self.queue().submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }


}

pub async fn run(cli: &cli::Cli) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // let mut display = display::Display::new(window).await.unwrap();
    let mut state = State::new(window, &cli.args()).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });

}

