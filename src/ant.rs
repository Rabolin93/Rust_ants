
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
    surroundings:SurroundingsStruct,

}

impl SAnt{
    fn new(ant_pos_w:usize,ant_pos_h:usize)->Self{
        Self{
            ant_pos_w,
            ant_pos_h,
            is_carrying:false,
            surroundings:SurroundingsStruct{
                above:Empty,
                below:Empty,
                to_left:Empty,
                to_right:Empty,
            }
        }
    }
    fn check_surroundings(&mut self,board:&mut Board){ //Dlaczego referencja do Board musi byc mutowalna??
        
        let mut up=self.ant_pos_h-1;
        let mut down=self.ant_pos_h+1;
        let mut left=self.ant_pos_w-1;
        let mut right=self.ant_pos_w+1;

        if self.ant_pos_h==0{
            up=board.height-1
        }
        if self.ant_pos_h==board.height-1{
            down=0
        }
        if self.ant_pos_w==0{
            left=board.width-1
        }
        if self.ant_pos_w==board.width-1{
            right=0
        }

        self.surroundings=SurroundingsStruct{
            above:*board.get(up,self.ant_pos_w),
            below:*board.get(down,self.ant_pos_w),
            to_right:*board.get(self.ant_pos_h,right),
            to_left:*board.get(self.ant_pos_h,left),
        }

    }
    
    fn ant_move(&mut self,board:&mut Board){
        self.check_surroundings(board);
        let mut target:Entity;




    }


}