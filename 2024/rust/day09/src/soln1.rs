use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

pub type Input = (Vec<(usize, u32)>, Vec<(usize, u32)>);
pub type Output = usize;

pub fn parse(input: &str) -> Input {
    let files = input
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .filter(|(idx, char)| *idx % 2 == 0)
        .map(|(idx, char)| (idx, char.to_digit(10).unwrap()))
        .collect_vec();
    let spaces = input
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .filter(|(idx, char)| *idx % 2 == 1)
        .map(|(idx, char)| (idx, char.to_digit(10).unwrap()))
        .collect_vec();
    (files, spaces)
}

pub fn part1(raw_input: &str) -> Output {
    let (files, spaces) = parse(raw_input);
    let mut disk = vec![];
    let mut file_no = 0;
    let mut spc_idxs = vec![];
    let mut files_actual = vec![];
    for (i, (a, b)) in files.iter().interleave(spaces.iter()).enumerate() {
        if i % 2 == 0 {
            disk.extend(vec![file_no; (*b).try_into().unwrap()]);
            files_actual.push((disk.len() - 1, file_no, (*b)));
            file_no += 1;
        } else {
            spc_idxs.push((disk.len(), *b));
            disk.extend(vec![10000; (*b).try_into().unwrap()]);
        }
    }
    let mut spc_ptr = spc_idxs[0].0;
    let mut file_ptr = files_actual[files_actual.len() - 1].0;
    let dl = disk.len();
    println!("disk.len = {dl} spc = {spc_ptr} file = {file_ptr}");
    while disk[spc_ptr] == 10000 && disk[file_ptr] != 10000 {
        disk[spc_ptr] = disk[file_ptr];
        disk[file_ptr] = 10000;
        while spc_ptr < disk.len() && disk[spc_ptr] != 10000 {
            spc_ptr += 1;
        }
        while file_ptr > 0 && disk[file_ptr] == 10000 {
            file_ptr -= 1;
        }
        if spc_ptr == disk.len() || file_ptr == 0 {
            break;
        }
        if spc_ptr > file_ptr {
            break;
        }
        // println!("disk = {disk:?} spc = {spc_ptr} file = {file_ptr}");
    }
    let mut total = 0;
    for (idx, num) in disk.iter().enumerate() {
        if *num != 10000 {
            total += idx * num
        }
    }
    // println!("disk = {:?}", disk);
    total
}

pub fn part2(raw_input: &str) -> Output {
    let (files, spaces) = parse(raw_input);
    let mut disk = vec![];
    let mut file_no = 0;
    let mut spc_idxs = vec![];
    let mut files_actual = vec![];
    for (i, (a, b)) in files.iter().interleave(spaces.iter()).enumerate() {
        if i % 2 == 0 {
            disk.extend(vec![file_no; (*b).try_into().unwrap()]);
            files_actual.push((disk.len() - 1, file_no, (*b)));
            file_no += 1;
        } else {
            spc_idxs.push((disk.len(), *b));
            disk.extend(vec![50000; (*b).try_into().unwrap()]);
        }
    }
    let mut spc_ptr = spc_idxs[0].0;
    let mut file_ptr = files_actual[files_actual.len() - 1].0;
    let dl = disk.len();
    let mut file_idx = files_actual.len() - 1;
    // println!("files = {files_actual:?}");
    let zf = files_actual.iter().filter(|v| v.2 == 0).count();
    println!("zero files = {zf:?}");
    println!("disk.len = {dl} spc = {spc_ptr} file = {file_ptr}");
    // println!("disk = {disk:?}");
    'outer: while file_idx > 0 {
        // if file_idx == 0 {
        // was_zero_begin = true;
        // }
        'inner: for (spc_start, spc_len) in &spc_idxs {
            // println!("try spc {spc_start} len={spc_len} for file_idx={file_idx}");
            if *spc_len >= files_actual[file_idx].2 && *spc_start < files_actual[file_idx].0 {
                // println!("match");
                file_ptr = files_actual[file_idx].0;
                let file_id = files_actual[file_idx].1;
                spc_ptr = *spc_start;
                while file_ptr > 0 && disk[file_ptr] == file_id {
                    disk[spc_ptr] = disk[file_ptr];
                    disk[file_ptr] = 50000;
                    file_ptr -= 1;
                    spc_ptr += 1;
                }
                // println!("after move disk = {disk:?}");
                if file_idx == 0 {
                    break 'outer;
                }
                break 'inner;
            }
        }
        if file_idx > 0 {
            file_idx -= 1;
        } else {
            break 'outer;
        }

        spc_idxs = vec![];
        let mut cspc_start = -1isize;
        let mut cspc_size = 0;
        for idx in 0..disk.len() {
            if disk[idx] == 50000 && cspc_start == -1 {
                cspc_start = idx as isize;
                cspc_size = 1;
            } else if disk[idx] == 50000 && cspc_start != -1 {
                cspc_size += 1;
            }
            if disk[idx] != 50000 && cspc_start != -1 {
                spc_idxs.push((cspc_start as usize, cspc_size));
                cspc_start = -1isize;
                cspc_size = 0;
            }
        }
    }

    let mut total = 0;
    for (idx, num) in disk.iter().enumerate() {
        if *num != 50000 {
            total += idx * num
        }
    }

    // println!("disk = {:?}", disk);
    total
    //6321896733106
}

