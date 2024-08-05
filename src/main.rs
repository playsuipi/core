use playsuipi_core::api;
use std::env;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{stdin, BufReader, Read, Result as IOResult};
use std::ptr;

const SUITS: [&str; 4] = ["♣", "♦", "♥", "♠"];

const SUIPI: [&str; 5] = [
    "   ____     _      _   __",
    "  / __/_ __(_)__  (_) / /",
    " _\\ \\/ // / / _ \\/ / /_/ ",
    "/___/\\_,_/_/ .__/_/ (_)  ",
    "          /_/            ",
];

fn show_suipi() -> String {
    SUIPI.join("\n")
}

fn show_card(card: &u8) -> String {
    if *card > 51 {
        String::from("__")
    } else {
        let val = card % 13 + 1;
        format!(
            "{}{}",
            match val {
                1 => "A".to_string(),
                2..=10 => val.to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                _ => "_".to_string(),
            },
            String::from(SUITS[(card / 13) as usize])
        )
    }
}

fn show_group(cards: Vec<u8>, value: u8) -> String {
    cards
        .iter()
        .fold(
            (Vec::new(), Vec::new(), 0),
            |(mut builds, mut xs, mut count), x| {
                xs.push(show_card(x));
                count += x % 13 + 1;
                if count == value {
                    builds.push(xs);
                    (builds, Vec::new(), 0)
                } else {
                    (builds, xs, count)
                }
            },
        )
        .0
        .iter()
        .map(|xs| {
            if xs.len() == 1 {
                xs.first().unwrap().to_string()
            } else {
                format!("{{{}}}", xs.join(" + "))
            }
        })
        .collect::<Vec<String>>()
        .join(" & ")
}

fn show_pile(pile: api::Pile, status: &api::Status) -> String {
    let cards = pile
        .cards
        .iter()
        .map(|&x| x.to_owned())
        .filter(|&x| x != 52)
        .collect::<Vec<u8>>();
    let owned = if pile.owner == status.turn { "*" } else { "" };
    if cards.is_empty() {
        String::from("[]")
    } else if cards.len() == 1 {
        format!("({})", show_card(cards.first().unwrap()))
    } else if pile.build {
        format!(
            "{}{}{{{}}}",
            owned,
            pile.value,
            cards
                .iter()
                .map(show_card)
                .collect::<Vec<String>>()
                .join(" + "),
        )
    } else {
        format!("{}{}[{}]", owned, pile.value, show_group(cards, pile.value))
    }
}

fn show_hand(hand: [u8; 8]) -> String {
    hand.iter()
        .enumerate()
        .map(|(i, x)| format!("{}=({})", (i as u8 + 49) as char, show_card(x)))
        .collect::<Vec<String>>()
        .join(", ")
}

fn show_floor(floor: Box<[api::Pile; 13]>, status: &api::Status) -> String {
    floor
        .iter()
        .enumerate()
        .map(|(i, x)| {
            format!(
                "{}={}",
                (i as u8 + 65) as char,
                show_pile(x.to_owned(), status)
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn show_scores(opp: &api::Scorecard, dealer: &api::Scorecard) -> String {
    format!(
        "[*] Scores:\n\n\
        Player | Aces | Most Cards | Most Spades | 10♦ | 2♠ | Suipis | Total\n\
        ------ | ---- | ---------- | ----------- | --- | -- | ------ | -----\n\
        Opp    |    {} |          {} |           {} |   {} |  {} |      {} |  {}\n\
        Dealer |    {} |          {} |           {} |   {} |  {} |      {} |  {}\n\
        ",
        opp.aces,
        opp.most_cards,
        opp.most_spades,
        opp.ten_of_diamonds,
        opp.two_of_spades,
        opp.suipi_count,
        opp.total,
        dealer.aces,
        dealer.most_cards,
        dealer.most_spades,
        dealer.ten_of_diamonds,
        dealer.two_of_spades,
        dealer.suipi_count,
        dealer.total,
    )
}

fn get_input() -> IOResult<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

fn get_move() -> CString {
    println!("> Input your move below:");
    let mut x = get_input();
    while x.is_err() {
        println!("> Input your move below:");
        x = get_input();
    }
    CString::new(x.unwrap()).unwrap()
}

fn get_seed<R: Read>(r: R) -> IOResult<[u8; 32]> {
    let mut br = BufReader::new(r);
    let mut lines = String::new();
    br.read_to_string(&mut lines)?;
    let mut seed = [0; 32];
    lines
        .split('\n')
        .filter_map(|str| match str.parse::<u8>() {
            Ok(x) => Some(x),
            Err(_) => None,
        })
        .enumerate()
        .for_each(|(i, x)| seed[i] = x);
    Ok(seed)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed = if args.len() > 1 {
        match File::open(args[1].as_str()) {
            Ok(f) => match get_seed(f) {
                Ok(s) => &s,
                Err(_) => ptr::null(),
            },
            Err(_) => ptr::null(),
        }
    } else {
        ptr::null()
    };
    let mut g = unsafe { api::new_game(seed) };
    let mut status = api::status(&g);
    let mut game = status.game;
    let mut round = status.round;
    println!("[*] Seed: {:?}", status.seed);
    println!("{}", show_suipi());
    while status.game < 2 {
        if status.turn {
            println!("\n[*] Dealer's turn:");
        } else {
            println!("\n[*] Opponent's turn:");
        }
        println!("\nFloor: {}", show_floor(api::read_floor(&g), &status));
        println!("Hand:  {}\n", show_hand(*api::read_hand(&g)));
        unsafe {
            loop {
                let error = CStr::from_ptr(api::apply_move(&mut g, get_move().as_ptr()))
                    .to_str()
                    .unwrap();
                if !error.is_empty() {
                    println!("{}", error);
                } else {
                    break;
                }
            }
        }
        api::next_turn(&mut g);
        status = api::status(&g);
        if status.floor == 0 {
            println!("{}", show_suipi());
        }
        if game != status.game {
            let scores = api::get_scores(&g);
            println!(
                "{}",
                show_scores(
                    &scores[(game * 2) as usize],
                    &scores[(game * 2 + 1) as usize],
                )
            );
            game = status.game;
        } else if round != status.round {
            println!(
                "\n\
                ================\n\
                == Next Round ==\n\
                ================"
            );
            round = status.round;
        }
    }
}
