#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Entity{
    Ant,
    CAnt,
    Leaf,
    Empty
}



#[derive(Clone)]
pub struct Board{
    height:usize,
    width:usize,
    board:Vec<Vec<Entity>>
}

impl Board {
    pub fn new(height:i32,width:i32)->Self{
        let mut board:Vec<Vec<Entity>>=Vec::new();
        for _i in 0..height{
            let mut row:Vec<Entity>=Vec::new();
            for _j in 0..width{
                row.push(Entity::Empty)
            }
            board.push(row);
    
        }
        Self{height:height as usize,width:width as usize,board:board}
    }
    pub fn draw(&self){
        for i in 0..self.height{
            for j in 0..self.width{

                let icon= &self.board[i][j];
                match icon{
                    Entity::Ant=>print!("A"),
                    Entity::CAnt=>print!("C"),
                    Entity::Leaf=>print!("L"),
                    Entity::Empty=>print!("*"),
                }     
            }
            println!{};
        }
    }
    pub fn set(&mut self, h:usize,w:usize, entity:Entity){
        self.board[h][w]=entity;
    }
    pub fn get(&mut self, h:usize,w:usize)->&Entity{
        &self.board[h][w]
    }

}