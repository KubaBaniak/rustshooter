use termion::cursor;
use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin, Stdout};

struct Position(u16, u16);

struct Player {
    position: Position,
}

struct Bullet {
    speed: u16,
    direction: u16,
}

impl Player {
    //(row, col)
    fn get_coordinates(&self) -> (u16, u16) {
        (self.position.0, self.position.1)
    }
    // 1 - up, 2 - right, 3 - down, 4 - left
    fn move_player(&mut self, direction: u16) {
        match direction {
            1 => self.position.0 -= 1, // x + 1
            2 => self.position.1 += 1, // x + 1
            3 => self.position.0 += 1, // x - 1
            4 => self.position.1 -= 1, // y - 1
            _ => ()
        }
    }
}

fn main() {
    let mut player1 = Player { position:Position(3, 1) };
        // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Movement: A-W-S-D, Shoot: Arrow keys, Q to quit.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    

    for c in stdin.keys() {
        // Clear the current line.

        match c.unwrap() {
            Key::Char('q') => break,
            //Key::Char(c)   => match c {
            //    'a' => todo!(),
            //    'w' => todo!(),
            //    's' => todo!(),
            //    'd' => todo!(),
            //    _   => todo!()
            //},
            Key::Up        => player1.move_player(1),
            Key::Right     => player1.move_player(2),
            Key::Down      => player1.move_player(3),
            Key::Left      => player1.move_player(4),
            _              => (),
        }

        display_map(&player1, &mut stdout).unwrap();

        // Flush again.
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn display_map(player_pos: &Player, terminal: &mut RawTerminal<Stdout> )
    -> Result<(), std::io::Error>
{
    let size: u16 = 8;
    write!(*terminal,
           "{}{}  a b c d e f g h\n",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           )?;
    for row in 1..=size { 
        write!(*terminal,
               "{}{} ",
               termion::cursor::Goto(1, row+1),
               row,
               )?;
        for col in 1..=size {
            if player_pos.get_coordinates() == (row, col) {
                write!(*terminal, "x ")?;
            } else {
                write!(*terminal, "□ ")?;
            }
        }
        write!(*terminal, "{}\n", row)?;
    }
    write!(*terminal,
           "{}  a b c d e f g h\n",
           termion::cursor::Goto(1, size+2),
           )?;
    Ok(())
}

