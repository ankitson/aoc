#[derive(Debug)]
pub enum FoldType {
    XFold,
    YFold,
}
#[derive(Debug)]
pub struct Fold {
    pub ftype: FoldType,
    pub at: usize,
}
pub fn parse(input: &str, grid_size: usize) -> (Vec<Vec<usize>>, Vec<Fold>) {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; grid_size]; grid_size];

    let mut folds: Vec<Fold> = Vec::new();
    let lines = input.lines();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("fold") {
            let mut splits = line.split_ascii_whitespace();
            let last = splits.nth_back(0).unwrap();
            let (axis, num) = last.split_once("=").unwrap();
            let num_n = num.parse::<usize>().unwrap();
            let fold = match axis {
                "y" => Fold {
                    ftype: FoldType::YFold,
                    at: num_n,
                },
                "x" => Fold {
                    ftype: FoldType::XFold,
                    at: num_n,
                },
                _ => panic!("Illegal fold axis"),
            };
            folds.push(fold);
        } else {
            let (xc, yc) = line.split_once(",").unwrap();
            let x = xc.parse::<usize>().unwrap();
            let y = yc.parse::<usize>().unwrap();
            grid[y][x] = 1;
        }
    }
    (grid, folds)
}
