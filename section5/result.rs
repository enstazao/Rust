// This option enum code does not explicitly tell about student exist in database or not
// None is interpreted as the absence of the grade 
// checking for the student whose record is not with us should be an error
// if student_name exist then check for the grade   

struct Student {
    name: String,
    grade: Option<u32>,
}

// // funciton will search student in database and return it's grade
// fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
//     for student in student_db {
//         if student.name == *student_name {
//             return student.grade;
//         }
//     }
//     None  // not reachable because this code is for the student that exist 
// }

// fn check_student(student_name: &String, student_db: &vec<Student>) -> Result<(), String> {
//     // Option enum is used to have None value 
//     // Result enum to have result or an error
//     // enum Result<T, E> {
//     //     OK(T),
//     //     Err(E)
//     // }

//         for student in student_db {
//             if student.name == *student_name {
//                 return Ok(());
//             }
//         }
//         Err(String::from("Student not found"));

// }


fn check_student_get_grade(student_name: &String, student_db: &vec<Student>) -> Result<Option<u32>, String> {
    // Option enum is used to have None value 
    // Result enum to have result or an error
    // enum Result<T, E> {
    //     OK(T),
    //     Err(E)
    // }

        for student in student_db {
            if student.name == *student_name {
                return Ok(student.grade);
            }
        }
        Err(String::from("Student not found"))

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

    let student_name: String = String::from("hello");
    let student_status: Result<Option<u32>, String> = check_student_get_grade(&student_name, &student_db);
    match student_status {
        Ok(option_grade) => {
            if let Some(grade: u32) = option_grade {
                println!("grade is {grade}");
            }
        },
        Err(error_msg: String) => println!("{error_msg}"),
    }
    // let student_grade: Option<u32> = get_grade(&student_name, &student_db);
    // match student_grade {
    //     Some(grade) => println!("Grade is {grade}"),
    //     None => {}
    // }
}

// If want to present presence or absence of any value use the option 
// if you want to represent success or failure of some operation use the result enum