use crate::{AppData, AppEvent, MusicalTime, Note, NoteData, Pitch};
use vizia::prelude::*;

pub mod piano_view;
pub use piano_view::*;

pub mod grid_view;
pub use grid_view::*;

pub mod note_view;
pub use note_view::*;

pub enum PianoRollEvent {
    SelectNote(usize),
    MoveDown(usize),
    MoveUp(usize),
    StartDrag(usize),
    EndDrag(usize),
}

#[derive(Lens)]
pub struct PianoRoll {}

impl PianoRoll {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {
                ScrollView::new(cx, 0.0, 0.5, false, false, |cx| {
                    // ZStack::new(cx, |cx|{
                    // Grid + Notes
                    GridView::new(cx, |cx| {
                        //Binding::new(cx, PianoRoll::notes, |cx, notes|{
                        for (idx, note) in AppData::notes.get(cx).iter().enumerate() {
                            let note = note.clone();
                            Binding::new(cx, GridView::root, move |cx, grid| {
                                let grid = grid.get(cx);
                                if time_to_pos(
                                    cx,
                                    note.get_start(),
                                    note.get_end(),
                                    grid.start,
                                    grid.end,
                                )
                                .is_some()
                                {
                                    NoteView::new(cx, idx).bind(
                                        AppData::notes.index(idx),
                                        move |handle, note| {
                                            println!("update");
                                            let note_data = note.get(&handle);
                                            let (posx, width) = time_to_pos(
                                                &handle,
                                                note_data.get_start(),
                                                note_data.get_end(),
                                                grid.start,
                                                grid.end,
                                            )
                                            .unwrap();
                                            let (posy, height) =
                                                pitch_to_pos(note_data.get_pitch());

                                            handle
                                                .left(Pixels(60.0 + posx))
                                                .top(Pixels(posy))
                                                .height(Pixels(height))
                                                .width(Pixels(width));
                                        },
                                    );
                                }
                            });
                        }
                    });

                    // Piano
                    PianoView::new(cx, AppData::scale);
                });
            })
            .row_between(Pixels(1.0))
    }
}

impl View for PianoRoll {
    fn element(&self) -> Option<&'static str> {
        Some("piano-roll")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|piano_roll_event, meta| match piano_roll_event {
            PianoRollEvent::MoveDown(index) => {
                cx.emit(AppEvent::MoveNoteDown(*index));
            }

            PianoRollEvent::MoveUp(index) => {
                cx.emit(AppEvent::MoveNoteUp(*index));
            }

            _ => {}
        });
    }
}

fn pitch_to_pos(pitch: Pitch) -> (f32, f32) {
    let note = pitch.note();

    let offset = note_to_key_height_offset(note);
    let height = note_to_key_height(note);

    println!("-- {} {}", offset, height);

    (offset, height)
}

fn time_to_pos<D>(
    cx: &D,
    start: MusicalTime,
    end: MusicalTime,
    grid_start: MusicalTime,
    grid_end: MusicalTime,
) -> Option<(f32, f32)>
where
    D: DataContext,
{
    if start > grid_end || end < grid_start {
        return None;
    }

    let px_per_beat = 100.0 * AppData::scale.get(cx) as f64;

    let offset_from_start = (start.as_beats_f64() - grid_start.as_beats_f64()) * px_per_beat;
    let length = (end.as_beats_f64() - start.as_beats_f64()) * px_per_beat;

    Some((offset_from_start as f32, length as f32))
}

pub fn note_to_key_height(note: Note) -> f32 {
    if note.is_black_key() {
        14.0
    } else {
        let mut height = 23.0;
        match note {
            Note::D | Note::F | Note::B => height += 1.0,
            _ => {}
        }
        height
    }
}

pub fn note_to_key_height_offset(note: Note) -> f32 {
    let mut offset = 0.0;

    for i in 0..note.into() {
        let mut new_note = note.clone();
        new_note.add_semitones(i);
        offset += note_to_key_height(new_note);
    }

    offset
}
