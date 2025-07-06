use std::{fs::File, io::{self, BufRead, BufReader}};

#[derive(Debug)]
#[derive(PartialEq)]
enum Safety {
    Safe, 
    Unsafe,
}

fn main() { 
    
    let filename = "src/day_2_input_file.txt";
    println!("The number of safe reports is: {}", count_safe_reports(filename).unwrap());  
}


fn check_safety(report_1: &Vec<i32>) -> Safety {
    if report_1[0] < report_1[1]{
        for index in 1..report_1.len(){
            let diff = (report_1[index-1] as i32 - report_1[index] as i32).abs();
            if report_1[index-1] > report_1[index] || diff < 1 || diff > 3{
                // println!("Report is unsafe!");
                return Safety::Unsafe;
            }
        }
    } else if report_1[0] > report_1[1] {
        for index in 1..report_1.len(){
            let diff = (report_1[index-1] as i32 - report_1[index] as i32).abs();
            if report_1[index-1] < report_1[index] || diff < 1 || diff > 3{
                // println!("Report is unsafe!");
                return Safety::Unsafe;
            }
        }
    } else if report_1[0] == report_1[1] {
        // println!("Report is unsafe!");
        return Safety::Unsafe;
    }
    return Safety::Safe;
}

fn convert_line_to_report (line: String) -> Vec<i32>{
    let mut previous_space: usize = 0;  
    let mut report: Vec<i32> = Vec::new();  

    for index in 0..line.len(){        
        let mut number: &str;
        if line.chars().nth(index).unwrap() == ' '{
            
            if report.is_empty(){
                number = &line[0..index];
            } else {
                number = &line[previous_space+1..index];
            }
            
            report.push(number.parse().unwrap());
            previous_space = index;
        }

        if index == line.len()-1 {
            number = &line[previous_space+1..];
            report.push(number.parse().unwrap());
        }
    }
    return report;    
}

fn count_safe_reports (filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);   
    let mut total_safe_reports = 0; 
    
    // Cycle through all reports (all the rows in the text file)
    for line in reader.lines(){
        let line = line?;    

        // Obtain report vector of type i32 from string
        let report: Vec<i32>;
        report = convert_line_to_report(line);
        println!("{:?}", report);

        // Problem Dampener Check
        if check_safety(&report) == Safety::Unsafe {
            let mut safety_count = 0;
            let mut dummy_report: Vec<i32>;
            for num in 0..report.len(){
                dummy_report = report.clone();
                dummy_report.remove(num);
                if check_safety(&dummy_report) == Safety::Safe{
                    safety_count += 1;
                    println!("{:?}", dummy_report);
                }                
            }    

            if safety_count >= 1 {
                total_safe_reports += 1;
            }        
        } else if check_safety(&report) == Safety::Safe {
            total_safe_reports += 1;
        }              
    }

    Ok(total_safe_reports)
}