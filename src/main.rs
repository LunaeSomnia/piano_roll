use vizia::prelude::*;

pub mod util;
pub use util::*;

pub mod components;
pub use components::*;

#[derive(Lens)]
pub struct AppData {
    // // Start time of the clip being edited by the piano roll.
    // pub clip_start: MusicalTime,
    // // End time of the clip being edited by the piano roll.
    // pub clip_end: MusicalTime,
    pub notes: Vec<NoteData>,
    pub scale: f32,
}

pub enum AppEvent {
    MoveNoteDown(usize),
    MoveNoteUp(usize),
}

fn main() {
    Application::new(|cx| {
        AppData {
            notes: vec![NoteData::new(
                Pitch::new(2, Note::C, 0.0),
                MusicalTime::from_beats(0),
                MusicalTime::from_quarter_beats(0, 1),
            )],
            scale: 1.0,
        }
        .build(cx);

        cx.add_stylesheet(include_style!("src/theme.css"))
            .expect("Failed to load stylesheet");
        TopBar::new(cx);
        ZStack::new(cx, |cx| {
            Element::new(cx).background_color(Color::from("#323232"));
            PianoRoll::new(cx).space(Pixels(4.0));
            Element::new(cx)
                .border_width(Pixels(4.0))
                .border_color(Color::from("#323232"))
                .border_radius(Pixels(8.0))
                .hoverable(false);
        });
    })
    .run();
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            AppEvent::MoveNoteDown(idx) => {
                println!("move down {}", idx);
                self.notes[*idx].decrement();
            }
            AppEvent::MoveNoteUp(idx) => {
                println!("move up {}", idx);
                self.notes[*idx].increment();
            }
        })
    }
}
