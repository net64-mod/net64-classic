use azul::azul_dependencies::glium::Surface;
use azul::prelude::{App as AzulApp, *};

pub struct App {
    azul_app: AzulApp<OpenGlAppState>,
    window: Window<OpenGlAppState>,
}

impl App {
    pub fn new() {
        let mut azul_app = AzulApp::new(OpenGlAppState {}, AppConfig::default()).unwrap();
        let window = azul_app
            .create_window(WindowCreateOptions::default(), css::native())
            .unwrap();
        azul_app.run(window).unwrap();
    }
}

struct OpenGlAppState {}

impl Layout for OpenGlAppState {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        Dom::gl_texture(
            GlTextureCallback(render_my_texture),
            StackCheckedPointer::new(self, self).unwrap(),
        )
    }
}

fn render_my_texture(
    state: &StackCheckedPointer<OpenGlAppState>,
    info: LayoutInfo<OpenGlAppState>,
    hi_dpi_bounds: HidpiAdjustedBounds,
) -> Option<Texture> {
    let physical_size = hi_dpi_bounds.get_physical_size();
    let texture = info
        .window
        .read_only_window()
        .create_texture(physical_size.width as u32, physical_size.height as u32);

    texture.as_surface().clear_color(1.0, 0.0, 0.0, 1.0);
    Some(texture)
}