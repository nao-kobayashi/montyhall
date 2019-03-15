//外部crateの宣言はmain.rs or lib.rsで行う。
extern crate rand;

//別ファイルの関数とか構造体を使うときは下記の指定
pub mod montyhall;
use crate::montyhall::{ MontyHall, read_std_in };

fn main() {
    println!("３つのドアのどれかひとつに車が，残りはは山羊が隠れています。");
    println!("車が隠れていまるドアを当ててください。");

    let mut i = 1;
    let mut monty_hall = MontyHall::new();
    loop {
        monty_hall.play();
        if i % 10 == 0 {
            println!("中止しますか？ (Y/N):");
            let s = read_std_in();
            if s.to_lowercase() == "y" {
                break;
            }
        }
        i += 1;
    }
}



#[cfg(test)]
mod test {
    //本当は下記でテストするモジュールをインポート指定する。
    //use super::*;

    #[test]
    fn test1() {
        //monthhall.rsのXORの判定があってるか
        let mut ans = "y";
        let mut door1 = 1;
        let mut door2 = 2;
        assert_eq!(true, (ans=="y") ^ (door1 == door2));

        ans = "y";
        door1 = 1;
        door2 = 1;
        assert_eq!(false, (ans=="y") ^ (door1 == door2));

        ans = "n";
        door1 = 1;
        door2 = 1;
        assert_eq!(true, (ans=="y") ^ (door1 == door2));

        ans = "n";
        door1 = 1;
        door2 = 2;
        assert_eq!(false, (ans=="y") ^ (door1 == door2));

    }
}
