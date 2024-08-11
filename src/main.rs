use std::io::{self, stdout, Write};

#[derive(PartialEq)]
enum GameGenreError{
    InvalidGenre
}

impl  GameGenreError {
    fn show_error(&self){
        match self {
            GameGenreError::InvalidGenre => println!("Invalid game genre")
        }
    }
}
#[derive(PartialEq)]

enum GameGenre{
    Action,
    Strategy,
    Fps,
    Sports,
    Farming,
    Rpg,
    Simulation
}

impl GameGenre{
    fn to_string(&self) -> &str{
        match self{
            GameGenre::Action => "Action",
            GameGenre::Strategy => "Strategy",
            GameGenre::Fps => "FPS",
            GameGenre::Sports => "Sports",
            GameGenre::Farming => "Farming",
            GameGenre::Rpg => "RPG",
            GameGenre::Simulation => "Simulation"
        }
    }

    fn string_to_genre(genre:&str) -> Result<GameGenre, GameGenreError>{
        match genre.to_lowercase().trim() {
            "action" => Ok(GameGenre::Action),
            "strategy" => Ok(GameGenre::Strategy),
            "fps" => Ok(GameGenre::Fps),
            "sports" => Ok(GameGenre::Sports),
            "farming" => Ok(GameGenre::Farming),
            "rpg" => Ok(GameGenre::Rpg),
            "simulation" => Ok(GameGenre::Simulation),
            _ => Err(GameGenreError::InvalidGenre)
        }
    }

}

#[derive(PartialEq)]
struct Game{
    name:String,
    genre:GameGenre,
    price:f32,
    stock:u32
}

impl Game{
    fn show_game(&self){
        println!("Game ");
        println!("Name: {}",self.name);
        println!("Genre: {}",self.genre.to_string());
        println!("Price: {}",self.price);
        println!("Stock: {}", self.stock);
    }


    fn create_game(name:&str, genre:GameGenre, price:f32, stock:u32)-> Game{
        Game {
            name: name.to_string(),
            genre,
            price,
            stock
        }
    }

}

enum GameError{
    ErrorInGenre(GameGenreError),
    InvalidStock,
    InvalidPrice,
    ErrorAskingValues(String)
}

impl GameError {
    fn show_error(&self){
        match self {
            GameError::InvalidStock => println!("Invalid number in stock"),
            GameError::InvalidPrice => println!("Invalid number in price"),
            GameError::ErrorInGenre(t) => {
                println!("Invalid genre\n ");
                t.show_error();
            }
            GameError::ErrorAskingValues(string) => println!("Error asking for the values in: {string}")
        }
    }
}
struct Inventory{
    games:Vec<Game> 
}

impl Inventory{
    fn add_game(&mut self,new_game:Game) {
        self.games.push(new_game);
    }

    //fn game_exist(&self, name:&str) -> bool{
        //self.games.
    //}

    fn remove_game(&mut self, game_name:String){
        self.games.retain(|x| x.name!=game_name);
    }

    fn list_games(&self){
        for game in self.games.iter(){
            game.show_game();
        }
    }

    fn find_game(&mut self, name:&str) -> Result<&mut Game, InventoryError>{

        for game in self.games.iter_mut(){
            if game.name == name{
                return Ok(game);
            }
        }
        Err(InventoryError::GameNotFound)
   }

   fn update_stock(&mut self, name:&str, stock_to_substract:u32) -> Result<(), InventoryError>{

        match self.find_game(name) {
            Err(t) =>{
                Err(t)
            }
            Ok(t) =>{ 
                if t.stock >= stock_to_substract {
                    t.stock -= stock_to_substract;
                    return Ok(())
                }
                Err(InventoryError::OutOfStock)
            }
            
        }
   }

}

enum InventoryError{
    GameNotFound,
    OutOfStock,
}

impl InventoryError {
    fn show_error(&self){
        match self {
            InventoryError::GameNotFound => println!("Game not found in inventory"),
            InventoryError::OutOfStock => println!("The game is out of stock"),
        }
    }
}

