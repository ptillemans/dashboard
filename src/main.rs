use gtk::Label;
use gtk::Orientation::Vertical;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use cairo;

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
        let builder = get_glade_builder();
        init_label(&builder, &model);
        init_drawing_area(&builder);
        let window = init_window(&builder, &relm);

        window.show_all();

        Win { model, window }
    }
}

fn get_glade_builder() -> gtk::Builder {
    // GTK+ widgets are used normally within a `Widget`.
    // First we get the file content.
    let glade_src = include_str!("dashboard.glade");
    // Then we call the Builder call.
    gtk::Builder::new_from_string(glade_src)

}

fn init_label(builder: &gtk::Builder, model: &Model) -> () {
    let label: gtk::Label = builder.get_object("label1").unwrap();
    label.set_text(&model.greeting);

}

fn init_drawing_area(builder: &gtk::Builder) -> (){
    let canvas: gtk::DrawingArea = builder.get_object("canvas1").unwrap();
    canvas.connect_draw(move |_, context| {
        draw_drawing_area(context);
        Inhibit(false)
    });
}

fn init_window(builder: &gtk::Builder, relm: &Relm<Win>) -> gtk::Window {
    let window: gtk::Window = builder.get_object("window1").unwrap();
    // Connect the signal `delete_event` to send the `Quit` message.
    connect!(
        relm,
        window,
        connect_delete_event(_, _),
        return (Some(Msg::Quit), Inhibit(false))
    );
    window
}

fn draw_drawing_area(context: &cairo::Context) -> () {
    context.set_source_rgb(0.2, 0.4, 0.0);
    context.paint();

    context.set_font_size(60.0);
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.move_to(100.0, 100.0);
    context.show_text("Hello!!!");
}

fn main() {
    Win::run(()).unwrap();
}
