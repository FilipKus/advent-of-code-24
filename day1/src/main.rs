#![allow(unused_variables)]

use std::{fs::{self, File}, i32, io::{self, BufRead, BufReader}, vec};

fn add_num_sorted_vec (added_num: i32, vector: Vec<i32>) -> Vec<i32>{
        let vector_length: usize = vector.len();
        let mut new_vector = vec![0; vector_length];

        if vector[0] == 0{
            new_vector[0] = added_num;
            return new_vector;
        }
        
        for i in 0..vector_length{
            if vector[i] > added_num && i == 0{

                new_vector[0] = added_num;
                new_vector[1..].clone_from_slice(&vector[0..vector_length-1]);
                return new_vector;

            } else if vector[i] > added_num {

                if i == 1{
                    new_vector[0] = vector[0];
                } else {
                    new_vector[0..i].clone_from_slice(&vector[0..i]);
                }                
                new_vector[i] = added_num;
                new_vector[i+1..].clone_from_slice(&vector[i..vector_length-1]);
                return new_vector;

            } else if vector[i] == 0 {
                new_vector[0..i].clone_from_slice(&vector[0..i]);
                new_vector[i] = added_num;
                return new_vector;
            }
        }        
        return new_vector;
    }

fn sort_vec (vector: Vec<i32>) -> Vec<i32>{
    let mut new_vector = vec![0; vector.len()];
    for i in 0..vector.len(){
        new_vector = add_num_sorted_vec(vector[i], new_vector);
        // println!("{:?}", new_vector);        
    }

    // println!("{:?}", new_vector);
    return new_vector;
}

fn calc_distance (vector_1: Vec<i32>, vector_2: Vec<i32>) -> i32 {
    let mut total_distance: i32 = 0;

    for i in 0..vector_1.len(){
        total_distance = total_distance + (vector_1[i] - vector_2[i]).abs();
        // println!("{}", total_distance);
    }

    return total_distance;
}

fn obtain_vectors_from_file (filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut vector_1: Vec<i32> = Vec::new();
    let mut vector_2: Vec<i32> = Vec::new();
    

    for line in reader.lines(){
        let line = line?;

        // Determine where the gap is between the two numbers
        let mut gap_index = 0;
        for i in 0..line.len(){
            if line.chars().nth(i).unwrap() == ' ' && gap_index == 0{
                gap_index = i;
                // println!("{}", gap_index);
            }
        }

        let first_num = &line[0..gap_index];
        let second_num = &line[gap_index+3..];

        vector_1.push(first_num.parse().unwrap());
        vector_2.push(second_num.parse().unwrap());

        // println!("{}", first_num);
        // println!("{}", second_num);
    }

    // println!("{:?}", vector_1);
    // println!("{:?}", vector_2);

    Ok((vector_1, vector_2))
}

fn num_appearance (vector: &Vec<i32>, num: i32) -> i32 {
    if let Some(index) = vector.iter().position(|&x| x == num){
        // println!("{} is located at index {}", num, index);

        let mut all_appearances_found = 0;
        let mut appearance_count = 0;

        while all_appearances_found == 0 {
            if index+1 == vector.len() {
                appearance_count = appearance_count + 1;
                all_appearances_found = 1;
            } else if vector[index + appearance_count] == num {
                appearance_count = appearance_count + 1;
            } else {
                all_appearances_found = 1;
            }
        }
        // println!("{}", appearance_count);
        return  appearance_count as i32;
    } else{
        // println!("Couldn't find index");
        return -1;
    }

}

fn similarity_score (vector_1: Vec<i32>, vector_2: Vec<i32>) -> i32{
    let mut similarity_score: i32 = 0;
    let mut appearance_count: i32;

    for i in 0..vector_1.len(){
        appearance_count = num_appearance(&vector_2, vector_1[i]);
        match appearance_count {
            _ if appearance_count >= 0 => {
                similarity_score = similarity_score + (appearance_count * vector_1[i]);                
            },
            _ => {
                // println!("Index not found")
            },
        }
               
        // println!("{}", appearance_count);
        // println!("{}", similarity_score);
    }
    // println!("{}", similarity_score);
    return similarity_score;

}
fn main() -> io::Result<()> {
    
    // ---------------------------------------------------------------------------------------------
    // ----------------------------------- PART ONE ------------------------------------------------
    // ---------------------------------------------------------------------------------------------
    // let file = "src/day_1_input_file.txt";
    let file = "src/test_input_file.txt";
    
   // Obtain columns
    let (column_1, column_2) = obtain_vectors_from_file(file)?;

    // Sort columns
    let column_1_sorted = sort_vec(column_1);
    let column_2_sorted = sort_vec(column_2);
    

    // Calculate distance between the two columns
    let total_distance = calc_distance(column_1_sorted, column_2_sorted);


    // ---------------------------------------------------------------------------------------------
    // ----------------------------------- PART TWO ------------------------------------------------
    // ---------------------------------------------------------------------------------------------
    
    // let file = "src/day_1_input_file.txt";
    let file = "src/day_1_input_file.txt";
    
   // Obtain columns
    let (column_1, column_2) = obtain_vectors_from_file(file)?;
    // println!("{:?}", column_1);
    // println!("{:?}", column_2);
    // println!("");

    // Sort columns
    let column_1_sorted = sort_vec(column_1);
    let column_2_sorted = sort_vec(column_2);    
    // println!("{:?}", column_1_sorted);
    // println!("{:?}", column_2_sorted);

    // num_appearance(&column_1_sorted, 3);
    println!("The similarity score is: {}", similarity_score(column_1_sorted, column_2_sorted));
    // Final answer is: 22 962 826

    Ok(())
}
