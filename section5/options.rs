// In main we have student database having some student records
// Each student in the database is an instance of the Student struct
// sturct has two fields name and grade 
// We need to provide both the fields of name and grade
// It may happen that grade are not currently available or not finalized
// but current implementation does not allow the empty grade 
// Option enum is in rust preload so rust automatically import this
// we could use them without writing extra code
struct Student {
    name: String,
    grade: Option<u32>,
}

// funciton will search student in database and return it's grade
fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn main() {
    let student_db: Vec<Student> = vec![
        Student {
            name: String::from("alice"),
            grade: Some(65),
        },
        Student {
            name: String::from("bob"),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None
        }
    ];

    let student_name: String = String::from("bob");
    let student_grade: Option<u32> = get_grade(&student_name, &student_db);
    // match student_grade {
    //     Some(grade) => println!("Grade is {grade}"),
    //     None => {}
    // }

    if let Some(grade) = student_grade {
        println!("grade is {grade}");
    }
}

// enum Option<T> {
    // None,
    // Some(T),
// }

// Some variant that holds a generic value and None variant that holds the None value represent absence value
// Room for improvement
// Arm related to None seems unusual 
// when you are interested in handling just one variant while disregarding all others you can utilize the if let syntax
// if let syntax is more precise then the match syntax
// this should be used in scenarios in which we only care about single variant and want to ignore other variants 