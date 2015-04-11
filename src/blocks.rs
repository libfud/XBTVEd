use super::schedule::Schedule;
use super::program::{Program, Source, Instruction};
use super::program::Instruction::{Play, SubProgram};
use super::tags::Tags;

#[derive(Clone)]
pub enum Placement {
    Beginning,
    Middle(usize),
    End
}

#[derive(Clone)]
pub enum FillType {
    Once(Placement),
    Twice(Placement, Placement),
    Thrice(Placement, Placement, Placement),
    None
}

impl FillType {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            &FillType::Once(_) => Ok(()),
            &FillType::Twice(ref x, ref y) => match (x, y) {
                (&Placement::Beginning, &Placement::Beginning) |
                (&Placement::Beginning, &Placement::Middle(_)) |
                (&Placement::Beginning, &Placement::End) |
                (&Placement::Middle(_), &Placement::End) |
                (&Placement::End, &Placement::End) => Ok(()),

                (&Placement::Middle(x), &Placement::Middle(y)) => {
                    if x <= y { 
                        Ok(())
                    } else {
                        Err("Mismatched time placements".to_string())
                    }
                },
                
                _ => Err("Bad placement composition".to_string())
            },
            &FillType::Thrice(ref x, ref y, ref z) => match (x, y, z) {
                (&Placement::Beginning, &Placement::Beginning, &Placement::Beginning) |
                (&Placement::Beginning, &Placement::Beginning, &Placement::Middle(_)) |
                (&Placement::Beginning, &Placement::Middle(_), &Placement::End) |
                (&Placement::Beginning, &Placement::End, &Placement::End) |
                (&Placement::Beginning, &Placement::Beginning, &Placement::End) |
                (&Placement::Middle(_), &Placement::End, &Placement::End) |
                (&Placement::End, &Placement::End, &Placement::End) => Ok(()),

                (&Placement::Middle(x), &Placement::Middle(y), &Placement::End) |
                (&Placement::Beginning, &Placement::Middle(x), &Placement::Middle(y)) => {
                    if x <= y {
                        Ok(())
                    } else {
                        Err("Mismatched time placements".to_string())
                    }
                },

                (&Placement::Middle(x), &Placement::Middle(y), &Placement::Middle(z)) => {
                    if x <= y && y <= z {
                        Ok(())
                    } else {
                        Err("Mismatched time placements".to_string())
                    }
                },

                _ => Err("Bad placement composition".to_string()),
            },
            &FillType::None => Ok(())
        }
    }
}

#[derive(Clone)]
pub struct FillerIterator {
    filltype: FillType,
    fillmedia: Vec<String>,
    fill_pos: usize
}

impl FillerIterator {
    pub fn new(filltype: FillType, filler: Vec<String>) -> Result<FillerIterator, String> {

        try!(filltype.validate());
        
        Ok(FillerIterator {
            filltype: filltype,
            fillmedia: filler,
            fill_pos: 0
        })
    }

    pub fn make_subprog(&mut self) -> Instruction {
        if self.fill_pos + 1 == self.fillmedia.len() {
            self.fill_pos = 0;
        }
        let subprog = SubProgram(Program::new(Source::Pathname(self.fillmedia.get(self.fill_pos).unwrap().clone()), 
                                              Tags::new(), vec!(Play(0, 0))));
        self.fill_pos += 1;
        subprog
    }

    pub fn once(&mut self, place: &Placement) -> Vec<Instruction> {
        let subprog = self.make_subprog();
        match *place {
            Placement::Beginning => vec!(subprog, Play(0, 0)),
            Placement::Middle(time) => vec!(Play(0, time), subprog, Play(time, 0)),
            Placement::End => vec!(Play(0, 0), subprog)
        }
    }

    pub fn twice(&mut self, place1: &Placement, place2: &Placement) -> Vec<Instruction> {
        let subprog1 = self.make_subprog();
        let subprog2 = self.make_subprog();

        match (place1.clone(), place2.clone()) {
            (Placement::Beginning, Placement::Beginning) => {
                vec!(subprog1, subprog2, Play(0, 0))
            },

            (Placement::Beginning, Placement::Middle(time)) => {
                 vec!(subprog1, Play(0, time), subprog2, Play(time, 0))
            },
            
            (Placement::Beginning, Placement::End) => vec!(subprog1, Play(0, 0), subprog2),
            
            (Placement::Middle(time), Placement::End) => vec!(Play(0, time), subprog1, Play(time, 0), subprog2),

            (Placement::Middle(time1), Placement::Middle(time2)) => {
                vec!(Play(0, time1), subprog1, Play(time1, time2 - time1), subprog2, Play(time2, 0))
            },

            (Placement::End, Placement::End) => vec!(Play(0, 0), subprog1, subprog2),

            (_, _) => panic!("Invalid positioning.")
        }
    }

