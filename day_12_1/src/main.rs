use std::{clone, fs::read_to_string, iter::Map, ops::Index, time::Instant};

#[derive(Debug, Clone)]
struct SpringGroup {
    id: usize,
    size: usize,
    start_index: usize,
}
#[derive(Debug, Clone)]
struct Maps {
    springs: Vec<char>,
    groups: Vec<usize>,
}

#[derive(Debug, Clone)]
struct ConditionMap {
    maps: Maps,
    base_arrangement: Vec<char>,
    spring_groups: Vec<SpringGroup>,
}

fn main() {
    let now = Instant::now();
    let path = "./data/day12TT";
    let full_data = get_list_from_file(path);
    let mut value_accumulator: usize = 0;
    let mut counter = 1;
    for line in full_data {
        //let total_arrangements_in_maps =
        println!("line number {}", counter);
        arrangement_coordinator(line);
        counter += 1;
        //value_accumulator += total_arrangements_in_maps;
    }
    println!(
        "Theres a total of {} possible arrangements in the data provided",
        value_accumulator
    );
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn arrangement_coordinator(line: String) {
    // -> usize
    // parse string line into Vec<char> and Vec<usize>
    // and make maps struct
    let (springs, groups) = parse_line(line);
    let maps = Maps { springs, groups };
    // Get ConditionMap with build_cm()
    let map = build_cm(maps);
    let arrangement_amount = map_analyzer(&map);
    // DBUG for c in map.base_arrangement{
    // DBUG     print!("{}", c);
    // DBUG }
    // DBUG print!(" ");
    // DBUG for n in map.maps.groups{
    // DBUG     print!("{},",n);
    // DBUG }
    // DBUG println!("");

    // Retrieve Condition map and analyze said map to
    // determine how many arrangements of the groups of springs are
    // possible
}
fn map_analyzer(map: &ConditionMap) -> usize {
    // -> usize
    // Determines how many possible valid arrangements of groups in maps
    // exists.
    // I operate with something I call degrees of freedom, shorthand
    // "freedoms". A freedom is how many slots a group can fit into.
    // every group has at least one freedom.
    // a group can be single, linked or both
    // a single group is completely independent from the other groups
    // a linked group shares freedoms with one or more other groups, so
    // the number of freedoms at any moment is determined by the position of
    // another group.
    // groups can be  both single and linked, where they have some independent
    // freedom and some shared freedom, I havent actually studied a map with
    // this characteristic, but I have to account for it.
    // multiplying the freedoms of each group (linked groups count as one)
    // will give us the answer to how many possible arrangements exists in the
    // map
    // first off, lets clone everything we need

    let mut working_vector = map.base_arrangement.clone();
    let mut groups = map.spring_groups.clone();
    let reference = &map.maps.springs;
    // open vector to track total freedoms in map
    let mut freedom_tracker: Vec<usize> = Vec::with_capacity(8);
    // filter groups with more than one freedom
    let mut free_groups: Vec<SpringGroup> = get_free_groups(&groups, &working_vector, &reference);
    // check if any group has more than one freedom, if no, return 1.
    if free_groups.len() == 0 {
        return 1;
    }
    // separate the single groups from the linked groups
    let (mut single_groups, mut linked_groups) =
        group_organizer(&free_groups, &working_vector, &reference);

    return 0;
}
fn group_organizer(
    groups: &Vec<SpringGroup>,
    working_vector: &Vec<char>,
    reference: &Vec<char>,
) -> (Vec<SpringGroup>, Vec<Vec<SpringGroup>>) {
    // Separates single groups from linked groups, and puts them into
    // named vectors. Single groups are simply shoved into their vector
    // while linked groups need some extra care, we want to sort the
    // sets of linked groups into separate vectors, to be done with it.
    let mut singles: Vec<SpringGroup> = Vec::with_capacity(4);
    let mut linked: Vec<Vec<SpringGroup>> = Vec::with_capacity(4);
    let mut bounds = groups.len();
    let mut index: usize = 0;
    loop {
        if index == bounds {
            break;
        }
        let active_group = groups[index].clone();
        if is_single(&index, &active_group, &groups, &working_vector, &reference) {
            singles.push(active_group);
            index += 1;
            continue;
        } else {
            // manage linked groups here
        }
    }

    (singles, linked)
}
fn is_single(
    index: &usize,
    active_group: &SpringGroup,
    groups: &Vec<SpringGroup>,
    working_vector: &Vec<char>,
    reference: &Vec<char>,
) -> bool {
    // checks for singles
    let bounds = groups.len();
    let active_id = active_group.id;
    // lazy checks first
    // if theres only one group in vector, group is single
    // if group is last man standing, group is single
    // if next group id is not contiguous, group is single
    if bounds == 1 || index + 1 == bounds || active_id + 1 != groups[index + 1].id {
        return true;
    }
    // the next check is a bit more involved.
    // We count active groups current freedoms
    // Then we move all subsequent groups fully to the right
    // We count the freedoms again. If the number
    // of freedoms remain the same, the group is single.
    // This should cover all cases.
    let mut work_vec = working_vector.clone();
    let current_freedom = freedom_counter(&active_group, &work_vec, &reference);
    work_vec = shake_right(&index, &groups, &work_vec, &reference);
    let new_freedom = freedom_counter(&active_group, &work_vec, &reference);
    if current_freedom == new_freedom {
        return true;
    }
    false
}
fn shake_right(
    index: &usize,
    groups: &Vec<SpringGroup>,
    work_vec: &Vec<char>,
    reference: &Vec<char>,
) -> Vec<char> {
    // makes all groups after active group as right leaning as possible.
    // imagine picking the vector up by the active group, and shaking it
    // so that the ones not held fall all the way to the right
    // this ensures that freedom count is accurate, and might save a huge
    // headache later on
    // copy and clone whatever you need
    let index = *index;
    let mut working_vector = work_vec.clone();

    // this loop goes backwards through the groups and moves them as far right
    // as possible, the counter counts the index of the groups vector
    // breaks when its through all subsequent groups in relation to the active
    // group
    let mut counter = work_vec.len();
    loop {
        counter -= 1;
        if counter == index {
            break;
        }
        // some semantic clarity
        let active_group = groups[counter].clone();
        let group_id = char::from_digit(active_group.id as u32, 10).unwrap();
        let mut start_index = active_group.start_index;
        let mut window = start_index;
        let mut leading_edge = start_index + active_group.size - 1;
        // clean working vector for current group
        while window <= leading_edge {
            working_vector[window] = reference[window];
            window += 1;
        }
        // get freedoms of group, and substract one (to account for current pos)
        // then apply as offset to start_index and leading edge, then place
        // group ID into working vector
        let offset = freedom_counter(&active_group, &working_vector, reference) - 1;
        start_index += offset;
        leading_edge += offset;
        window = start_index;
        while window <= leading_edge {
            working_vector[window] = group_id;
            window += 1;
        }
    }

    working_vector
}
fn freedom_counter(
    active_group: &SpringGroup,
    working_vector: &Vec<char>,
    reference: &Vec<char>,
) -> usize {
    // counts freedoms of a group by moving group step by step over
    // working vector until it encounters another group or the edge of working vector
    // Defining stuff for semantic clarity
    let index = active_group.start_index;
    let size = active_group.size;
    let mut leading_edge = index + active_group.size - 1;
    // window defines a range to the left of leading edge,
    // representing the complete group in the working vector
    let mut window = leading_edge - (size - 1);
    let bounds = working_vector.len();
    let valid = '?';
    // this is the counter that tracks the freedoms
    let mut freedoms: usize = 1;
    // lets loop over our stuff!
    loop {
        // set index of next spot to consider, set valid flag to false
        // check for break condition
        let mut valid_flag = true;
        let next = leading_edge + 1;
        if next == bounds || working_vector[next].is_numeric() || reference[window] == '#' {
            break;
        }
        // increment leading edge and window
        leading_edge = next;
        window = leading_edge - (size - 1);
        let mut count = window;
        // check if window covers only valid spots
        while count <= leading_edge {
            if working_vector[count] != valid {
                valid_flag = false;
                continue;
            }
            count += 1;
        }
        freedoms += 1;
    }
    freedoms
}
fn get_free_groups(
    groups: &Vec<SpringGroup>,
    working_vector: &Vec<char>,
    reference: &Vec<char>,
) -> Vec<SpringGroup> {
    // -> Vec<SpringGroup>
    // works out what groups have any freedoms at all
    let mut output_groups = groups.clone();
    let mut counter = output_groups.len();
    let mut check_vector = working_vector.clone();
    // removes obviously locked groups
    loop {
        if counter == 0 {
            break;
        }
        counter -= 1;
        if is_locked(&output_groups[counter], reference) {
            output_groups.remove(counter);
        }
    }

    counter = output_groups.len();
    // removes less obviously locked groups
    // always counting backwards
    loop {
        if counter == 0 {
            break;
        }
        counter -= 1;
        let (check_vector, still_locked) =
            locked_deep(&output_groups[counter], &check_vector, &reference);
        if still_locked {
            output_groups.remove(counter);
        }
    }
    output_groups
}
fn locked_deep(
    group: &SpringGroup,
    working_vector: &Vec<char>,
    reference: &Vec<char>,
) -> (Vec<char>, bool) {
    // -> (Vec<char>, bool)
    // checks if group is locked, even if stuff moves around. This is very
    // step by step, final, optimized solution will do more stuff at once
    // lots of useful calculations done here are simply dropped, but you know
    // cognitive load is a beast, and this one is full of it.

    // define next index after group, and its neighbour
    let mut check_vec = working_vector.clone();
    let mut start_index = group.start_index;
    let next = group.start_index + group.size;
    let neighbour = next + 1;

    // checks if group is at edge, and moveable, if moveable, move
    // update checkvec, since we're only modelling now, sloppy and fast
    // is the name of the game
    // DBUG println!("currently checking group ID {}", group.id);
    if neighbour == reference.len() {
        check_vec[start_index] = reference[start_index];
        check_vec[start_index + 1] = '0';
        return (check_vec, false);
    }
    if check_vec[neighbour].is_numeric() {
        println!("{:?} groupid: {} also locked", check_vec, group.id);
        return (check_vec, true);
    } else {
        check_vec[start_index] = reference[start_index];
        check_vec[start_index + 1] = '0';
        return (check_vec, false);
    }
}
fn is_locked(group: &SpringGroup, reference: &Vec<char>) -> bool {
    // checks if group is locked in place by topography
    let start = reference[group.start_index];
    let next = group.start_index + group.size;
    if start == '#' || next == reference.len() || reference[next] == '.' {
        println!("{:?} groupid: {} locked", reference, group.id);
        return true;
    }
    false
}
fn build_cm(maps: Maps) -> ConditionMap {
    // -> ConditionMap
    // Get ConditionMap.spring_groups: Vec<SpringGroup>
    // Get left leaning first valid arrangement
    let (spring_groups, base_arrangement) = build_spr_groups(&maps);
    let map = ConditionMap {
        maps,
        base_arrangement,
        spring_groups,
    };

    map
    // Fill and return struct
}
fn build_spr_groups(maps: &Maps) -> (Vec<SpringGroup>, Vec<char>) {
    // -> (Vec<SpringGroup>, Vec<char>)
    // The third time I'm attempting to build a decent function for this..
    // Takes maps, and uses them to build a valid base arrangement of
    // spring groups in the spring maps, also returns a vector of
    // spring group coordinates.

    // Builds a valid base case of spring groups
    // The rules are:

    // - Groups must appear in order provided
    // - No groups on '.'
    // - All '#' must be covered by a group
    // - '?' can be filled
    // - No groups may be adjacent to another group

    // this time around, to give some extra safety, i will consume
    // placed groups into a new vector as I go, this should eliminate
    // some errors encountered in the previous two attempts
    // Actual function code and comments start below here:
    // First set up a working enviroment and some helpful constants
    // Reference, to have a clean map to compare to
    // working vector to do process and analyze
    // Vector to collect placed groups
    // Also make groups variable, for semantic clarity
    // Have valid symbols ready
    // Instantiating vector which will hold the finished SpringGroup structs
    let reference = maps.springs.clone();
    let mut working_vector = reference.clone();
    let mut output_vector: Vec<char> = Vec::with_capacity(30);
    let groups = maps.groups.clone();
    let valid_symbols: [char; 2] = ['?', '#'];
    let mut output_groups: Vec<SpringGroup> = Vec::with_capacity(8);
    //println!("{:?}", reference);

    for (index, group_size) in groups.iter().enumerate() {
        // loops through groups, checks them against reference, and
        // eventually transfers valid groups from working vector into
        // output vector. Index is used as group ID in SpringGroup struct
        // Define window matching size of current group to pass over
        // working Vector.
        // Instantiate counter for indexing.
        // Instantiate bounds to avoid overflows
        // Make group amount variable to help semantic clarity
        // and start loop.
        let mut counter: usize = 0;
        let bounds = working_vector.len();
        let group_amount = groups.len();
        'group_loop: loop {
            let mut window = counter + group_size;
            let next_index = counter + group_size;
            let mut valid_flag: bool = true;
            // start or advance window
            // check that window contains all valid symbols
            while window > counter {
                window -= 1;
                if valid_symbols.contains(&working_vector[window]) == false {
                    valid_flag = false;
                    break;
                }
            }
            // check for trailing '#', also check your not looking out of bounds
            if next_index < bounds && working_vector[next_index] == '#' {
                valid_flag = false;
            }
            // if both checks pass, the group has a valid placement.
            // Insert group into working vector, split group and next index
            // (if available) away from working vector, and push to
            // output_vector
            if valid_flag {
                // reset window
                window = next_index - 1;
                // loop through window and insert group id where needed
                while window >= counter {
                    working_vector[window] = char::from_digit(index.clone() as u32, 10).unwrap();
                    if window == 0 {
                        break;
                    }
                    window -= 1;
                }
                // split working vector, push left side to output, make right side
                // into new working vector
                if index + 1 == group_amount {
                    output_vector.append(&mut working_vector);
                } else {
                    let (left, right) = working_vector.split_at(next_index + 1);
                    for el in left {
                        output_vector.push(*el);
                    }
                    working_vector = (*right).to_vec();
                }
                break 'group_loop;
            }
            counter += 1;
        }
    }
    // get start index of each group, build SpringGroup struct and push to output
    for (id, size) in groups.iter().enumerate() {
        let start_index = get_start_index(&output_vector, &id);
        let spring_group = SpringGroup {
            id,
            size: *size,
            start_index,
        };
        output_groups.push(spring_group);
    }
    // check if there are any '#' in the output vector
    // unfuckify any occurences
    if is_fucked(&output_vector) {
        // DBUG println!("source:         {:?}", reference);
        // DBUG println!("output:         {:?}", output_vector);
        (output_groups, output_vector) = unfuckify(&reference, &output_vector, &output_groups);
    }
    // DBUG println!("output vector {:?}", output_vector);
    (output_groups, output_vector)
}
fn is_fucked(spring_map: &Vec<char>) -> bool {
    let forbidden: char = '#';
    if spring_map.contains(&forbidden) {
        return true;
    }
    false
}
fn get_start_index(spring_map: &Vec<char>, group_id: &usize) -> usize {
    let char_id = char::from_digit(group_id.clone() as u32, 10).unwrap();
    let mut output: usize = 0;
    for (index, el) in spring_map.iter().enumerate() {
        if *el == char_id {
            output = index;
            break;
        }
    }
    output
}
fn unfuckify(
    reference: &Vec<char>,
    working_vector: &Vec<char>,
    input_groups: &Vec<SpringGroup>,
) -> (Vec<SpringGroup>, Vec<char>) {
    // -> (Vec<SpringGroup>, Vec<char>)
    // this little guy fixes your messed up spring groups
    // and returns edited Vec<SpringGroup> and Vec<char>
    // lets be REALLY explicit here, this might get messy
    // clone all the things and set up for searching
    let mut work_vec = working_vector.clone();
    let mut groups = input_groups.clone();
    let forbidden: char = '#';
    // yes, I understand that the hash is both valid and forbidden
    // context matters a lot here..
    let valid_symbols: [char; 2] = ['?', '#'];
    // build a vector of the start indices of the groups we are
    // dealing with, should probably be inside loop, but Im leaving it outside
    // for now, and opt for explicit vector manipulation inside loop instead
    let mut start_indices: Vec<usize> = Vec::with_capacity(8);
    for group in &groups {
        start_indices.push(group.start_index.clone());
    }
    // the rest of the function is an infinite loop, it runs until everything
    // is fixed, and exits with return values once all conditions for valid
    // groups and base case are met.
    loop {
        // check if its time to return
        if is_fucked(&work_vec) == false {
            // checks if unfuckification put two groups into contact with
            // eachother. The dreaded unfuckify-fuckup
            if is_deeply_fucked(&groups) {
                // DBUG println!("Source:         {:?}", reference);
                // DBUG println!("deeply fucked:  {:?}", work_vec);
                (groups, work_vec) = deep_unfuckify(&reference, &work_vec, &groups);
                // after deep unfuckification, we should probably do a
                // sanity check, to see that we didnt expose any new hashes
                // however, I'm lazy, and this case does not appear in my
                // data, so I'll leave it for later
            }
            // DBUG println!("Should be good: {:?}", work_vec);
            return (groups, work_vec);
        }
        // find index of the last '#' in working Vector
        // this is done very naively by going through vector
        // and updating the work index value
        let mut hash_location: usize = 0;
        for (index, el) in work_vec.iter().enumerate() {
            if *el == forbidden {
                hash_location = index;
            }
        }
        // now that we got the location, we need to determine what group
        // needs to move where to make a left leaning valid configuration
        // this will be done by checking the hash_location against the
        // start_indices vector, we move backwards through the start_indices
        // vector.
        let mut counter = start_indices.len() - 1;
        while start_indices[counter] > hash_location {
            counter -= 1;
        }
        // now counter holds the id and index of the SpringGroup that needs to
        // change to cover the '#' and unfuck this particular hash_location lets
        // hold this group in a named variable

        let mut active_group = groups[counter].clone();
        // lets also get the size of the group into a variable, for semantic
        // clarity
        let size = active_group.size;
        // define last index of group
        let group_end = active_group.start_index + size;
        // i think we are ready to do some inserting now
        // first, lets determine the new start index for the group
        // first we lift the group from the work_vec, to avoid matching to itself
        // set counter to group start index
        counter = active_group.start_index;
        while counter < group_end {
            work_vec[counter] = reference[counter];
            counter += 1;
        }

        // just checking this is correct
        // work_vec[counter] = '!'; Logic checks out
        // this should also help uncover new hashes that needs dealing with
        // then we need to figure out the earliest good spot to place the group
        // that is accomplished by starting at the hash location and searching
        // backwards for ? and #, as far as it goes while group size still covers
        // hash_location, if the furthest location borders a hash, that location is
        // invalid, as there will be a number there once the whole thing is unfuckified
        // instantiate new index variable.
        // also set start check variable for new index - 1 for semantic clarity
        let mut new_start = hash_location;
        let mut start_check = new_start - 1;
        while hash_location - start_check < size {
            if valid_symbols.contains(&reference[start_check]) {
                new_start = start_check;
                start_check -= 1;
                continue;
            }
            break;
        }
        // Now, if my logic is solid, new start should hold the new start index
        // for the active group, so lets change the value
        // and update struct in groups vector
        // also update start indices vector
        active_group.start_index = new_start;
        groups[active_group.id] = active_group.clone();
        start_indices[active_group.id] = new_start;

        // then we insert the new group into work_vec
        // set the counter to last index of group
        counter = new_start + size - 1;
        // also set g_id_char for semantic clarity
        let g_id_char = char::from_digit(active_group.id.clone() as u32, 10).unwrap();
        while counter >= new_start {
            work_vec[counter] = g_id_char;
            counter -= 1;
        }

        // I'm putting it down here, but the code should happen above.
        // Theres one more case that needs to be accounted for
        // I'm pretty sure its possible, with the algo as it is, to overwrite
        // later groups, if the hash is placed in the middle of the pack.
        // There needs to exist both a check, that the right side of the group
        // is not bordering or overlapping the next group, and a method to
        // solve the situation if it arises. I will find an example, its bound to be
        // in the set.
        // At least I know that I'm right now, this situation, at least with
        // neighborings groups, arises.
        // But we have some information. We _know_ that the active group is
        // in an optimal location, its both maximally left leaning, and its covering
        // the hashes it needs to cover. So its placed just right. Now, the pressing
        // question, is this best solved with some code right here, or do we need an
        // unfuckify 2 function?
        // I'm leanig towards a new function, unfuckify_2, the depths of fucked. The
        // checks should be done by comparing data in the SpringGroups, not by
        // analyzing the output string, if the checks detect adjacent or overlapping
        // numbers. The output string, along with a reference and working string
        // need to be processed by the unfuckify_2 function.
        // I believe is_super_fucked() can check for adjacent or overlapping groups
        // but actually checking the work is for tomorrow.
        // Did some semantic cleaning, the errors are now deeply fucked, calling for
        // deep unfuckification.
        // DBUG println!(
        // DBUG     "                {:?} the hash is at index {}, group {} at start index {} needs to move",
        // DBUG     work_vec, hash_location, active_group.id, active_group.start_index
        // DBUG );
    }
}
fn deep_unfuckify(
    reference: &Vec<char>,
    working_vector: &Vec<char>,
    input_groups: &Vec<SpringGroup>,
) -> (Vec<SpringGroup>, Vec<char>) {
    //  -> (Vec<SpringGroup>, Vec<char>)
    // Fixes output vector and Spring groups to not have overlapping or adjacent
    // groups. I believe this is the last case that needs to be covered before we
    // can say with certainty that we have a properly parsed input line.

    // first of all, clone everything. Lazy, I know, but will have to do for now.
    let mut groups = input_groups.clone();
    let mut work_vec = working_vector.clone();
    // set up values to be used for counting
    // we need to know how many groups we have
    let group_amount = groups.len();
    // we need our valid symbols aswell
    let valid_symbols: [char; 2] = ['?', '#'];
    // again we set up an infinite loop which exits function and returns
    // the values once all conditions are met.
    loop {
        // first check if its time to exit
        if is_deeply_fucked(&groups) == false {
            // DBUG println!("LGTM!           {:?}", work_vec);
            return (groups, work_vec);
        }
        // figure out the rightmost group that needs changing
        // set a counter, since we are going backwards.
        // remember that the base assumption is that the further left a group is
        // placed, the more confident we are of its correctness
        let mut counter = group_amount;
        loop {
            counter -= 1;
            // first some semantic clarity
            let group_to_check_index = groups[counter].start_index;
            let reference_group_placement =
                groups[counter - 1].start_index + groups[counter - 1].size;
            if group_to_check_index <= reference_group_placement {
                break;
            }
        }
        // counter has now captured the index of the group we want to modify
        // lets continue being super explicit
        let mut active_group = groups[counter].clone();
        let next = active_group.start_index + active_group.size;
        let left_group = &groups[counter - 1];
        // the procedure now, is to move the active group over to the next valid
        // spot on the right in the working vector.

        // I believe it is safest to clean both groups associated with the
        // fuckening from work_vec.
        // Then reinsert the left one.
        // Then check validity of the next step for active group
        // if not valid, search ahead until a valid placement is found
        // then insert active group.
        // we assume the data is good so no overflow will ever happen
        let left_group_range = left_group.start_index + left_group.size - 1;
        let mut active_group_range = active_group.start_index + active_group.size - 1;
        // cleaning work_vec of left group, set counter
        counter = left_group_range;
        while counter >= left_group.start_index {
            work_vec[counter] = reference[counter];
            counter -= 1;
        }
        // cleaning work_vec of active group, set counter
        counter = active_group_range;
        while counter >= active_group.start_index {
            work_vec[counter] = reference[counter];
            counter -= 1;
        }
        // then reinsert left group. It might seem a bit redundant, but
        // this clean/redraw step consideres the case were left actually
        // overwrote part of active group.

        counter = left_group_range;
        while counter >= left_group.start_index {
            let char_id = char::from_digit(left_group.id as u32, 10).unwrap();
            work_vec[counter] = char_id;
            counter -= 1;
        }
        // check if next index for active group is not valid
        if valid_symbols.contains(&work_vec[next]) == false {
            // here we need some logic to look ahead for the next valid
            // placement of the group
            // it needs to account for the range of active group
            // the main purpose of this loop is to change "next"
            // variable so the group update below is valid
            // However, I'm lazy right now, and my set does not
            // contain this case, so this is saved for the
            // optimizing pass that may or may not come
        }
        // DBUG println!("BREAK CHECK {:?}", work_vec);
        // determine new start index for active group by substracting
        // (group size - 1) from next variable and update active group
        // SpringGroup, and update the same in groups vector
        let new_start_index = next - (active_group.size - 1);
        active_group.start_index = new_start_index;
        groups[active_group.id] = active_group.clone();

        // update active group range and insert active group into work_vec
        active_group_range = active_group.start_index + active_group.size - 1;
        counter = active_group_range;
        while counter >= active_group.start_index {
            let char_id = char::from_digit(active_group.id as u32, 10).unwrap();
            work_vec[counter] = char_id;
            counter -= 1;
        }
    }
}
fn is_deeply_fucked(input_groups: &Vec<SpringGroup>) -> bool {
    // -> bool
    // checks for adjacent or overlapping groups
    let mut counter = input_groups.len() - 1;
    loop {
        if input_groups[counter].start_index
            <= input_groups[counter - 1].start_index + input_groups[counter - 1].size
        {
            return true;
        }
        counter -= 1;
        if counter == 0 {
            break;
        }
    }
    false
}
fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
    // Parses line from full data into types used in:
    // ConditionMap.map_springs and
    // ConditionMap.map_groups

    // crack input string into two parts
    let cracked: Vec<&str> = line.split_whitespace().collect();
    // parse the two output vectors into Vec<char> for spring map
    // and Vec<usize> for group map
    let map_springs: Vec<char> = cracked[0].chars().collect();
    let map_groups: Vec<usize> = cracked[1]
        .split(',')
        .map(|s| s.parse().expect("this should have worked"))
        .collect();
    // return tuple of nice data
    (map_springs, map_groups)
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
