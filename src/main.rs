use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

struct Position(u32, u32);

impl Position {
    fn get_coordinates(&self) -> (u32, u32) {
        (self.0, self.1)
    }
}

fn main() {
    let coords = Position(3, 2);
        // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Movement: A-W-S-D, Shoot: Arrow keys, Q to quit.{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    

    for c in stdin.keys() {
        // Clear the current line.
        display_map(&coords);

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c)   => match c {
                'a' => todo!(),
                'w' => todo!(),
                's' => todo!(),
                'd' => todo!(),
                _   => todo!()
            },
            Key::Left      => todo!(),
            Key::Right     => todo!(),
            Key::Up        => todo!(),
            Key::Down      => todo!(),
            _              => todo!(),
        }

        // Flush again.
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn display_map(player_pos: &Position) {

    for i in 0..=7 {
        for j in 0..=7 {
            if player_pos.get_coordinates() == (i, j) {
                print!("x")
            } else {
                print!("_");
            }
        }
        println!()
    }
}

