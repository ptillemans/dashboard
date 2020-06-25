use gtk::Label;
use gtk::Orientation::Vertical;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

struct Model {
    greeting: String,
}

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Win {
    model: Model,
    window: Window,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            greeting: "Hello, World!".to_string(),
        }
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        // First we get the file content.
        let glade_src = include_str!("dashboard.glade");
        // Then we call the Builder call.
        let builder = gtk::Builder::new_from_string(glade_src);

        let label: gtk::Label = builder.get_object("label1").unwrap();
        label.set_text(&model.greeting);

        let canvas: gtk::DrawingArea = builder.get_object("canvas1").unwrap();
        canvas.connect_draw(move |_, context| {
            context.set_source_rgb(0.2, 0.4, 0.0);
            context.paint();

            context.set_font_size(60.0);
            context.set_source_rgb(0.0, 0.0, 0.0);
            context.move_to(100.0, 100.0);
            context.show_text("Hello!!!");
            Inhibit(false)
        });
        let window: gtk::Window = builder.get_object("window1").unwrap();

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win { model, window }
    }
}

fn main() {
    Win::run(()).unwrap();
}
