use gtk::{Inhibit, WidgetExt, Window, WindowType};
use relm::{Relm, Update, Widget};

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

pub struct App {
    model: Model,
    window: Window,
}

impl Update for App {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        window.show_all();

        App { model, window }
    }
}
