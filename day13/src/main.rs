mod data;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let mut lines = data.lines();
    let _timestamp: u32 = lines.next().unwrap().parse().unwrap();
    // I don't trust that 'x's actually mean nothing
    // let's not throw them away until we see part 2
    let ids: Vec<_> = lines.next().unwrap().split(',').collect();

    puzzle2(ids);
}

fn puzzle2(ids: Vec<&str>) {
    // called it
    let ids: Vec<Option<u32>> = ids.iter().map(|x| {
        let maybe_32: Result<u32, _> = x.parse();

        maybe_32.ok()
    }).collect();

    let mut timestamp: i64 = 0;
    let mut step: i64 = 1;

    for i in 0..ids.len() {
        if ids[i].is_none() {
            continue;
        }

        let id: i64 = ids[i].unwrap() as i64;

        let mut target = (id - i as i64) % id;

        // modulus can return negative numbers
        if target < 0 {
            target += id;
        }

        while timestamp % id as i64 != target {
            timestamp += step;
        }
        
        step *= id as i64;
    }
    println!("{}", timestamp);
}

fn _puzzle1(timestamp: u32, ids: Vec<&str>) {
    let mut min = u32::MAX;
    let mut min_id = 0;

    for id in ids {
        let maybe_u32: Result<u32, _> = id.parse();

        if maybe_u32.is_ok() {
            let id = maybe_u32.unwrap();

            let wait = id - timestamp % id;

            if wait < min {
                min = wait;
                min_id = id;
            }
        }
    }

    println!("id: {}, wait: {} ({})", min_id, min, min_id*min);
}
