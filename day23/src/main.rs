use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read};

fn find_next(finder: &mut VecDeque<usize>, skip: &Vec<usize>) {
    finder.rotate_right(1);
    while skip.contains(finder.front().unwrap()) {
        finder.rotate_right(1);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let mut circle: VecDeque<usize> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect();
    let mut circle: VecDeque<usize> = vec![4,6,3,5,2,8,1,7,9].into_iter().collect();
    let moves = 100;
    circle.rotate_left(1);
    let mut finder: Vec<_> = circle.iter().cloned().collect();
    finder.sort();
    let mut finder: VecDeque<_> = finder.into_iter().collect();
    while finder.front().unwrap() != circle.back().unwrap() {
        finder.rotate_left(1);
    }
    let mut removed = Vec::new();
    for _ in 0..moves {
        println!("circ: {:?}", circle);
        for _ in 0..3 {
            removed.push(circle.pop_front().unwrap())
        }
        println!("removed: {:?}", removed);
        find_next(&mut finder, &removed);
        let new_current = *circle.front().unwrap();
        println!("finder: {:?}", finder);
        while circle.back().unwrap() != finder.front().unwrap() {
            circle.rotate_right(1);
        }
        for _ in 0..3 {
            circle.push_front(removed.pop().unwrap());
        }
        while *circle.back().unwrap() != new_current {
         circle.rotate_left(1);
        }
        while *finder.front().unwrap() != new_current {
         finder.rotate_left(1);
        }
    }

    while *circle.front().unwrap() != 1 {
        circle.rotate_left(1);
    }
    println!(
        "Sol: {}",
        circle
            .iter()
            .skip(1)
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("")
    );

    Ok(())
}
