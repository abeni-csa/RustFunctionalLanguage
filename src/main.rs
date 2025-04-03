use examle::call_me;
use std::thread;

mod examle;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_prefernce: Option<ShirtColor>) -> ShirtColor {
        user_prefernce.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        println!("blue [{num_blue}] red [{num_red}]");
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
        ],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with prefernce {:?} get {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with prefernce {:?} get {:?}",
        user_pref2, giveaway2
    );
    print!("\n\n======================example on closure ==================================\n\n");
    call_me();
    print!(
        "\n\n====================== end example on closure ==================================\n\n"
    );
    let list = vec![1, 2, 3, 4];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread:{list:?}"))
        .join()
        .unwrap();
    let mut list_rect = [
        Rectangle {
            width: 10,
            hight: 1,
        },
        Rectangle {
            width: 3,
            hight: 15,
        },
        Rectangle {
            width: 7,
            hight: 12,
        },
    ];

    list_rect.sort_by_key(|r| r.hight);
    println!("{list_rect:#?}");
}
