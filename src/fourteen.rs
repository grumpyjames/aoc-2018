pub fn digits(num: usize) -> Vec<usize> {
    let mut result = Vec::new();
    digits_r(num, &mut result);

    result.into_iter().rev().collect()
}

fn digits_r(num: usize, res: &mut Vec<usize>) {
    let mod_ten = num % 10;
    res.push(mod_ten);
    if num >= 10 {
        digits_r((num - mod_ten) / 10, res);
    }
}

fn main() {
    let recipes: Vec<usize> = [3, 7].to_vec();

    run_recipe_simulator(recipes.clone(), 9);
    run_recipe_simulator(recipes.clone(), 5);
    run_recipe_simulator(recipes.clone(), 18);
    run_recipe_simulator(recipes.clone(), 2018);
    run_recipe_simulator(recipes.clone(), 147061);

    run_until_found(recipes.clone(), 51589);
    run_until_found(recipes.clone(), 92510);
    run_until_found(recipes.clone(), 59414);
    run_until_found(recipes.clone(), 147061);
}

fn run_recipe_simulator(initial: Vec<usize>, recipes_to_make: usize) -> () {
    let mut recipes = initial;
    let mut elf_one_index = 0;
    let mut elf_two_index = 1;
    for _i in 0..(recipes_to_make + 10) {
        append_new_recipes(&mut recipes, elf_one_index, elf_two_index);
        elf_one_index = (recipes[elf_one_index] + elf_one_index + 1) % recipes.len();
        elf_two_index = (recipes[elf_two_index] + elf_two_index + 1) % recipes.len();
    }
    print!("{}: ", recipes_to_make);
    for i in recipes_to_make..recipes_to_make + 10 {
        print!("{}", recipes[i]);
    }
    println!();

    print!("{}: ", recipes_to_make);
    for i in recipes_to_make..recipes_to_make + 5 {
        print!("{}", recipes[i]);
    }
    println!();
}

fn run_until_found(initial: Vec<usize>, recipes_to_make: usize) -> () {
    let mut recipes = initial;
    let mut elf_one_index = 0;
    let mut elf_two_index = 1;
    let target_digits = digits(recipes_to_make);
    println!("Searching for {:?}", target_digits);

    let mut found = false;
    while !found {
        let recipe_one = recipes[elf_one_index];
        let recipe_two = recipes[elf_two_index];
        let new_recipe_total = recipe_one + recipe_two;
        for digit in digits(new_recipe_total) {
            if !found
            {
                recipes.push(digit);
                found = recipes.ends_with(&target_digits);
            }
        }

        elf_one_index = (recipes[elf_one_index] + elf_one_index + 1) % recipes.len();
        elf_two_index = (recipes[elf_two_index] + elf_two_index + 1) % recipes.len();
    }

    println!("{}", recipes.len() - target_digits.len());
}

fn append_new_recipes(recipes: &mut Vec<usize>, elf_one_index: usize, elf_two_index: usize) -> () {
    let recipe_one = recipes[elf_one_index];
    let recipe_two = recipes[elf_two_index];
    let new_recipe_total = recipe_one + recipe_two;
    for digit in digits(new_recipe_total) {
        recipes.push(digit)
    }
}