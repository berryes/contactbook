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

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn personCollector() -> Person{
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        println!("{}",&rand_string);
        let mut guy:Person = Person { 
            id: rand_string, 
            firstname: String::new(), 
            lastname: String::new(), 
            birth: String::new(), 
            phone: String::new()
         };

        println!("First name:");
        std::io::stdin().read_line(&mut guy.firstname).unwrap();
         
        println!("Last name:");
        std::io::stdin().read_line(&mut guy.lastname).unwrap();

        println!("Birthday:");
        std::io::stdin().read_line(&mut guy.birth).unwrap();
        
        println!("Phone number:");
        std::io::stdin().read_line(&mut guy.phone).unwrap();

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
    async fn addPerson(guy:Person){
        let conn = SqliteConnection::connect("./database.sqlite").await;

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
        guy.id,
        guy.firstname,
        guy.lastname,
        guy.birth,
        guy.phone
        );
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

    // creating tables
    Database::InitTables().await;

    // contactbook  | Vector/array of the same types (PERSON)
    let mut people: Vec<Person> = Vec::<Person>::new();
    let selection: String = selector();    

     match selection.as_str() {
        "0" =>{ 

        }
        "1" =>{ // add
               let person:Person = personCollector();
               Database::addPerson(person).await;
        }
        "2" =>{ // list

        }
        "3" =>{ // Search

        }


        _ => return
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