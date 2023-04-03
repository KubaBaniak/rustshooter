use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin, Stdout};

#[derive(Clone)]
struct Position(u16, u16);

struct ElementsOnMap {
    bullets: Vec<Bullet>,
}

struct Player {
    position: Position,
}

struct Bullet {
    position: Position,
    speed: u16,
    direction: u16,
}

impl Position {
    //(row, col)
    fn get_coordinates(&self) -> (u16, u16) {
        (self.0, self.1)
    }
}

impl Player {
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

impl Bullet {
    fn new(direction: u16, mut position: Position) -> Bullet {
        match direction {
            1 => position.0 -= 1, // x + 1
            2 => position.1 += 1, // x + 1
            3 => position.0 += 1, // x - 1
            4 => position.1 -= 1, // y - 1
            _ => ()
        };
        Bullet {
            direction,
            position,
            speed: 1,     
        }
    }

    fn update_position(&mut self) {
        match self.direction {
            1 => self.position.0 -= 1, // x + 1
            2 => self.position.1 += 1, // x + 1
            3 => self.position.0 += 1, // x - 1
            4 => self.position.1 -= 1, // y - 1
            _ => ()
        };
    }
}

fn main() {
    let mut map = ElementsOnMap {bullets: Vec::new()};
    let mut player1 = Player { position:Position(3, 1) };
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Movement: A-W-S-D, Shoot: Arrow keys, Q to quit.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Up        => player1.move_player(1),
            Key::Right     => player1.move_player(2),
            Key::Down      => player1.move_player(3),
            Key::Left      => player1.move_player(4),
            _              => (),
        };

        map.bullets.push(Bullet::new(1, player1.position.clone()));

        for bullet in map.bullets.iter() {
            bullet.update_position()
        }

        display_map(&player1, &mut stdout, &map).unwrap();
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn display_map(player: &Player,
               terminal: &mut RawTerminal<Stdout>,
               map: &ElementsOnMap
               )
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
            let mut is_bullet = false;
            for bullet in &map.bullets {
                if bullet.position.get_coordinates() == (row, col) {
                    is_bullet = true;
                    break
                }
            }
            if player.position.get_coordinates() == (row, col) {
                write!(*terminal, "x ")?;
            } else if is_bullet {
                write!(*terminal, "* ")?;
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

fn shoot() {
}
