#[derive(Debug)]

pub struct Person{
    id: String, // yDVaPGbSZA0AOe4Vr2elU6tNRLwqR9
    firstname: String, // Kiss
    lastname: String, // Adam
    birth: String, // 01/05/2001
    phone: String, // 06 55 114 783 
}


fn showPPL(ppl: Vec<Person>) {
    // pls dynamci sizing TODO  
    
        println!("Firstname        Lastname        Birthday        Phone");
        for p in ppl{
            println!("{}   {}   {}   {}",p.firstname,p.lastname,p.birth,p.phone)
          }
}

/// public function named selector which returns a string (-> String)
pub fn selector() -> String{
        // select menu TEMP
        let mut line:String = String::new();
        println!("\n [1] Add person \n [2] List people \n [3] Search");
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
}

pub fn input(changer: &mut String) {
    std::io::stdin().read_line(changer)
    .ok()
    .unwrap();
}

use rand::{thread_rng, Rng}; // used for generating random string for contact IDs
use rand::distributions::Alphanumeric; // used for converting random int to string
use unicode_segmentation::UnicodeSegmentation; // segmentor
use regex::Regex; // regex for validating inputs
use sqlite; // db controller


pub fn personCollector() -> Person{
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

        let mut guy:Person = Person { 
            id: rand_string, // assinging random string from rand_string var (gets destroyed cus its not borrowed) 
            firstname: String::new(), 
            lastname: String::new(), 
            birth: String::new(), 
            phone: String::new()
         };

        // FIRST NAME
        println!("First name:");
        input(&mut guy.firstname); // mutating  var from console onput
         while guy.firstname.trim().is_empty() || !(guy.firstname.graphemes(true).count() < 21) { 
            println!("Invalid input | First name: ");
            input(&mut guy.firstname); // mutating back to console input again cus given shit was bad
         }


         // LAST NAME
         // Rules: only letters, max 20 chars
         println!(" Lastname: ");
         input(&mut guy.lastname); // mutating temp var from console onput
          while guy.lastname.trim().is_empty() || !(guy.firstname.graphemes(true).count() < 21)  { //  asks for input again if string length is over 20 or empty
             println!("Invalid | Lastname: ");
             input(&mut guy.lastname); // mutating back cus given shit was bad
          }

          // BIRTH DAY
        println!("Birthday (DD/MM/YYYY): ");
        input(&mut guy.birth); // mutating temp var from console onput
/*         let regexus = Regex::new(r"(?:(?:31(\/|-|\.)(?:0?[13578]|1[02]))\1|(?:(?:29|30)(\/|-|\.)(?:0?[13-9]|1[0-2])\2))(?:(?:1[6-9]|[2-9]\d)?\d{2})$|^(?:29(\/|-|\.)0?2\3(?:(?:(?:1[6-9]|[2-9]\d)?(?:0[48]|[2468][048]|[13579][26])|(?:(?:16|[2468][048]|[3579][26])00))))$|^(?:0?[1-9]|1\d|2[0-8])(\/|-|\.)(?:(?:0?[1-9])|(?:1[0-2]))\4(?:(?:1[6-9]|[2-9]\d)?\d{2})").unwrap(); // creating regex sample
 */         while /* !regexus.is_match(&guy.birth) && */ guy.birth.trim().is_empty() { // checking if given string fits for the regex sample and is not empty
            println!("Invalid | Birthday (DD/MM/YYYY): ");
            input(&mut guy.birth); // mutating back cus given shit was bad
         }


         // PHONE NUMBER
        println!("Phone number: ");
        input(&mut guy.phone); // mutating temp var from console onput
/*         let regexus = Regex::new(r#"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$"#).unwrap(); // creating regex sample
 */         while /* !regexus.is_match(&guy.phone) && */ guy.phone.trim().is_empty() { // checking if given string fits for the regex sample and is not empty
            println!("Invalid | Phone number : ");
            input(&mut guy.phone); // mutating back cus given shit was bad
         }


        return guy;
}


struct  Database; // database "class"

trait connector { 
     fn InitTables();
     fn addPerson(guy:Person);
     fn getPeople(changer: &mut Vec<Person>);
     fn search(query: String, mutable: &mut Vec<Person>);
}




