use std::collections::HashMap;

pub fn driver() {
    // Stem and leaf plot from statistics
    // Attempts from Organic Chemistry Tutor video: https://youtu.be/MUCvUgGfzdo?feature=shared

    // Set 1
    println!("Set 1:");
    let mut set_1: Vec<u8> = vec![15, 27, 8, 17, 13, 22, 24, 25, 13, 36, 32, 32, 32, 28, 7];
    set_1.sort(); // should be in the function #FIXME: remove this from the driver
    let stem_and_leaf_plot: &HashMap<u8, Vec<u8>> = &steam_leaf(set_1);
    sl_table(stem_and_leaf_plot);

    // Set 2
    println!("\nSet 2:");
    let mut set_2: Vec<u8> = vec![72, 85, 89, 93, 88, 76, 108, 115, 97, 102, 113];
    set_2.sort();
    // let stem_and_leaf_plot_2 = &stem_leaf(set_2);

    // Set 3, side by side stem and leaf plot
    println!("\nSet 3:");
    let mut set_3: Vec<u8> = vec![];
    set_3.sort();
}

fn steam_leaf(sample: Vec<u8>) -> HashMap<u8, Vec<u8>> {
    let mut stem_leaf_plot: HashMap<u8, Vec<u8>> = HashMap::new();
    for element in &sample {
        let stem: u8 = element / 10;
        let leaf: u8 = element % 10;
        let entry: &mut Vec<u8> = { stem_leaf_plot.entry(stem).or_insert(Vec::new()) };
        // let entry: &mut Vec<u8> = { stem_leaf_plot.entry(element / 10).or_insert(Vec::new()) }; // inline variable

        entry.push(leaf);
    }

    stem_leaf_plot
}

fn sl_table(sf_plot: &HashMap<u8, Vec<u8>>) {
    let mut stem_keys: Vec<_> = sf_plot.keys().collect();
    stem_keys.sort();

    for &stem_key in stem_keys {
        print!("{} | ", stem_key);
        let leaf_values: &Vec<u8> = sf_plot.get(&stem_key).unwrap();
        for &leaf_value in leaf_values {
            print!("{}", leaf_value);
        }
        println!();
    }
}