// pub fn part1_broken(raw_input: &str) -> Output {
//     let (files, spaces) = parse(raw_input);
//     println!("{:?} {:?}", files, spaces);
//     let mut cur_file_idx = files.len() - 1;
//     let mut cur_left = files[files.len() - 1].0;
//     let mut disk = vec![];
//     let mut file_no = 0;
//     let mut spc_idxs = vec![];
//     let mut files_actual = vec![];
//     for (i, (a, b)) in files.iter().interleave(spaces.iter()).enumerate() {
//         if i % 2 == 0 {
//             disk.extend(vec![file_no; (*b).try_into().unwrap()]);
//             files_actual.push((file_no, (*b)));
//             file_no += 1;
//         } else {
//             spc_idxs.push((disk.len(), *b));
//             disk.extend(vec![10000; (*b).try_into().unwrap()]);
//         }
//         // println!("disk = {:?}", disk);
//     }

//     println!("disk = {:?}", disk);

//     let mut idx = files_actual.len() - 1;
//     let mut spc_idx_idx = 0;
//     let (mut spc_idx, mut spc_size) = spc_idxs[0];
//     let mut cur_file_id = files_actual[idx].0;
//     let mut cur_file_rem = files_actual[idx].1 as u32;
//     'outer: while spc_idx < disk.len() {
//         let gap_fill_size = spc_size.min(cur_file_rem) as usize;
//         println!("spc size = {spc_size} cur_file_rem={cur_file_rem} gap_fil={gap_fill_size}");
//         println!("filling with {cur_file_id} disk = {:?}", disk);
//         disk[spc_idx..spc_idx + gap_fill_size].fill(cur_file_id as usize);
//         if spc_idx {
//             break;
//         }
//         println!("filled with {cur_file_id} disk = {:?}", disk);
//         spc_idx += gap_fill_size;
//         spc_size -= gap_fill_size as u32;
//         if spc_size == 0 {
//             spc_idx_idx += 1;
//             if spc_idx_idx >= spc_idxs.len() {
//                 println!("all spaces filled");
//                 let dl = disk.len();
//                 disk[spc_idx + gap_fill_size..dl].fill(10000);
//                 break 'outer;
//             }
//             (spc_idx, spc_size) = spc_idxs[spc_idx_idx]
//         }
//         cur_file_rem -= gap_fill_size as u32;
//         if cur_file_rem <= 0 {
//             if idx == 0 {
//                 println!("all files added");
//                 break 'outer;
//             }
//             idx -= 1;
//             (cur_file_id, cur_file_rem) = (files_actual[idx].0, files_actual[idx].1 as u32);
//         }
//     }
//     println!("disk = {:?}", disk);
//     0
// }
