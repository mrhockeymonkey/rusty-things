#![deny(elided_lifetimes_in_paths)]

use std::collections::VecDeque;
use std::marker::PhantomData;



fn main() {

    let some_values = vec![1,2,3];
    let start = Thing{ id: 0, values: &some_values };
    //let p = Processor{};
    //p.process(start);
    Processor::process(start);
}

#[derive(Clone, Copy)]

struct Thing<'a> {
    id: usize,
    values: &'a Vec<usize>
}


impl<'a> Thing<'a> {
    fn next(&self) -> Vec<Thing<'a>> {
        vec![
            Thing{ id: self.id + 1, values: self.values},
            Thing{ id: self.id + 2, values: self.values},
            Thing{ id: self.id + 3, values: self.values},
        ]
    }
}


struct Processor{

}

impl Processor {
    fn process<'a>(start: Thing<'a>) {
        let mut q: VecDeque<Thing<'a>> = VecDeque::new();

        q.push_back(start);

        while let Some(this) = q.pop_front() {

            for n in this.next() {
                if n.id < 10 {
                    q.push_back(n);
                    //println!("{}", n.id);

                    //q.push_back(Thing{id: n.id, values: n.values});
                }
            }
        }

        // thing is dropped here but why does that matter?
    }
}
