
use crate::Entity;
use crate::Entity::*;
use crate::Board;
use rand::Rng;


pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}
impl Direction{
    fn random_direction()->Direction{
        let mut rng=rand::thread_rng();
        match rng.gen_range(0..4){
            0=>Direction::Up,
            1=>Direction::Down,
            2=>Direction::Left,
            3=>Direction::Right,
            _=>unreachable!(),
        }
    }
}

pub struct SurroundingsStruct{
    above:Entity,
    below:Entity,
    to_left:Entity,
    to_right:Entity,
}

pub struct SAnt{
    ant_pos_w:usize,
    ant_pos_h:usize,
    is_carrying:bool,
    carry_block:usize,
    surroundings:SurroundingsStruct,

}

impl SAnt{
    pub fn new(ant_pos_w:usize,ant_pos_h:usize)->Self{
        Self{
            ant_pos_w,
            ant_pos_h,
            is_carrying:false,
            carry_block:0,
            surroundings:SurroundingsStruct{
                above:Empty,
                below:Empty,
                to_left:Empty,
                to_right:Empty,
            }
        }
    }
    fn check_surroundings(&mut self,board:&mut Board){ //Dlaczego referencja do Board musi byc mutowalna??
        
        let up;
        let down;
        let left;
        let right;

        if self.ant_pos_h==0{
            up=board.height-1
        }
        else{
            up=self.ant_pos_h-1
        }
        if self.ant_pos_h==board.height-1{
            down=0
        }
        else{
            down=self.ant_pos_h+1
        }
        if self.ant_pos_w==0{
            left=board.width-1
        }
        else{
            left=self.ant_pos_w-1
        }
        if self.ant_pos_w==board.width-1{
            right=0
        }
        else{
            right=self.ant_pos_w+1
        }

        self.surroundings=SurroundingsStruct{
            above:*board.get(up,self.ant_pos_w),
            below:*board.get(down,self.ant_pos_w),
            to_right:*board.get(self.ant_pos_h,right),
            to_left:*board.get(self.ant_pos_h,left),
        }

    }

    pub fn ant_move(&mut self,board:&mut Board){
        self.check_surroundings(board);
        let mut tries=0;

        while tries<4{
            let direction=Direction::random_direction();
            match direction{
                Direction::Up=>{
                    if self.surroundings.above==Ant{
                        tries+=1;
                        continue;
                    }
                    else{
                        if self.surroundings.above==Leaf && self.is_carrying{
                            self.drop_leaf(board);
                        }
                        self.change_position(board, direction);
                        break;
                    }
                }
                Direction::Down=>{
                    if self.surroundings.below==Ant{
                        tries+=1;
                        continue;
                    }
                    else{
                        if self.surroundings.below==Leaf && self.is_carrying{
                            self.drop_leaf(board);
                        }
                        self.change_position(board, direction);
                        break;
                    }
                }
                Direction::Left=>{
                    if self.surroundings.to_left==Ant{
                        tries+=1;
                        continue;
                    }
                    else{
                        if self.surroundings.to_left==Leaf && self.is_carrying{
                            self.drop_leaf(board);
                        }
                        self.change_position(board, direction);
                        break;
                    }
                }
                Direction::Right=>{
                    if self.surroundings.to_right==Ant{
                        tries+=1;
                        continue;
                    }
                    else{
                        if self.surroundings.to_right==Leaf && self.is_carrying{
                            self.drop_leaf(board);
                        }
                        self.change_position(board, direction);
                        break;
                    }
                }
            }
        }
        if self.carry_block!=0{
            self.carry_block-=1;
        }
        






    }

    fn change_position(&mut self, board:&mut Board,direction:Direction){
        if *board.get(self.ant_pos_h,self.ant_pos_w)!=Leaf{
            board.set(self.ant_pos_h, self.ant_pos_w , Empty); //Zeruje obecna pozycje na ktorej jest mrowka
        }
        match direction{
            Direction::Up=>{
                if self.ant_pos_h!=0{
                    self.ant_pos_h-=1;
                }
                else{
                    self.ant_pos_h=board.height-1;
                }
                self.pick_leaf(board);
                if *board.get(self.ant_pos_h,self.ant_pos_w)!=Leaf{
                    board.set(self.ant_pos_h,self.ant_pos_w,Ant);
                }
            }
            Direction::Down=>{
                if self.ant_pos_h!=board.height-1{
                    self.ant_pos_h+=1;
                }
                else{
                    self.ant_pos_h=0;
                }
                self.pick_leaf(board);
                if *board.get(self.ant_pos_h,self.ant_pos_w)!=Leaf{
                    board.set(self.ant_pos_h,self.ant_pos_w,Ant);
                }
            }
            Direction::Left=>{
                if self.ant_pos_w!=0{
                    self.ant_pos_w-=1;
                }
                else{
                    self.ant_pos_w=board.width-1;
                }
                self.pick_leaf(board);
                if *board.get(self.ant_pos_h,self.ant_pos_w)!=Leaf{
                    board.set(self.ant_pos_h,self.ant_pos_w,Ant);
                }
            }
            Direction::Right=>{
                if self.ant_pos_w!=board.width-1{
                    self.ant_pos_w+=1;
                }
                else{
                    self.ant_pos_w=0;
                }
                self.pick_leaf(board);
                if *board.get(self.ant_pos_h,self.ant_pos_w)!=Leaf{
                    board.set(self.ant_pos_h,self.ant_pos_w,Ant);
                }
            }
        }

        

    }

    fn pick_leaf(&mut self,board:&mut Board){
        if *board.get(self.ant_pos_h,self.ant_pos_w)==Leaf && self.is_carrying==false && self.carry_block==0{
            board.set(self.ant_pos_h,self.ant_pos_w,Ant);
            self.is_carrying=true;
            //println!("picked up a leaf");
        }

    }

    fn drop_leaf(&mut self,board:&mut Board){
        if *board.get(self.ant_pos_h,self.ant_pos_w)==Ant && self.is_carrying==true{
            self.is_carrying=false;
            self.carry_block=5;
            board.set(self.ant_pos_h,self.ant_pos_w,Leaf);
            //println!("dropped a leaf")
        }
        
    }
}