// implementing those functions into the database structure
impl connector for Database
    {    
       // connects to the database (sqlite file)  and creating tables 
     fn InitTables(){
        let connection = sqlite::open("./database.sqlite").unwrap();
        let query: String  = String::from("
        CREATE TABLE IF NOT EXISTS people 
        (
        id text, 
        firstname text,
        lastname text,
        birth text,
        phone text,
        PRIMARY KEY(id)
        );
        ");
        connection.execute(query).expect("Failed to create table")

    }

    // adds person record to db
     fn addPerson(guy:Person){

        // SQL query for inserting datas
        let querus:String = format!("
        INSERT INTO people (id,firstname,lastname,birth,phone)
        VALUES(
            '{}',
            '{}',
            '{}',
            '{}',
            '{}'
        );
        ",


        // replacing {} to data( ex: guy.firstname = Kiss)
        // also trimming it cus the fuckign string contains enters
        guy.id.trim(),
        guy.firstname.trim(),
        guy.lastname.trim(),
        guy.birth.trim(),
        guy.phone.trim()
        );
        
        let connection = sqlite::open("./database.sqlite").unwrap();
        connection.execute(querus);

    }

     fn getPeople(changer: &mut Vec<Person>){ // mutable var is given as a parameter of the func

        let connection = sqlite::open("./database.sqlite").unwrap();
        
        connection
            .iterate("SELECT * FROM people", |pairs| { // selecting all the records then ittarating thru it 
                
                let mut emberke:Person = Person {
                    id: String::new(),
                    firstname: String::new(),
                    lastname: String::new(),
                    birth: String::new(),
                    phone: String::new()
                };

                for &(name, value) in pairs.iter() {
                    // pls dont look it this is ugly | https://search.berryez.xyz/search?q=rust+assign+value+to+struct+dynamically
                    // for some reason i cant use a match case here.
                    // i will try to assing a value dynamicly at some point but i still cant figure it out  | it would be usefull for adding custom fields + easier to expand the program | like emberke[name] = value.unwrap().as_string()
                
                    if name == "id"{
                       emberke.id = value.unwrap().to_string();
                    }
                    else if name == "firstname" {
                        emberke.firstname = value.unwrap().to_string().replace("\r\n", "") // deleting enter or wtf that shit is;
                    }
                    else if name == "lastname" {
                        emberke.lastname = value.unwrap().to_string().replace("\r\n", "");
                    }
                    else if name == "birth" {
                        emberke.birth = value.unwrap().to_string().replace("\r\n", "");
                    }
                    else if name == "phone" {
                        emberke.phone = value.unwrap().to_string().replace("\r\n", "");
                    }
                } // end of FOR value

                changer.push(emberke); // mutating changer vector by pushing emberke
                
                true
            })
            .unwrap();

    }

    fn search(query: String, mutable: &mut Vec<Person>) {
        let connection = sqlite::open("./database.sqlite").unwrap();

        let mut DBquery: String = String::new();

        DBquery = format!("
        SELECT * FROM people 
        WHERE firstname LIKE '{}'
        OR  lastname LIKE '{}'
        OR  birth LIKE '{}'
        OR phone LIKE '{}'
        ",
        query.trim(),
        query.trim(),
        query.trim(),
        query.trim()
        );
            connection
            .iterate(DBquery, |pairs| {
                for &(name, value) in pairs.iter() {
                    let mut emberke = Person{
                        id:String::new(),
                        firstname:String::new(),
                        lastname:String::new(),
                        birth:String::new(),
                        phone:String::new()
                    };

                    if name == "id"{
                        emberke.id = value.unwrap().to_string();
                     }
                     else if name == "firstname" {
                         emberke.firstname = value.unwrap().to_string()
                     }
                     else if name == "lastname" {
                         emberke.lastname = value.unwrap().to_string();
                     }
                     else if name == "birth" {
                         emberke.birth = value.unwrap().to_string()
                     }
                     else if name == "phone" {
                         emberke.phone = value.unwrap().to_string();
                     }
                     mutable.push(emberke)
                }
                true
            })
            .unwrap();
            
    }
} 

use std::path::Path; // fs
use std::fs::File;


// where the magic happens
fn main(){

    // in case database file was not generated yet
    if !Path::new("database.sqlite").exists() { 
        File::create("database.sqlite");
    };

    // creating tables | or not if they already exist
    Database::InitTables();

    // select menu
    let selection: String = selector(); 


    // i wasted 15 mintutes bc the fucking string contained the enter's unicode string or whatever (x\r\n) 
    match  selection.trim() {
        
        "1" =>{ // add person
                let person:Person = personCollector(); // collecting data about the person
                Database::addPerson(person);
                println!("Person added")
        }

        "2" =>{ // List people
            let mut ppl: Vec<Person> = Vec::new();
            Database::getPeople(&mut ppl);
            showPPL(ppl);
        },

        "3" =>{ // search

            // Getting input
            println!("Search query:");
            let mut query: String = String::new();
            input(&mut query);
            
            // Declaring list of people and mutating the result into that
            let mut people: Vec<Person> = Vec::new();
            Database::search(query,&mut people);
            showPPL(people); // tabling people

        }
        _=> return
    }


   /*  println!("{:?} -> {}",people,selection); */
}

/*
 __________________________
  < I WANNA KILL MY SELF >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
*/