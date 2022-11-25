export class Book {
    constructor(book){
        this.created = book.created ?? Date.now()
        this.hostdata = book.hostdata 
        this.version = book.version ?? null
        this.data = book.data ?? {}
    }
    addPerson(person){
        if(!this.data.people){this.data.people = []}
        this.data.people.push(person)
        
/*         let id = uuidv4();
        while(this.data.people[id]){ id = uuidv4()}
        this.data.people[id] = person */
    }
    
}
