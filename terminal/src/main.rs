#[derive(Debug)]

pub struct Person{
    id: u128,
    name: String,
    age: u128,
}

/// public function named selector which returns a string (-> String)
pub fn selector() -> String{
        // select menu TEMP
        let mut line:String = String::new();
        println!("\n [1] Add person \n [2] List people \n [3] Search");
        std::io::stdin().read_line(&mut line).unwrap();
        return line;
}

use sqlite::State;
struct  Database; // database "class"

trait Connection { 
    fn init();
    fn listPeople();
    fn generateRandom();
}



impl Connection for Database{
    fn init(){
        let connection = sqlite::open("./database.sqlite").unwrap();
        let tableQuery = "CREATE TABLE IF NOT EXISTS people (name TEXT, number INTEGER)";
        connection.execute(tableQuery).unwrap();
    }
    fn generateRandom() {
        let connection = sqlite::open("./database.sqlite").unwrap();

        let people = "INSERT INTO people VALUES ('Alice', 42);
        INSERT INTO people VALUES ('Bob', 69);";
        connection.execute(people).unwrap();
    }

    fn listPeople(){
        let connection = sqlite::open("./database.sqlite").unwrap();
        let query: &str = "SELECT * FROM people";
        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, 50)).unwrap();

        while let Ok(State::Row) = statement.next() {
            println!("name = {}", statement.read::<String, _>("name").unwrap());
            println!("age = {}", statement.read::<i64, _>("number").unwrap());
        }
    }
}

fn main(){

    // contactbook  | Vector/array of the same types (PERSON)
    let mut people: Vec<Person> = Vec::<Person>::new();
    
    let selection: String = selector();

    let user:Person = Person {
        id: 3213123,
        name: String::from("teszember"),
        age: 3
    };
        // database initialize
        Database::init();
        Database::generateRandom(); // generating entries
        Database::listPeople();

    people.push(user);         
   /*  println!("{:?} -> {}",people,selection); */
}