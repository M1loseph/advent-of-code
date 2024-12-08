use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type OrderingRules = HashMap<i64, HashSet<i64>>;
type Update = Vec<i64>;
type Updates = Vec<Update>;

fn read_input() -> (OrderingRules, Updates) {
    let file_content = read_to_string("input.txt").unwrap();

    let mut ordering_rules = HashMap::new();
    let mut updates = Vec::new();
    let mut parsing_ordering_rules = true;

    for line in file_content.lines() {
        if line.is_empty() {
            parsing_ordering_rules = false;
            continue;
        }

        if parsing_ordering_rules {
            let (left, right) = line.split_once("|").unwrap();
            let value = ordering_rules
                .entry(left.parse().unwrap())
                .or_insert(HashSet::new());
            value.insert(right.parse().unwrap());
        } else {
            let update = line
                .split(",")
                .map(|page_number| page_number.parse().unwrap())
                .collect();
            updates.push(update);
        }
    }

    (ordering_rules, updates)
}

fn is_correct(update: &Update, ordering_rules: &OrderingRules) -> bool {
    for i in 0..update.len() {
        if let Some(numbers_after_this) = ordering_rules.get(&update[i]) {
            for j in 0..i {
                if numbers_after_this.contains(&update[j]) {
                    return false;
                }
            }
        }
    }
    true
}

fn puzzle_1(ordering_rules: &OrderingRules, updates: &Updates) {
    let sum = updates
        .iter()
        .map(|update| {
            return if is_correct(update, ordering_rules) {
                0
            } else {
                let middle_index = update.len() / 2;
                update[middle_index]
            };
        })
        .sum::<i64>();
    println!("The sume of median of correct instructions is {sum}");
}

fn fix_update(ordering_rules: &OrderingRules, update: &Update) -> Update {
    let mut correct_update = Vec::new();
    'outer: for number in update {
        if let Some(after_numbers) = ordering_rules.get(number) {
            for i in 0..correct_update.len() {
                if after_numbers.contains(&correct_update[i]) {
                    correct_update.insert(i, *number);
                    continue 'outer;
                }
            }
        }
        correct_update.push(*number);
    }
    correct_update
}

fn puzzle_2(ordering_rules: &OrderingRules, updates: &Updates) {
    let sum = updates
        .iter()
        .map(|update| {
            return if is_correct(update, ordering_rules) {
                0
            } else {
                let correct = fix_update(ordering_rules, update);
                let middle_index = correct.len() / 2;
                correct[middle_index]
            };
        })
        .sum::<i64>();
    println!("The sume of median of corrected invalid instructions is {sum}");
}

fn main() {
    let (ordering_rules, updates) = read_input();
    puzzle_1(&ordering_rules, &updates);
    puzzle_2(&ordering_rules, &updates);
}
