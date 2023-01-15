

// enum GenderCato {
//     Male,
//     Female,
//     Trans
// }

// struct Person {
//     name:String,
//     email:String,
//     gender:GenderCato
// }

// fn main() {
//     let per1 = Person{
//         name:String::from("Pankaj"),
//         email:String::from("kjhkjh@gmail.com"),
//         gender:GenderCato::Male
//     }
// }


//----------------------------------
// We will match the enum values

enum CarType {
    Hatchback,
    Sedan,
    SUV
}

fn size_of_car(car:CarType) {
    match car {
        CarType::Hatchback => {
            println!("Small sized car");
        },
        CarType::Sedan => {
            println!("Medium sized car");
        },
        CarType::SUV => {
            println!("Larged sized car");
        },

    }
}

fn main() {
    size_of_car(CarType::SUV);
    size_of_car(CarType::Hatchback);
    size_of_car(CarType::Sedan);

    person_data();

}

// Match $ enum with data types

enum GenderCato {
    Name(String),
    User_ID(i32)
}

fn person_data() {
    let p1 = GenderCato::Name(String::from("Freaky coder"));
    let p2 = GenderCato::User_ID(100);

    match p1 {
        GenderCato::Name(val) => {
            println!("{}",val);
        }

        GenderCato::User_ID(val) => {
            println!("{}",val);
        }


    }
}