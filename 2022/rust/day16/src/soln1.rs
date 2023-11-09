use itertools::Itertools;
use term_grid as tg;
use crate::shared::{parse, Input, Output};

pub struct Soln1 {}
impl Soln1 {
    pub fn part1(raw_input: &str) -> Output {
        // let input = parse(raw_input);
        // println!("{:?}", input);
        Self::test_term_grid();
        "ssss".to_string()
        //Self::part1_core(&input)
    }


    pub fn test_term_grid() {
        let ncols = 119;
        let files = vec![
            "Arduino", "auth.zip",
"crashes-2021",
"devhome",
"dotfiles",
"Downloads",
"Dropbox",
"fontawesome-free-6.3.0-web.zip",
"go",
"homeserver",
"Jts",
"mnt",
"monero-storage",
"node",
"notes",
"Postman",
"resume.html",
"sdk",
"snap",
"Sync",
"todo.txt",
"Unity",
"xdg-Desktop",
"xdg-Documents",
"xdg-Music",
"xdg-Pictures",
"xdg-Public",
"xdg-Templates",
"xdg-Videos",
"xyz.txt",
        ];
        //let grid = tg::Grid::new(tg::GridOptions::with_columns(ncols), files);
        let mut grid = tg::Grid::new(tg::GridOptions {
            direction:  tg::Direction::TopToBottom,
            filling:    tg::Filling::Spaces(2),
        });
        grid.reserve(files.len());

        for file in &files {
            grid.add(tg::Cell {
                contents: file.to_string(),
                width:    file.len(),
                alignment: tg::Alignment::Left,
            });
        }
        if let Some(display) = grid.fit_into_width(145) {
            println!("{}", display);
        } else {
            println!("Couldn't fit grid into {} columns", ncols);
        }


        //println!("files\n = {:?}", files);
        "return".to_string();
    }


    pub fn part1_core(input: &Input) -> Output {
        todo!()
    }

    pub fn part2(raw_input: &str) -> Output {
        let input = parse(raw_input);
        Self::part2_core(&input)
    }

    pub fn part2_core(input: &Input) -> Output {
        todo!()
    }
}
