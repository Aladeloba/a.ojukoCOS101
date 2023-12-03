use std::io;

fn office_administrator(experience_years:i64,role:&mut Vec<String>,level:&mut Vec<String>) {

    let admin_roles: Vec<&str> = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let admin_level: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];


    if experience_years == 0 && experience_years <= 2 {
        role.push((admin_roles[0]).to_string());
        level.push((admin_level[0]).to_string());
    }
    else if experience_years > 2 && experience_years <= 5 {
        role.push((admin_roles[1]).to_string());
        level.push((admin_level[1]).to_string());
    } 
    else if experience_years > 5 && experience_years <= 8 {
        role.push((admin_roles[2]).to_string());
        level.push((admin_level[2]).to_string());
    }
    else if experience_years > 8 && experience_years <= 10 {
        role.push((admin_roles[3]).to_string());
        level.push((admin_level[3]).to_string());
    }
    else if experience_years > 10 && experience_years <= 13 {
        role.push((admin_roles[4]).to_string());
        level.push((admin_level[4]).to_string());
    }
    else if experience_years > 13 {
        role.push((admin_roles[5]).to_string());
        level.push((admin_level[5]).to_string());
    }

}

fn academic(experience_years:i64,role:&mut Vec<String>,level:&mut Vec<String>) {


    let academic_roles: Vec<&str> = vec!["n/a","Research Assistant","PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let academic_level: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    if experience_years == 0 && experience_years <= 2 {
        role.push((academic_roles[0]).to_string());
        level.push((academic_level[0]).to_string());
    }
    else if experience_years > 2 && experience_years <= 5 {
        role.push((academic_roles[1]).to_string());
        level.push((academic_level[1]).to_string());
    } 
    else if experience_years > 5 && experience_years <= 8 {
        role.push((academic_roles[2]).to_string());
        level.push((academic_level[2]).to_string());
    }
    else if experience_years > 8 && experience_years <= 10 {
        role.push((academic_roles[3]).to_string());
        level.push((academic_level[3]).to_string());
    }
    else if experience_years > 10 && experience_years <= 13 {
        role.push((academic_roles[4]).to_string());
        level.push((academic_level[4]).to_string());
    }
    else if experience_years > 13 {
        role.push((academic_roles[5]).to_string());
        level.push((academic_level[5]).to_string());
    }   

}

fn lawyer(experience_years:i64,role:&mut Vec<String>,level:&mut Vec<String>) {


    let lawyer_roles:Vec<&str> = vec!["Paralegal","Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let lawyer_level: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    if experience_years == 0 && experience_years <= 2 {
        role.push((lawyer_roles[0]).to_string());
        level.push((lawyer_level[0]).to_string());
    }
    else if experience_years > 2 && experience_years <= 5 {
        role.push((lawyer_roles[1]).to_string());
        level.push((lawyer_level[1]).to_string());
    } 
    else if experience_years > 5 && experience_years <= 8 {
        role.push((lawyer_roles[2]).to_string());
        level.push((lawyer_level[2]).to_string());
    }
    else if experience_years > 8 && experience_years <= 10 {
        role.push((lawyer_roles[3]).to_string());
        level.push((lawyer_level[3]).to_string());
    }
    else if experience_years > 10 && experience_years <= 13 {
        role.push((lawyer_roles[4]).to_string());
        level.push((lawyer_level[4]).to_string());
    }
    else if experience_years > 13 {
        role.push((lawyer_roles[5]).to_string());
        level.push((lawyer_level[5]).to_string());
    }
}

fn teacher(experience_years:i64,role:&mut Vec<String>,level:&mut Vec<String>) {


    let teacher_roles:Vec<&str> = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    let teacher_level: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    if experience_years == 0 && experience_years <= 2 {
        role.push((teacher_roles[0]).to_string());
        level.push((teacher_level[0]).to_string());
    }
    else if experience_years > 2 && experience_years <= 5 {
        role.push((teacher_roles[1]).to_string());
        level.push((teacher_level[1]).to_string());
    } 
    else if experience_years > 5 && experience_years <= 8 {
        role.push((teacher_roles[2]).to_string());
        level.push((teacher_level[2]).to_string());
    }
    else if experience_years > 8 && experience_years <= 10 {
        role.push((teacher_roles[3]).to_string());
        level.push((teacher_level[3]).to_string());
    }
    else if experience_years > 10 && experience_years <= 13 {
        role.push((teacher_roles[4]).to_string());
        level.push((teacher_level[4]).to_string());
    }
    else if experience_years > 13 {
        role.push((teacher_roles[5]).to_string());
        level.push((teacher_level[5]).to_string());
    }
}




fn main() {
    let mut role: Vec<String> = Vec::new();
    let mut level : Vec<String> = Vec::new();

    println!("Welcome to the Public Service Validation System
            \nPlease enter your name");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Hello, {}
            \nPlease enter your Public Service Office", name);

    let mut office_type = String::new();
    io::stdin().read_line(&mut office_type).expect("Failed to read input");
    let office_type = office_type.trim().to_lowercase();

    println!("Please input years of experience");
    let mut experience_years = String::new();
    io::stdin().read_line(&mut experience_years).expect("Failed to read input");
    let experience_years:i64 = experience_years.trim().parse().expect("Invalid Input");

    if office_type == "office administrator" {
        office_administrator(experience_years,&mut role,&mut level);
    }
    else if office_type == "academic" {
        academic(experience_years,&mut role,&mut level);
    }

    else if office_type == "lawyer" {
        lawyer(experience_years,&mut role,&mut level);
    }
    else if office_type == "teacher" {
        teacher(experience_years,&mut role,&mut level);
    }


    println!("{}, you are a {:?}, and your staff level is {:?}", name,role,level);

}
