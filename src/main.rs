use std::io;
use std::process::exit;
use crate::board::Board;
use crate::board::Entity;
use rand::Rng;
use crate::ant::SAnt;


pub mod board;
pub mod ant;

fn main() {

    let mut choice:i32;
    let mut config:Cfg= Cfg::default();

    
    print_menu();
    choice=choose_a_number().unwrap_or_default();

    loop{
        match choice{
            1=>{println!("Start the simulation");
                print_menu();
                simulation(&mut config);
                choice=0;
                },
            2=>{
                options(&mut config);
                print_menu();
                choice=0;
                },
            3=>{
                println!("Goodbye!");
                exit(0x1000);
                },
            _=>{
                println!("Choose from possible options- 1,2 or 3");
                choice=choose_a_number().unwrap_or_default();
                },
        }
    }

    
}



//Functions
fn choose_a_number()->Result<i32,Box<dyn std::error::Error>>{

    let mut choice=String::new();

    io::stdin().read_line(&mut choice)?;
    let choice:i32= choice.trim().parse()?;


    Ok(choice)
}



fn print_menu(){
    println!("Welcome to Ants Colony building simulation!");
    println!("Choose Your option from the list and press enter:");
    println!("1: Start the simulation");
    println!("2: Options");
    println!("3: Quit the program");

}

fn print_options_menu(){
    println!("Options- select a number:");
    println!("1: Set the board size");
    println!("2: Set the number of ants");
    println!("3: Set the number of turns");
    println!("4: Set automatic turns");
    println!("5: Show current configuration");
    println!("6: Back to menu");
    
}

fn options(config: &mut Cfg){
    let mut choice:i32;


    loop{
        print_options_menu();
        choice=choose_a_number().unwrap_or_default();
        match choice{
            1=> {
                println!("Enter board width:");
                let width:i32=match choose_a_number(){
                    Ok(number)=>number,
                    Err(_)=>{
                        println!("Wrong input, the value stays at default");
                    config.board_width
                        }
                    };
                    config.board_width=width;
                println!("Enter board height:");
                let height:i32=match choose_a_number(){
                    Ok(number)=>number,
                    Err(_)=>{
                        println!("Wrong input, the value stays at default");
                        config.board_height
                        }
                    };
                config.board_height=height;
                }
            2=> {
                println!("Enter the number of ants");
                let ants:i32=match choose_a_number(){
                    Ok(number)=>number,
                    Err(_)=>{
                        println!("Wrong input, the value stays at default");
                        config.number_of_ants
                        }
                    };
                config.number_of_ants=ants;
                }
            3=>{
                println!("Enter the number of turns");
                let turns:i32=match choose_a_number(){
                    Ok(number)=>number,
                    Err(_)=>{
                        println!("Wrong input, the value stays at default");
                        config.number_of_turns
                        }
                    };
                config.number_of_turns=turns;
            }
            4=>{

                println!("Select 1 for automatic turns or 2 for manual turns");
                let automatic:i32=match choose_a_number(){
                    Ok(number)=>{
                        if number==1{
                            number
                        }
                        else if number==2{
                            number
                        }
                        else{
                            println!("Wrong input, the value stays at default");
                            2
                        }
                        },
                    Err(_)=>{
                        println!("Wrong input, the value stays at default");
                        2
                        }
                    };
                if automatic==1{
                    config.automatic_turns=true;
                }
                else{
                    config.automatic_turns=false;
                    
                }
            }
            5=>{println!("{}",config)}
            6=>break,

            _=>{
                    println!("Choose from possible options");


            }
        }
    }
}

fn simulation(config: &mut Cfg){

    let mut rng=rand::thread_rng();
    let mut sim_board=Board::new(config.board_height,config.board_width);
    let mut number_of_ants:i32=0;
    let mut turn=0;
    let mut dud=String::new();
    let mut vector_of_ants:Vec<SAnt>=Vec::new();
    let mut number_of_leaves=((sim_board.height as f32*sim_board.width as f32)*0.15) as i32;
    

    while number_of_ants!=config.number_of_ants{
        let hpos=rng.gen_range(0..config.board_height);
        let wpos=rng.gen_range(0..config.board_width);
        if *sim_board.get(hpos as usize,wpos as usize)==Entity::Empty{
            sim_board.set(hpos as usize,wpos as usize,Entity::Ant);
            let new_ant:SAnt=SAnt::new(wpos as usize, hpos as usize);
            vector_of_ants.push(new_ant);
            number_of_ants+=1;
        }

    }

    while number_of_leaves!=0{
        let hpos=rng.gen_range(0..config.board_height);
        let wpos=rng.gen_range(0..config.board_width);
        if *sim_board.get(hpos as usize,wpos as usize)==Entity::Empty{
            sim_board.set(hpos as usize,wpos as usize,Entity::Leaf);
        }
        number_of_leaves-=1;

    }
    println!("Inital board:");
    sim_board.draw();
    println!("Press enter to continue");
    let _=io::stdin().read_line(&mut dud);
    
   


    while turn!=config.number_of_turns+1{
        if config.automatic_turns==true{
            //println!("Turn {}", turn);
            //Rysowanie w kazdej turze- wydluza dzialanie programu
            //sim_board.draw(); 
            for ant in &mut vector_of_ants{
                ant.ant_move(&mut sim_board);
            
            }
            turn+=1;
            continue
        }
        else{
            println!("Press enter to continue");
            let _=io::stdin().read_line(&mut dud);
        }
        println!("Turn {}", turn);
        sim_board.draw();
        for ant in &mut vector_of_ants{
            ant.ant_move(&mut sim_board);
            
        }
        

        turn+=1;
    }
    if config.automatic_turns==true{
        sim_board.draw();
    }
    
}



//Structs

struct Cfg{
    board_height:i32,
    board_width:i32,
    number_of_ants:i32,
    number_of_turns:i32,
    automatic_turns:bool,

}
impl Default for Cfg{
    fn default() -> Self {
        Cfg{
        board_height:50,
        board_width:50,
        number_of_ants:10,
        number_of_turns:100000,
        automatic_turns:true,
        }
    }
}
impl std::fmt::Display for Cfg{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Configuration: height {}, width {}, ants {}, turns {}, automatic {}",self.board_height, self.board_width, self.number_of_ants, self.number_of_turns, self.automatic_turns)
    }
}


