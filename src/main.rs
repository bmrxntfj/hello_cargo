use std::fmt;
use std::mem;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::convert::{
    From,
    TryFrom,
    TryInto,
};

mod cypto;

#[derive(Debug)]
struct Person<'a>{
  name:&'a str,
  age:u16
}

impl Person<'_>{
    fn rnd<'a>()->Person<'a>{
        Person{name:"",age:0}
    }

    fn new<'a>(n:&'a str,a:u16)->Person<'a>{
        Person{name:n,age:a}
    }

    fn play(&self)->u32{
        println!("play lonely.so hard!");
        0
    }

    fn playwith(&self,other:Person<'_>)->u32{
        println!("{} play with {},so happy!",self.name,other.name);
        1
    }
}

struct Order<'a>(i64,i32,&'a str);

impl fmt::Display for Order<'_>{
  fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
    write!(f,"id:{},no:{},address:{}",self.0,self.1,self.2)
  }
}

#[derive(Debug)]
struct Number{
    value:i32,
}

impl From<i32> for Number{
    fn from(item:i32)->Self{
        Number{value:item}
    }
}

#[derive(Debug, PartialEq)]
struct PostiveNumber{
    value:i32
}

impl TryFrom<i32> for PostiveNumber {
    type Error = ();

    fn try_from(item: i32) -> Result<Self, Self::Error> {
        if item > 0 {
            Ok(PostiveNumber{value:item})
        } else {
            Err(())
        }
    }
}

#[allow(dead_code)]
enum Color{
    Red,
    Green,
    Blue,
    RGB(u32,u32,u32),
}

#[derive(Debug)]
struct SingleGen<T>(T);


