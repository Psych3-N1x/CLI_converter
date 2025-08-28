mod jpg2ico;
mod jpg2png;
mod png2jpg;
mod png2ico;

use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    style::{Color, SetForegroundColor, ResetColor, Print},
};
use std::io::{self, Write};
use std::error::Error;

use crate::jpg2ico::convert_jpg_to_ico;
use crate::jpg2png::convert_jpg_to_png;
use crate::png2jpg::convert_png_to_jpg;
use crate::png2ico::convert_png_to_ico;

#[derive(Debug)]
struct MenuItem {
    name: &'static str,
    id: usize,
}

impl MenuItem {
    fn new(name: &'static str, id: usize) -> Self {
        Self { name, id }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let menu_items = vec![
        MenuItem::new("jpg2png", 0),
        MenuItem::new("png2jpg", 1),
        MenuItem::new("jpg2ico", 2),
        MenuItem::new("png2ico", 3),
        MenuItem::new("Exit", 4),
    ];
    let mut selected = 0;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, Hide)?;

    // Nettoyage à la sortie
    let result = run_menu(&mut stdout, &menu_items, &mut selected);

    // Toujours restaurer le terminal
    disable_raw_mode()?;
    execute!(stdout, Show)?;
    result
}

fn run_menu(
    stdout: &mut io::Stdout,
    menu_items: &[MenuItem],
    selected: &mut usize,
) -> Result<(), Box<dyn Error>> {
    // Affiche le titre une seule fois
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print("Converter Management Tool\n"),
        Print("--------------------------\n")
    )?;
    stdout.flush()?;

    // Affiche le menu initial
    for (i, item) in menu_items.iter().enumerate() {
        queue!(
            stdout,
            MoveTo(0, i as u16 + 3),
            Print(if i == *selected {
                format!("> {}", item.name)
            } else {
                format!("  {}", item.name)
            })
        )?;
    }
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        if *selected > 0 {
                            let new_selection = *selected - 1;
                            update_selection(stdout, menu_items, *selected, new_selection)?;
                            *selected = new_selection;
                        }
                    }
                    KeyCode::Down => {
                        if *selected < menu_items.len() - 1 {
                            let new_selection = *selected + 1;
                            update_selection(stdout, menu_items, *selected, new_selection)?;
                            *selected = new_selection;
                        }
                    }
                    KeyCode::Enter => {
                        match *selected {
                            0 => {
                                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                                stdout.flush()?;
                                disable_raw_mode()?;
                                convert_jpg_to_png()?;
                                enable_raw_mode()?;
                                
                            },
                            1 => {
                                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                                stdout.flush()?;
                                disable_raw_mode()?;
                                convert_png_to_jpg()?;
                                enable_raw_mode()?;
                            },
                            2 => {
                                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                                stdout.flush()?;
                                disable_raw_mode()?;
                                convert_jpg_to_ico()?;
                                enable_raw_mode()?;  // <-- pour pouvoir écrire
                            },
                            3 => {
                                execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
                                stdout.flush()?;
                                disable_raw_mode()?;
                                convert_png_to_ico()?;
                                enable_raw_mode()?;
                            },
                            4 => break, // Exit
                           _ => {}
                        }
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn update_selection(
    stdout: &mut io::Stdout,
    menu_items: &[MenuItem],
    old_selection: usize,
    new_selection: usize,
) -> io::Result<()> {
    // Efface l'ancienne sélection
    queue!(
        stdout,
        MoveTo(0, old_selection as u16 + 3),
        Print(format!("  {}", menu_items[old_selection].name))
    )?;
    // Affiche la nouvelle sélection
    queue!(
        stdout,
        SetForegroundColor(Color::Green),
        MoveTo(0, new_selection as u16 + 3),
        Print(format!("> {}", menu_items[new_selection].name)),
        ResetColor
    )?;
    stdout.flush()
}