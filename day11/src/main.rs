#![feature(array_zip)]
mod soln1;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BarChart, Block, Borders, Row, Table, Widget};
use tui::Terminal;
pub fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let rows = (0..3).map(|i| {
    //     let stra: &str = format!("{}", i).as_str();
    //     Row::new(vec![move stra, "1", "2", "3"])
    // });

    let table = Table::new(vec![
        Row::new(vec!["hehehehehehe", "safdasdfasfasdf", "fadsfafsafasfafadsafds"]),
        Row::new(vec!["hehehehehehe", "safdasdfasfasdf", "fadsfafsafasfafadsafds"]),
        Row::new(vec!["hehehehehehe", "safdasdfasfasdf", "fadsfafsafasfafadsafds"]),
        Row::new(vec!["hehehehehehe", "safdasdfasfasdf", "fadsfafsafasfafadsafds"]),
        Row::new(vec!["hehehehehehe", "safdasdfasfasdf", "fadsfafsafasfafadsafds"]),
    ])
    .block(Block::default().title("Table").borders(Borders::ALL));

    let data: Vec<(&str, u64)> = vec![
        ("B1", 9),
        ("B2", 12),
        ("B3", 5),
        ("B4", 8),
        ("B5", 2),
        ("B6", 4),
        ("B7", 5),
        ("B8", 9),
        ("B9", 14),
        ("B10", 15),
        ("B11", 1),
        ("B12", 0),
        ("B13", 4),
        ("B14", 6),
        ("B15", 4),
        ("B16", 6),
        ("B17", 4),
        ("B18", 7),
        ("B19", 13),
        ("B20", 8),
        ("B21", 11),
        ("B22", 9),
        ("B23", 3),
        ("B24", 5),
    ];
    let barchart = BarChart::default()
        .block(Block::default().title("Data3").borders(Borders::ALL))
        .data(&data)
        .bar_style(Style::default().fg(Color::Red))
        .bar_width(7)
        .bar_gap(0)
        .value_style(Style::default().bg(Color::Red))
        .label_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::ITALIC));
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(table, size);
        // f.render_widget(barchart, size);
        // f.render_widget(block, size);
    })?;
    Ok(())
    // println!("Hello Day 11!");
    // let contents: &str = include_str!("../inputs/sample.txt");
    // let part1 = soln1::Soln1::part1(contents, 100);
    // println!("Part 1 = {:?}", part1);
    // let part2 = soln1::Soln1::part2(contents);
    // println!("Part 2 = {:?}", part2);
}

#[cfg(test)]
mod tests {
    use crate::soln1;

    #[test]
    fn test_sample() {
        let contents: &str = include_str!("../inputs/sample.txt");
        let part1 = soln1::Soln1::part1(contents, 100);
        assert_eq!(part1, 1656);
        let part2 = soln1::Soln1::part2(contents);
        println!("{:?}", part2);
        assert_eq!(part2, Some(195));
    }
}