#[warn(unused_doc_comments)]
#[warn(unused_labels)]
fn main() {
    let unit=();
    println!("unit is {:?}",unit);
    let bmrxntfj=Person{name:"bmrxntfj",age:20};
    println!("Hello, world! {:#?}",bmrxntfj);
    bmrxntfj.play();
    let pertty=Person::new("pretty sister",18);
    println!("my name is {:#?}, I'm a hot pop singer.",bmrxntfj);
    pertty.playwith(bmrxntfj);

    let order=Order(1,201,"ww");
    println!("{}",order);
    
    let x=1u8;
    let y=2u32;
    let z=3f32;
    
    let i=1;
    let f=1.0;
    println!("x:{},y:{},z:{},i:{},f:{}",mem::size_of_val(&x),mem::size_of_val(&y),mem::size_of_val(&z),mem::size_of_val(&i),mem::size_of_val(&f));

    let num=Number::from(30);
    println!("number from {:?}",num);
    let i=5;
    let num:Number=i.into();
    println!("int into number {:?}",num);

    let i=3;
    let num:Result<PostiveNumber, ()>=i.try_into();
    println!("int try_into number {:?}",num.unwrap().value);

    let mut parsed_num:i32="53".parse().unwrap();
    let turbo_parsed_num="453".parse::<i32>().unwrap();
    println!("parsed num {:?}, turbo_parsed_num={:?}",parsed_num,turbo_parsed_num);

    let sum_result={parsed_num+turbo_parsed_num};
    println!("result of sum is {}",sum_result);

    'firtloop:loop{
        parsed_num+=1;
        if parsed_num%2==0 {
            println!("parsed number is even {}",parsed_num);
            break;
        }
        else{
            'secondloop:loop{
                println!("parsed number is odd {}",parsed_num);
                break 'firtloop;
            }
        }
    };

    while parsed_num>20{
        parsed_num-=1;
    }

    for x in 1..100{
        parsed_num+=x;
    }
    println!("for parsed_num is {}",parsed_num);

    match parsed_num{
        1|2|40|22=>println!("selected {}",parsed_num),
        80..=200=>println!("80..200 {}",parsed_num),
        _=>println!("{}",parsed_num),
    }

    let names=vec!["big","small","bingo"];
    for name in names.iter(){
        match name{
            &"big"=>println!("is {}?",name),
            _=>println!("original {}",name)
        }
    }

    for name in names.into_iter(){
        match name{
            "big"=>println!("is {} into_iter?",name),
            _=>println!("original {}",name),
        }
    }

    let mut mutnames=vec!["big","small","bingo"];
    for name in mutnames.iter_mut(){
        *name=match name{
            &mut "big"=>"changed",
            _=>"default",
        }
    }
    println!("mutnames: {:?}", mutnames);

    let truple=(0,1,4);
    let truplevalue= match truple{
        (0,y,z)=>y+z,
        (1,..)=>1,
        _=>0,
    };
    println!("truple {:?} matched value is {:?}",truple,truplevalue);

    let foo=((0,2,5),5);
    match foo{
        ((0,y,z),k)=>println!("y:{},z:{},k:{}",y,z,k),
        (..,k) if k<6=>println!("k:{}",k),
        _=>println!("no xyzk"),
    };

    let mut vari=6;
    while let varx=vari{
        if varx>100 {
            vari=varx+1;
            break;
        }
        else{
            vari=varx+2;
        }
    };
    println!("while let is {}",vari);

    playgame("hungry birds!");

    fn play(x:i32)->i32{x+1}
    //println!("play number {}", play(3));
    println!("play number {}", call_me(3,play));

    let play1=|x:i32|->i32 {x+2};
    println!("play1 number {}", play1(3));

    let play2=|x:i32| x+3;
    println!("play2 number {}", play2(3));

    let boxvalue=Box::new(3);
    let consume=||{
        println!("box value is {:?}",boxvalue);
        mem::drop(boxvalue);
        //println!("droped box value is {:?}",boxvalue);
    };
    //consume();
    execute_fn_once(consume);

    ///println!("box value is {:?}",boxvalue);

    let evenlist=vec![2,4,6,8,10];
    let is_odd=move|x|evenlist.contains(x);
    println!("{} is even {}",3,is_odd(&3));
    println!("{} is even {}",4,is_odd(&4));

    let array1=[1,3,5,6,7];
    println!("2 in array {}",array1.iter().any(|&x|x==2));
    println!("find {:?} in array",array1.iter().find(|&&x| x==2));
    println!("find index {:?} in array",array1.iter().position(|&x| x==3));

    let sum:i32=0;
    let evensum:i32=(0..100).map(|x|x*x).take_while(|&x|x<1000).filter(|&x|iseven(x)).fold(0,|sum,i|sum+i);
    println!("sum even number is {}",evensum);
    //noreturn();
    util::hello();
    //util::hello_mod_crate();
    //util::hello_self();
    util::hello_super();
    util::hello_crate();
    util::web::hello_crate_nested();

    println!("url {:?}",util::Url{host:"iplusus".to_string(),port:80,query:"?w=k".to_string()});

    cypto::test_mod();

    let text_generic=SingleGen::<String>("double bmrxntfj".to_string());
    println!("text_generic::<String> value is {:?}",text_generic);
    let text_generic=SingleGen("double bmrxntfj");
    println!("text_generic(\"\") value is {:?}",text_generic);

    let color=Color::RGB(200,122,40);
    match color{
        Color::Red=>println!("color is red"),
        Color::RGB(r,g,b)=>println!("r:{},g:{},b:{}",r,g,b),
        _=>println!("color is blue or green"),
    };

    let reff=&4;
    let reffvalue=match reff{
        &reff=>reff,
    };
    println!("reff:{},value:{}",*reff,reffvalue);

    let rnd_number=rand::thread_rng().gen_range(0,300);
    //println!("rand number is :{}",rnd_number);
    
    println!("please type a number");
    loop{
      let mut guess=String::new();
      io::stdin()
          .read_line(&mut guess)
          .expect("falied to read line");
          
      ///println!("guessed:{}",guess);
      
      //let guess:u32=guess.trim().parse().expect("it's not a number,please type a number");
      let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
      };
      
      match guess.cmp(&rnd_number){
        Ordering::Less=> println!("too small"),
        Ordering::Greater=>println!("too big"),
        Ordering::Equal=>{
          println!("bingo {}",rnd_number);
          break;
        },
      }
    }
    
    fn playgame(name:&str){
        println!("play game {}, let's go...",name);
    }

    #[warn(dead_code)]
    fn execute_fn_once<F>(f:F) where
        F:FnOnce(){
        println!("execute before...");
        f();
        println!("execute after...");
    }

    fn execute_fn<F>(f:F) where
        F:Fn(){
        println!("execute before...");
        f();
        println!("execute after...");
    }

    fn execute_fn_mut<F>(mut f:F) where
        F:FnMut(){
        println!("execute before...");
        f();
        println!("execute after...");
    }

    fn call_me<F:Fn(i32)->i32>(n:i32,f:F)->i32{
        let x=f(n);
        x
    }

    fn iseven(x:i32)-> bool{
        x%2==0
    }

    fn noreturn()->!{
        panic!("no value to return.");
    }
}

mod util{
    pub fn hello(){
        println!("hello mod.");
    }

    pub(in crate::util) fn hello_mod_crate(){
        println!("hello mod in crate.");
    }

    #[warn(dead_code)]
    pub(self) fn hello_self(){
        println!("hello self in mod.");
    }

    pub(super) fn hello_super(){
        println!("hello super in mod.");
    }

    #[warn(dead_code)]
    pub(crate) fn hello_crate(){
        println!("hello crate.");
    }

    pub mod web{
        pub(crate) fn hello_crate_nested(){
            println!("hello crate in nested.");
        }
    }

    #[derive(Debug)]
    pub struct Url{
        pub host:String,
        pub port:u16,
        pub query:String
    }
}
