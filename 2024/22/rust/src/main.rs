use std::fs::read_to_string;

const SECRETS_DURING_A_DAY: usize = 2000;

fn prune(value: u64) -> u64 {
    value % 16777216
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn next_secret(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    let secret = prune(mix(secret, secret * 2048));
    secret
}

fn new_secret_numbers_sequence(initial_secret: u64, length: usize) -> Vec<u64> {
    let mut result = Vec::new();
    let mut secret = initial_secret;
    for _ in 0..length {
        secret = next_secret(secret);
        result.push(secret);
    }
    result
}

fn puzzle_1(secrets: &Vec<u64>) {
    let sum_of_secrets = secrets
        .iter()
        .map(|initial_secret| {
            new_secret_numbers_sequence(*initial_secret, SECRETS_DURING_A_DAY)
                [SECRETS_DURING_A_DAY - 1]
        })
        .sum::<u64>();
    println!("Sum of {SECRETS_DURING_A_DAY}th secrets: {sum_of_secrets}");
}

struct PriceAndDiff {
    price: u64,
    diff: i32,
}

fn puzzle_2(secrets: &Vec<u64>) {
    let sellers_secrets: Vec<Vec<u64>> = secrets
        .iter()
        .map(|initial_secret| new_secret_numbers_sequence(*initial_secret, SECRETS_DURING_A_DAY))
        .collect();
    let precomputed_diffs: Vec<Vec<PriceAndDiff>> = sellers_secrets.iter().map(|secrets| {
        let mut result = Vec::new();
        for i in 1..secrets.len() {
            result.push(PriceAndDiff {
                price: secrets[i] % 10,
                diff: (secrets[i - 1] as i64 % 10 - secrets[i] as i64 % 10) as i32,
            });
        }
        result
    }).collect();
    let mut best_price = None;
    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let mut total_price = 0;
                    for diff_and_price in &precomputed_diffs {
                        let mut price = None;
                        for i in 3..diff_and_price.len() {
                            if diff_and_price[i - 3].diff == a
                                && diff_and_price[i - 2].diff == b
                                && diff_and_price[i - 1].diff == c
                                && diff_and_price[i].diff == d
                            {
                                price = Some(diff_and_price[i].price);
                                break;
                            }
                        }
                        if let Some(price) = price {
                            total_price += price;
                        }
                    }
                    match best_price {
                        Some(best_so_far) => {
                            if best_so_far < total_price {
                                best_price = Some(total_price);
                            }
                        }
                        None => {
                            best_price = Some(total_price);
                        }
                    }
                }
            }
            println!("{a}{b}");
        }
    }
    println!("Best price is {:?}", best_price);
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let initial_secrets: Vec<u64> = file_content
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    puzzle_1(&initial_secrets);
    puzzle_2(&initial_secrets);
}
