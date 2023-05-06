// my solution for exercism task https://exercism.org/tracks/rust/exercises/bowling
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct Roll {
    pins:u16,
    next:Option<Rc<RefCell<Roll>>>,
}
type RcRoll=Rc<RefCell<Roll>>;

impl Roll {
    fn new (pins:u16)->Self {
        Roll {
            pins,
            next:None
        }
    }
}

#[derive(Debug)]
enum Frame {
    Strike(RcRoll),
    Spare((RcRoll,RcRoll)),
    Open((RcRoll,RcRoll)),
}

impl Frame {
    fn calculate_score(&self)->u16 {
        match self {
            Frame::Strike(first_throw)=>{
                let first_throw=first_throw.borrow();
                let second_throw=first_throw.next.as_ref().unwrap().as_ref().borrow();
                let third_throw=second_throw.next.as_ref().unwrap().as_ref().borrow();
                return first_throw.pins+second_throw.pins+third_throw.pins
            }
            Frame::Spare((first_throw,second_throw))=>{
                let first_throw=first_throw.borrow();
                let second_throw=second_throw.borrow();
                let third_throw=second_throw.next.as_ref().unwrap().as_ref().borrow();
                return first_throw.pins+second_throw.pins+third_throw.pins
            },
            Frame::Open((first_throw,second_throw))=>{
                let first_throw=first_throw.borrow();
                let second_throw=second_throw.borrow();
                return first_throw.pins+second_throw.pins
            }
        }
    }
}

pub struct BowlingGame {
    new_frame:bool,
    last_roll:Option<Rc<RefCell<Roll>>>,
    frames:Vec<Frame>,
    required_frames:u16,
    pins_standing:u16
}

impl BowlingGame {
    pub fn new() -> Self {
       return BowlingGame { new_frame: true, last_roll: None, frames: Vec::new(),required_frames:10,pins_standing:10 }
    }

     fn chain_roll(&mut self, new_roll:Rc<RefCell<Roll>>,pins:u16){
        match self.last_roll.take() {
            Some(rc)=>{
                    let mut last_roll=rc.borrow_mut();
                    last_roll.next=Some(new_roll.clone());
                    self.last_roll=Some(new_roll.clone());
            },
            None=>{
                self.last_roll=Some(new_roll.clone());
            }
        }
    }

    fn get_last_roll(&mut self)->Option<RcRoll> {
        match self.last_roll.take(){
            Some(last_roll)=>{
                 self.last_roll=Some(last_roll.clone());
                 return Some(last_roll)
            },
            None=>return None
        }
    }

    fn handle_fitt_ball(&mut self,new_roll:Rc<RefCell<Roll>>){
        let pins: u16=new_roll.borrow().pins;
        let last_frame=self.frames.last().unwrap();
        match (last_frame,self.new_frame,pins<self.pins_standing) {
            (Frame::Spare(_),_,_)=>{
                let extra_roll=Rc::new(RefCell::new(Roll::new(0)));
                let new_frame=Frame::Open((new_roll,extra_roll));
                self.frames.push(new_frame);
            },
            (_,false,_)=>{
                let last_roll= self.get_last_roll().unwrap();
                let new_frame=Frame::Open((last_roll,new_roll));
                self.frames.push(new_frame);
            },
            (_,true,true)=>{

                self.new_frame=false;
                self.pins_standing-=pins;
            },
            (_,true,false)=>{
                self.new_frame=false;
            }
        }
    }

    fn handle_throw(&mut self, new_roll:Rc<RefCell<Roll>> ) {
        if self.frames.len()==10 {
            self.handle_fitt_ball(new_roll);
            return 
        }

        let pins=new_roll.borrow().pins;
        match (self.new_frame,pins<self.pins_standing){
            (false,true)=>{
               let last_roll= self.get_last_roll().unwrap();
               let new_frame=Frame::Open((last_roll,new_roll));
               self.frames.push(new_frame);
               self.new_frame=true;
               self.pins_standing=10;

            },
            (false,false)=>{
                let last_roll= self.get_last_roll().unwrap();
                let new_frame=Frame::Spare((last_roll,new_roll));
                self.frames.push(new_frame);
                self.new_frame=true;
                self.pins_standing=10;
            },
            (true,false)=>{
                let new_frame=Frame::Strike(new_roll);
                self.frames.push(new_frame);
            },
            (true,true)=>{
                self.pins_standing-=pins;
                self.new_frame=false;
            }
        }

        if self.frames.len()==10 {
            let last_frame=&self.frames[9];
            match last_frame {
                Frame::Open(_)=>{},
                _=>{self.required_frames=11}
            }
        }

    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len()==self.required_frames as usize{
            return Err(Error::GameComplete)
        }

        if self.pins_standing<pins {
            return Err(Error::NotEnoughPinsLeft)
        }

        let new_roll=Rc::new(RefCell::new(Roll::new(pins)));

        self.handle_throw(new_roll.clone());
        self.chain_roll(new_roll,pins);
        
        return Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len()!=self.required_frames as usize {
           return None
        }

        //we calculate only for 10 throws. 11th if present is fitball whose points are included in 10th frame. 
        let result=self.frames[0..10].iter().fold(0,|acc,frame|acc+frame.calculate_score());

        Some(result)
    }
}
