use std::path::Path;
use super::schedule::{Schedule, Program};

#[derive(Clone)]
pub enum Placement {
    Beginning,
    Middle(usize),
    End
}

#[derive(Clone)]
pub enum Filler {
    Once(Placement),
    Twice(Placement, Placement),
    Thrice(Placement, Placement, Placement),
    Periodic(usize),
    Bookend(usize),
    None
}

#[derive(Clone)]
pub struct BlockIterator {
    name: String,
    entries: Vec<Vec<String>>,
    fillstyle: Filler,
    filler: Vec<String>,
    repeat: Vec<usize>,
    autotag: bool,
    longest: usize,
    sched_num: usize
}

impl BlockIterator {
    pub fn new_with_filler(nom: &str, entries: &Vec<Vec<String>>, fillstyle: &Filler,
                           filler: &Vec<String>, repeat: &Vec<usize>, tag: bool) -> BlockIterator {
        BlockIterator {
            name: nom.to_string(),
            entries: entries.clone(),
            fillstyle: fillstyle.clone(),
            filler: filler.clone(),
            repeat: repeat.clone(),
            autotag: tag,
            longest: entries.iter().zip(repeat.iter()).fold(0, |len, (entry, &repetitions)| {
                let total_reps = if entry.len() % repetitions == 0 {
                    repetitions
                } else {
                    repetitions + 1
                };

                if entry.len() / total_reps  > len {
                    entry.len()
                } else {
                    len
                }}),

            sched_num: 0
        }
    }
}

impl Iterator for BlockIterator {
    type Item = Schedule;

    fn next(&mut self) -> Option<Schedule> {
        if self.sched_num == self.longest {
            None
        } else {
            let sched_name = format!("{}-{}{}", self.name, {
                if self.longest < 10 {
                    ""
                } else if self.longest < 100 {
                    if self.sched_num + 1 < 10 {
                    "0"
                    } else {
                        ""
                    }
                } else {
                    if self.sched_num + 1 < 10 {
                        "00"
                    } else if self.sched_num + 1 < 100 {
                        "0"
                    } else {
                        ""
                    }
                }}, self.sched_num + 1);
            
            let queue: Vec<String> = self.entries.iter().zip(self.repeat.iter()).map(|(&entry, &repeater)| {
                    entry.iter().skip(self.sched_num * repeater).take(repeater).map(|w| w.clone())
            }).collect();
            None
        }
    }
}