    pub fn thrice(&mut self, place1: &Placement, place2: &Placement, place3: &Placement) -> Vec<Instruction> {
        let subprog1 = self.make_subprog();
        let subprog2 = self.make_subprog(); 
        let subprog3 = self.make_subprog(); 
                
        match (place1.clone(), place2.clone(), place3.clone()) {
            (Placement::Beginning, Placement::Beginning, Placement::Beginning) => {
                vec!(subprog1, subprog2, subprog3, Play(0, 0))
            },
            
            (Placement::Beginning, Placement::Beginning, Placement::Middle(time)) => {
                vec!(subprog1, subprog2, Play(0, time), subprog3, Play(time, 0))
            },
           
            (Placement::Beginning, Placement::Beginning, Placement::End) => {
                vec!(subprog1, subprog2, Play(0, 0), subprog3)
            },

            (Placement::Beginning, Placement::End, Placement::End) => {
                vec!(subprog1, Play(0, 0), subprog2, subprog3)
            },

            (Placement::Middle(time), Placement::End, Placement::End) => {
                vec!(Play(0, time), subprog1, Play(time, 0), subprog2, subprog3)
            },

            (Placement::Beginning, Placement::Middle(time), Placement::End) => {
                vec!(subprog1, Play(0, time), subprog2, Play(time, 0), subprog3)
            }

            (Placement::Beginning, Placement::Middle(time1), Placement::Middle(time2)) => {
                vec!(subprog1, Play(0, time1), subprog2, Play(time1, time2 - time1), subprog3,
                     Play(time2, 0))
            },

            (Placement::Middle(time1), Placement::Middle(time2), Placement::Middle(time3)) => {
                vec!(Play(0, time1), subprog1, Play(time1, time2 - time1),
                             subprog2, Play(time2, time3 - time2), subprog3, Play(time3, 0))
            },

            (Placement::Middle(time1), Placement::Middle(time2), Placement::End) => {
                vec!(Play(0, time1), subprog1, Play(time1, time2 - time1), subprog2,
                     Play(time2, 0), subprog3)
            },

            (Placement::End, Placement::End, Placement::End) => {
                vec!(Play(0, 0), subprog1, subprog2, subprog3)
            },

            (_, _, _) => panic!("Invalid positioning.")
        }
    }
        
}

impl Iterator for FillerIterator {
    type Item = Vec<Instruction>;

    fn next(&mut self) -> Option<Vec<Instruction>> {
        let instrs = match self.filltype.clone() {
            FillType::Once(ref x) => self.once(x),
            FillType::Twice(ref x, ref y) => self.twice(x, y),
            FillType::Thrice(ref x, ref y, ref z) => self.thrice(x, y, z),
            FillType::None => vec!(Play(0, 0))
        };
        Some(instrs)
    }
}

#[derive(Clone)]
pub struct BlockIterator {
    name: String,
    entries: Vec<Vec<String>>,
    filler: FillerIterator,
    repeat: Vec<usize>,
    longest: usize,
    sched_num: usize,
    tag_entries: Option<Vec<Vec<Tags>>>
}

impl BlockIterator {
    pub fn new(nom: &str, entries: &Vec<Vec<String>>, filler: &FillerIterator, 
               repeat: &Vec<usize>, tags: Option<Vec<Vec<Tags>>>) -> Result<BlockIterator, String> {
        try!(filler.filltype.validate());

        if entries.len() != repeat.len() {
            return Err("There must be the same number of elements in repeat vector as entries.".to_string())
        }

        match tags {
            None => { },
            Some(ref x) => {
                if x.len() != entries.len() {
                    return Err("There must be the same number of vectors of tags as entries.".to_string())
                }

                if entries.iter().zip(x.iter()).any(|(entry, tag_entry)| entry.len() != tag_entry.len()) {
                    return Err("The number of tag structures must match the number of elements in an entry vector."
                        .to_string())
                }
            }
        }                        

        Ok(BlockIterator {
            name: nom.to_string(),
            entries: entries.clone(),
            filler: filler.clone(),
            repeat: repeat.clone(),
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

            sched_num: 0,
            tag_entries: tags
        })
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
            
            let queue: Vec<String> = self.entries.iter().zip(self.repeat.iter())
                .fold(Vec::new(), |mut sched, (entry, &repeater)| {
                    let mut progs = entry.iter().skip(self.sched_num * repeater)
                        .take(repeater).map(|location| location.clone()).collect::<Vec<String>>();

                    sched.append(&mut progs);
                    sched
                });

            let tags: Vec<Tags> = match self.tag_entries {
                Some(ref tag_vecs) => tag_vecs.iter().zip(self.repeat.iter())
                .fold(Vec::new(), |mut tag_vec, (tag_entry, &repeater)| {
                    let mut tags = tag_entry.iter().skip(self.sched_num * repeater)
                        .take(repeater).map(|tag| tag.clone()).collect::<Vec<Tags>>();
                    tag_vec.append(&mut tags);
                    tag_vec
                }),

                None => (0 .. queue.len()).map(|_| Tags::new()).collect::<Vec<Tags>>()
            };

            let progs = queue.iter().zip(tags.into_iter()).map(|(loc, entry_tags)| {
                let instrs = self.filler.next().unwrap();
                let location = Source::Pathname(loc.clone());
                Program::new(location, entry_tags, instrs)
            }).collect::<Vec<Program>>();
            Some(Schedule::new(&sched_name, progs))
        }
    }
}

