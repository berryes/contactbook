#[derive(Debug)]




pub struct Person{
    id: String,
    firstname: String,
    lastname: String,
    birth: String,
    phone: String,
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

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use regex::Regex;

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
         while guy.firstname.trim().is_empty() { 
            println!("Invalid | First name: ");
            input(&mut guy.firstname); // mutating back to console input again cus given shit was bad
         }
         
        
         // LAST NAME
         println!("Lastname: ");
         input(&mut guy.lastname); // mutating temp var from console onput
          while guy.lastname.trim().is_empty() { //  is not empty
             println!("Invalid | Lastname: ");
             input(&mut guy.lastname); // mutating back cus given shit was bad
          }

          // BIRTH DAY
        println!("Birthday (DD/MM/YYYY): ");
        input(&mut guy.birth); // mutating temp var from console onput
        let regexus = Regex::new(r#"/^[0-9]{1,2}\/[0-9]{1,2}\/[0-9]{4}$/"#).unwrap(); // creating regex sample
         while !regexus.is_match(&guy.birth) && guy.birth.trim().is_empty() { // checking if given string fits for the regex sample and is not empty
            println!("Invalid | Birthday (DD/MM/YYYY): ");
            input(&mut guy.birth); // mutating back cus given shit was bad
         }


         // PHONE NUMBER
        println!("Phone number: ");
        input(&mut guy.phone); // mutating temp var from console onput
        let regexus = Regex::new(r#"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$"#).unwrap(); // creating regex sample
         while !regexus.is_match(&guy.phone) && guy.phone.trim().is_empty() { // checking if given string fits for the regex sample and is not empty
            println!("Invalid | Phone number : ");
            input(&mut guy.phone); // mutating back cus given shit was bad
         }


        return guy;
}


struct  Database; // database "class"

// traits (assigning functions)
use async_trait::async_trait;
#[async_trait]
trait connector { 
    async fn InitTables();
    async fn addPerson(guy:Person);
}

// implementing those functions into the database structure
#[async_trait]
impl connector for Database
    {    
       // connects to the database (sqlite file) 
    async fn InitTables(){
        let conn = SqliteConnection::connect("./database.sqlite").await;
        conn.expect("faul").execute(sqlx::query("
        CREATE TABLE people 
        (
        id text, 
        firstname text,
        lastname text,
        birth text,
        phone text,
        PRIMARY KEY(id)
        );
        ")).await; 
    }
    // adds person record to db
    async fn addPerson(guy:Person){
        let conn = SqliteConnection::connect("./database.sqlite").await; // connecting to db

        // SQL query for inserting data
        let querus = format!("
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
        guy.id,
        guy.firstname,
        guy.lastname,
        guy.birth,
        guy.phone
        );

        // executing script 
        conn.expect("faul").execute(sqlx::query(&querus)).await;
    }
} 

use std::path::Path; // fs
use sqlx::Connection; // idfk
use sqlx::sqlite::SqliteConnection; // connection handler
use sqlx::Executor; // for executing queries
use std::fs::File; // for creating database.sqlite

// why tf doesnt this bs lang have asnyc functions already | ISNT IT 8 YEARS OLD?!!
#[tokio::main]
async fn main(){

    // in case database file was not generated yet
    if !Path::new("database.sqlite").exists() { 
        File::create("database.sqlite");
    };

    // creating tables | or not if they already exist
    Database::InitTables().await;

    // select menu
    let selection: String = selector(); 

    print!("{}",selection);

    // i wasted 15 mintutes bc the fucking string contained the enter's unicode string or whatever (x\r\n) 
    match  selection.trim() {
        "1" =>{ // add person
                let person:Person = personCollector(); // collecting data about the person
                Database::addPerson(person).await;
                println!("Person added")
        }
        "2" =>{ // List people

        },
        "3" =>{ // search

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