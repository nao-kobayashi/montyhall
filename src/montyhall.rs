use rand::Rng;
use rand::prelude::*;
use std::collections::HashSet;

pub struct MontyHall {
    try_count: i32,
    win_count: i32,
    random: ThreadRng,
}

impl MontyHall {
    //コンストラクタは無いので
    //下記が生成のお作法
    //ちなみにデストラクタはDropというTraitを実装する。
    //(通常は勝手に解放されるので自分で実装する必要はない。)
    pub fn new() -> Self {
        MontyHall { 
            try_count: 0, 
            win_count: 0,
            random: rand::thread_rng(),
        }
    }

    //自分自身のメンバを書き換えるとか
    //ex. try_count += 1
    //メンバの関数が&mutを取るとき
    //ex. self.random.gen_range(1, 4);
    //関数の引数を&mutにする。（というかしないとコンパイルが通らない）
    pub fn play(&mut self) {
        let car_door_no = self.random.gen_range(1, 4);
    
        let mut selected_door_no = 0;
        while selected_door_no < 1 || selected_door_no > 3 {
            //Rustの標準出力は改行を送らないとflushされない。
            //自分でstdoutに送れば制御できるがめんどいのでしていない。
            println!("ドア番号を選んでください。(1/2/3):");

            let s = read_std_in();       
            if let Ok(num) = s.parse::<i32>() {
                //parse出来た時しか来ない。
                selected_door_no = num;
            }
        }

        //(1..4)は左閉右開区間 (1 <= n < 4)
        //for i in 0..10とかも同じ
        let mut door_nos = (1..4).map(|n| n).collect::<HashSet<i32>>();
        door_nos.remove(&car_door_no);
        door_nos.remove(&selected_door_no);

        let goat = self.random.gen_range(0, door_nos.len());
        let goat_door_no = door_nos.iter().map(|n| *n).collect::<Vec<i32>>()[goat];
        println!("{} は山羊です。開くドアを変えますか？ (Y/N):", goat_door_no);

        let ans = read_std_in();
        self.try_count += 1;
        if (ans.to_lowercase() == "y") ^ (selected_door_no == car_door_no) {
            self.win_count += 1;
            println!("おめでとう、車です。(これまでの勝率：{}/{})", self.win_count, self.try_count);
        } else {
            println!("ざんねん、山羊です。(これまでの勝率：{}/{})", self.win_count, self.try_count);
        }
    }
}

pub fn read_std_in() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.replace("\r\n", "")
}