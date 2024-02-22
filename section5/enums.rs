// // -------------------------------
// //      Enums
// // -------------------------------


// // to define the enum we use the enum keyword followed by the enum name
// // difference between enums and struct is that 
// // for the enum we define the type for the values
// // but for struct we donot do that
// enum WeekDay {
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday, 
//     Friday,
//     Saturday, 
//     Sunday,
// }
// fn main() {
//     // Consider a situation where we are required to define a mutable variable containing the day information 
//     let mut day: String = "Saturday".to_string();
//     // it's not ideal 

//     let week_day: Vec<String> = vec![
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thursday".to_string(),
//         "Friday".to_string(),
//         "Saturday".to_string(),
//         "Sunday".to_string()
//     ];
//     // now day can set to a specific day using indexes
//     day = week_day[1].clone();
//     // vectors does not allow the partial move of the items

//     let day: WeekDay = WeekDay::Saturday;


//     // This requires memorization of the index values and it's correspondence with day 
//     // a better solution is to use the enums 
//     // enums allow you to define a type by enumerating it's variants 
    
// }


enum TravelType {
    Car(f32),
    Train(f32),
    Aeroplance(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Aeroplance(miles) => miles * 5.0,
        };
        allowance
    }
}

fn main() {
    // consider a company where the company organized an event and where many participants participated 
    // some of them attended the event by coming through car while some using train and some using air travel 
    // the company decided to give travel allowance to participants 
    // to give allowance the company first look at the travel type 
    // based on travel type specific amount will be paid to participants
    // Like structs the functionality defined on the enums can be grouped together using the impl keyword
    // enums are really powerful and we can have data associated with them 
    // variants can hold data with them and we can store miles information in them 

    let participant = TravelType::Car(60.0);

    println!(
        "Allowance of participant is: {}",
        participant.travel_allowance()
    );

}