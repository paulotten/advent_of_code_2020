mod data;

fn main() {
    let data = data::get_data();
    //let data = data::_sample_data();

    let mut lines = data.lines();
    let timestamp: u32 = lines.next().unwrap().parse().unwrap();
    // I don't trust that 'x's actually mean nothing
    // let's not throw them away until we see part 2
    let ids: Vec<_> = lines.next().unwrap().split(',').collect();

    puzzle1(timestamp, ids);
}

fn puzzle1(timestamp: u32, ids: Vec<&str>) {
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
