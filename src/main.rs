use playsuipi_core::api;
use std::ffi::CString;
use std::io;
use std::ptr;

const SUITS: [&str; 4] = ["♣", "♦", "♥", "♠"];

fn show_card(card: &u8) -> String {
    if card.to_owned() > 51 {
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

fn show_hand(hand: Box<[u8; 8]>) -> String {
    hand.iter()
        .enumerate()
        .map(|(i, x)| {
            format!(
                "{}=({})",
                ((i as u8 + 49) as char).to_string(),
                show_card(x)
            )
        })
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
                ((i as u8 + 65) as char).to_string(),
                show_pile(x.to_owned(), status)
            )
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn get_move() -> CString {
    println!("> Input your move below:");
    let mut x = get_input();
    while let Err(_) = x {
        println!("> Input your move below:");
        x = get_input();
    }
    CString::new(x.unwrap()).unwrap()
}

fn main() {
    let mut g = api::new_game(ptr::null());
    let mut status = api::status(&g);
    println!("[*] Seed: {:?}", status.seed);
    while status.game < 2 {
        if status.turn {
            println!("\n[*] Dealer's turn:");
        } else {
            println!("\n[*] Opponent's turn:");
        }
        println!("\nFloor: {}", show_floor(api::read_floor(&g), &status));
        println!("Hand:  {}\n", show_hand(api::read_hand(&g)));
        let mut error = api::apply_move(&mut g, &get_move());
        while !error.is_empty() {
            println!("Error: {}", error.into_string().unwrap());
            error = api::apply_move(&mut g, &get_move());
        }
        api::next_turn(&mut g);
        status = api::status(&g);
    }
}
