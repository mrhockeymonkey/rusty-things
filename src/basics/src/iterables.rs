
pub fn run_examples() {
    //filter_things();
    //filter_map_things();
    opt();
}

fn filter_things() {
    let a = [0i32, 1, -1, 2, -2];

    let result: Vec<_> = a.iter()
        .filter(|x| x.is_positive())
        .collect();
    
    dbg!(result);
}

fn filter_map_things() {
    let a = [0i32, 1, -1, 2, -2];

    let result: Vec<_> = a.iter()
        .filter_map(|x| if x.is_positive() { Some(x) } else { None } )
        .collect();

    dbg!(result);
}

fn opt() {
    let a = [Some(1), Some(2), Some(3), Some(4)];
    
    let result: Vec<_> = a.iter()
        .filter_map(|&x| x)
        .collect();
    
    dbg!(result);
    
    let rresult: Option<Vec<_>> = a.into_iter()
        .collect();
    
    dbg!(rresult);
}