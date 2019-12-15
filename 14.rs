use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Rule {
    material: String,
    qty: u64,
}

fn parse_qtys(rules: &HashMap<std::string::String, (u64, std::vec::Vec<Rule>)>, curr: &std::string::String, surplus: &mut HashMap<std::string::String, u64>, need: &u64) -> u64 {
    let default_vals: (u64, std::vec::Vec<Rule>) = (0, vec![]);
    let rule = match rules.get(curr) {
        Some(number) => &number,
        _ => &default_vals,
    };

    let curr_qty = match surplus.get(curr) {
        Some(num) => num,
        _ => &0,
    };
    let mut actual_need: u64 = *need;

    if curr_qty > &0 {
        if curr_qty > &need {
            surplus.insert(curr.to_string(), curr_qty - need);
            return 0
        } else {
            actual_need = actual_need - curr_qty;
            surplus.insert(curr.to_string(), 0);
        }
    }

    let mut total: u64 = 0;

    for r in &rule.1 {
        let batches = ((actual_need as f64) / (rule.0 as f64)).ceil() as u64;
        surplus.insert(curr.to_string(), batches * rule.0 - actual_need);
        if r.material == "ORE" {
            let produced = r.qty * batches;
            return produced;
        }
        total = total + parse_qtys(rules, &r.material, surplus, &(batches * r.qty));
    }

    return total

}

fn main() {
    let input = fs::read_to_string("data/14")
        .expect("Failed to read file");
    let lines: Vec<&str> = input.split("\n").collect();

    let mut rules = HashMap::new();

    for line in lines {
        let lines: Vec<&str> = line.split(" => ").collect();
        let key: Vec<&str> = lines[1].split(" ").collect();

        let out_qty: u64 = match key[0].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let out_prod = key[1].trim().to_string();
        let mut ingredients: Vec<Rule> = Vec::new();
        let ingreds: Vec<&str> = lines[0].split(", ").collect();
        
        for ingred in ingreds {
            
            let params: Vec<&str> = ingred.split(" ").collect();
            let qty: u64 = match params[0].trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            ingredients.push(Rule {
                qty: qty,
                material: params[1].trim().to_string(),
            });
        }

        rules.insert(out_prod, (out_qty, ingredients));
    }

    println!("{}", parse_qtys(&rules, &"FUEL".to_string(), &mut HashMap::new(), &1));

    let mut left = 0;
    let mut right = 5_000_000;
    let target = 1_000_000_000_000;
    let mut best = 0;
    let mut best_i = 0;


    while left < right {
        let mid = (left + right) / 2;
        let ores = parse_qtys(&rules, &"FUEL".to_string(), &mut HashMap::new(), &(mid));

        if ores > best && ores <= target {
            best = ores;
            best_i = mid;
        }

        if ores < target {
            left = mid + 1; 
        } else {
            right = mid - 1;   
        }
    }

    println!("{} {}", best, best_i);
}