fn main() {
    let game1 = Game {
        name: String::from("Super Action Game"),
        genre: GameGenre::Action,
        price: 59.99,
        stock: 10,
    };

    let game2 = Game {
        name: String::from("Farmville Deluxe"),
        genre: GameGenre::Farming,
        price: 29.99,
        stock: 5,
    };

    let game3 = Game {
        name: String::from("Ultimate Strategy"),
        genre: GameGenre::Strategy,
        price: 49.99,
        stock: 8,
    };

    let game4 = Game {
        name: String::from("FPS Pro 2024"),
        genre: GameGenre::Fps,
        price: 69.99,
        stock: 3,
    };

    let game5 = Game {
        name: String::from("Soccer Star 2024"),
        genre: GameGenre::Sports,
        price: 39.99,
        stock: 12,
    };

    // Crea un inventario y añade los juegos de prueba
    let mut inventory = Inventory { games: Vec::new() };

    inventory.add_game(game1);
    inventory.add_game(game2);
    inventory.add_game(game3);
    inventory.add_game(game4);
    inventory.add_game(game5);

    menu(inventory);

}

fn menu(mut inventory:Inventory){
    loop {
        let game_to_mut:Game;
        println!("Inventory Manager 
                1. Add game 
                2. Remove game 
                3. List games
                4. Remove from stock
                5. Exit");

        let option_string:String;
        let  option:u8;
        match read_from_console("Your choice:") {
            Ok(t) => option_string = t.trim().to_string(),
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        }

        match option_string.parse::<u8>(){
            Ok(n) => option = n,
            Err(_) => {
                println!("Not a number value");
                continue;
            }
        }

        match option{
            1 =>{
                match ask_for_game(){
                    Ok(g) => game_to_mut = g,
                    Err(t) => {
                        t.show_error();
                        continue;
                    }
                }
                inventory.add_game(game_to_mut);
                println!("Done");
            }
            2 => {
                let name:String;
                match read_from_console("Name to remove: "){
                    Ok(s) => name = s,
                    Err(_) => {
                        println!("Failed to read the line");
                        continue;
                    }
                } 

                inventory.remove_game(name.trim().to_string());


            }
            3 => {
                inventory.list_games();
            }
            4 => {
                let name:String;
                let stock_to_remove_string:String;
                let stock_to_remove: u32;

                match read_from_console("Name of game: "){
                    Ok(t) => name = t.trim().to_string(),
                    Err(_) =>{
                        println!("Failed to read the line");
                        continue;
                    } 
                }

                match read_from_console("Stock to remove"){
                    Ok(s) => stock_to_remove_string = s,
                    Err(_) => {
                        println!("Failed to read the line");
                        continue;
                    }
                }

                match stock_to_remove_string.trim().parse::<u32>() {
                    Ok(n) => stock_to_remove = n,
                    Err(_) => {
                        println!("Not a number");
                        continue;
                    }
                }

                match inventory.update_stock(&name, stock_to_remove){
                    Ok(_) => println!("Done"),
                    Err(e) => e.show_error()

                }

            }
            5 => {
                break;
            }
            _ => {
                continue;
            }
        }



    }
}

fn read_from_console(message:&str) -> Result<String, io::Error>{
    print!("{message}");
    if let Err(e) =  stdout().flush() {
        return Err(e);
    }
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);
    print!("\n");
    match result {
        Ok(_) => Ok(input),
        Err(e) => Err(e),
    }
}

fn ask_for_game() -> Result<Game, GameError>{
    println!("Insert game info");
    let name:String;
    let genre_string:String;
    let price_string:String;
    let stock_string:String;
    let genre:GameGenre;

    match read_from_console("Name: "){
        Ok(n) => name= n,
        Err(_) => return Err(GameError::ErrorAskingValues("name".to_string()))
    }

    match read_from_console("Genre: "){
        Ok(n) => genre_string = n,
        Err(_) => return  Err(GameError::ErrorAskingValues("genre".to_string()))
    }

    match read_from_console("Price: "){
        Ok(n) => price_string = n,
        Err(_) => return Err(GameError::ErrorAskingValues("price".to_string()))
    }
    match read_from_console("Stock: "){
        Ok(n) => stock_string = n,
        Err(_) => return Err(GameError::ErrorAskingValues("stock".to_string()))
    }

    match GameGenre::string_to_genre(&genre_string){
        Ok(t) => genre = t,
        Err(e) => return Err(GameError::ErrorInGenre(e))
    }

    let price:f32;

    match price_string.trim().parse::<f32>(){
        Ok(n) => price = n,
        Err(_) => return Err(GameError::InvalidPrice)
    }

    let stock:u32;

    match stock_string.trim().parse::<u32>(){
        Ok(n) => stock = n,
        Err(_) => return Err(GameError::InvalidStock)
    }

    Ok(Game::create_game(&name.trim(), genre, price, stock))
    

}


