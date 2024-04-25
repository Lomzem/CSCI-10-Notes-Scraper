use crate::canvas::Note;

fn prompt_confirmation(note: &Note) {
    println!("Do you want this item?");
    println!("\"{}\"", &note.title.trim());
}

pub fn select_note(notes: &Vec<Note>) {
    notes.iter().rev().for_each(|n| prompt_confirmation(n))
    // notes.iter().rev().for_each(|n| println!("{:?}", n));
}
