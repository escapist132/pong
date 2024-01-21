mod render;

use std::io;
use rand::Rng;
use render::render;
use render::clear_field;

fn main() {

    // Задаем стартовые координаты для мяча и обеих ракеток
    let mut ball_coordinates = [39, 12];
    let mut ball_move_coef  = [1, 1];
    let mut left_player_racket: i32 = 12;
    let mut right_player_racket: i32 = 12;

    let mut players_score = [0, 0];
    clear_field();

    ball_vector(&mut ball_move_coef);

    loop {
        if score(&mut ball_coordinates, &mut left_player_racket, &mut right_player_racket, &mut players_score, &mut ball_move_coef) {
            render(&ball_coordinates, left_player_racket, right_player_racket);
            controller(&mut ball_coordinates, &mut left_player_racket, &mut right_player_racket, &mut ball_move_coef);
            clear_field();
        } else {
            break;
        }
    }
}


    // Функция чтения с клавиатуры (Первая пошаговая версия)
pub fn keyboard_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ERROR");
    input
}
   
fn controller(ball_coordinates: &mut [i32],
              y_left_player_racket: &mut i32,
              y_right_player_racket: &mut i32,
              ball_move_coef: &mut [i32]) {

    let symb_from_keyboard: String = keyboard_input();

    match symb_from_keyboard.trim() {
        "a" => *y_left_player_racket -= 1,
        "z" => *y_left_player_racket += 1,
        "k" => *y_right_player_racket -= 1,
        "m" => *y_right_player_racket += 1,
        _ => {}
    }

    if ball_coordinates[1] == 23 || ball_coordinates[1] == 1 {
        ball_move_coef[1] *= -1;
    }
    if ball_coordinates[0] == 3 && (ball_coordinates[1] == *y_left_player_racket ||
        ball_coordinates[1] == (*y_left_player_racket + 1) || ball_coordinates[1] == *y_left_player_racket - 1) {
        ball_move_coef[0] *= -1;
    } else if ball_coordinates[0] == 76 && (ball_coordinates[1] == *y_right_player_racket ||
        ball_coordinates[1] == (*y_right_player_racket + 1) || ball_coordinates[1] == *y_right_player_racket - 1) {
        ball_move_coef[0] *= -1;
    }

    ball_coordinates[0] += ball_move_coef[0];
    ball_coordinates[1] += ball_move_coef[1];

}

fn score(ball_coordinates: &mut [i32],
         y_left_player_racket: &mut i32,
         y_right_player_racket: &mut i32,
         players_score: &mut [i32],
         ball_move_coef: &mut [i32]) -> bool{
    let mut flag = true;

    if players_score[0] < 25 && players_score[1] < 25 {
        match ball_coordinates[0] {
            0 => {
                players_score[1] += 1;
                ball_coordinates[0] = 39;
                ball_coordinates[1] = 12;
                *y_right_player_racket = 12;
                *y_left_player_racket = 12;
                println!("\t\t  1-st Player: {}\t\t2-nd Player: {}", players_score[0], players_score[1]);
                ball_vector(ball_move_coef);
            },
            80 => {
                players_score[0] += 1;
                ball_coordinates[0] = 39;
                ball_coordinates[1] = 12;
                *y_right_player_racket = 12;
                *y_left_player_racket = 12;
                println!("\t\t  1-st Player: {}\t\t2-nd Player: {}", players_score[0], players_score[1]);
                ball_vector(ball_move_coef);
            },
            _ => println!("\t\t  1-st Player: {}\t\t2-nd Player: {}", players_score[0], players_score[1])
        }
    } else {
        if players_score[0] > players_score[1] {
            if endgame(1) == false {
                flag = false;
            }
        } else {
            if endgame(2) == false {
                flag = false;
            }
        }
        players_score[0] = 0;
        players_score[1] = 0;
        ball_coordinates[0] = 39;
        ball_coordinates[1] = 12;
        *y_right_player_racket = 12;
        *y_left_player_racket = 12;
        clear_field();
    }
    flag
}

fn ball_vector(ball_move_coef: &mut [i32]) {
    let mut rng = rand::thread_rng();
    let random_bool_for_x: bool = rng.gen();
    let random_bool_for_y: bool = rng.gen();

    if random_bool_for_x {
        ball_move_coef[0] *= -1;
    }
    if random_bool_for_y {
        ball_move_coef[1] *= -1;
    }
}

fn endgame (player_number: i32) -> bool {
    clear_field();
    let mut flag = true;
    println!("\n\n\n\t\t\t\tPlayer {} is WIN!!\n\t\t\t\tDo you want to restart?(y/n)\n", player_number);
    let symb_from_keyboard: String = keyboard_input();
    if symb_from_keyboard.trim() == "y" || symb_from_keyboard.trim() == "Y" {
        flag = true;
    } else if symb_from_keyboard.trim() == "n" || symb_from_keyboard.trim() == "N" {
        flag = false;
    } else {
        endgame(player_number);
    }

    flag
}