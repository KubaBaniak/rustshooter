use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin, Stdout};

#[derive(Clone, Debug)]
struct Position(u16, u16);

#[derive(Debug)]
struct ElementsOnMap {
    bullets: Vec<Bullet>,
    player: Player,
}

#[derive(Debug)]
struct Player {
    position: Position,
}

#[derive(Debug)]
struct Bullet {
    position: Position,
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
        match (direction, &mut self.position) {
            (1, Position(row, _)) if *row != 1 => *row -= 1, // x + 1
            (2, Position(_, col)) if *col != 8 => *col += 1, // x + 1
            (3, Position(row, _)) if *row != 8 => *row += 1, // x + 1
            (4, Position(_, col)) if *col != 1 => *col -= 1, // x + 1
            _ => ()
        }
    }
}

impl Bullet {
    fn new(direction: u16, mut position: Position) -> Bullet {
        match direction {
            1 => position.0 -= 1, // x + 1
            //2 => position.1 += 1, // x + 1 SAME AS BELOW
            //3 => position.0 += 1, // x - 1
            //4 => position.1 -= 1, // y - 1
            _ => ()
        };
        Bullet { direction, position }
    }

    fn update_position(&mut self) {
        match self.direction {
            1 => self.position.0 -= 1, // x + 1
            //2 => self.position.1 += 1, // x + 1 TODO: Shoot to the left, right, back
            //3 => self.position.0 += 1, // x - 1
            //4 => self.position.1 -= 1, // y - 1
            _ => ()
        };
    }
}

impl ElementsOnMap {
    // Update bullets on map and delete these which went past it.
    fn update_map(&mut self) {
        for bullet in &mut self.bullets {
            bullet.update_position()
        }
        self.bullets.retain(|x| x.position.0 >= 1);
    }

    fn display_map(&self, terminal: &mut RawTerminal<Stdout>)
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
                    for bullet in &self.bullets {
                        if bullet.position.get_coordinates() == (row, col) {
                            is_bullet = true;
                            break
                        }
                    }
                    if self.player.position.get_coordinates() == (row, col) {
                        write!(*terminal, "x ")?;
                    } else if is_bullet {
                        write!(*terminal, "↥ ")?;
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

}

fn main() {
    let player = Player { position:Position(3, 1) };
    let mut map = ElementsOnMap {
        bullets: Vec::new(),
        player,
    };
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}Movement: A-W-S-D, Shoot: Arrow keys, Q to quit.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => panic!("CLICKED q"),
            Key::Char(' ') => map.bullets.push(Bullet::new(1, map.player.position.clone())),
            Key::Up        => map.player.move_player(1),
            Key::Right     => map.player.move_player(2),
            Key::Down      => map.player.move_player(3),
            Key::Left      => map.player.move_player(4),
            _              => (),
        };

        map.display_map(&mut stdout).expect("ERROR WHILE DISPLAYING");
        map.update_map();
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

