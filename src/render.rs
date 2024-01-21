use crate::controller;
use std::process::Command;

    // Публичная функция отрисовки карты принимающая координаты объектов
pub fn render(ball_coordinates: &[i32], y_left_player_racket: i32, y_right_player_racket: i32) {
        // Верхняя граница
    for _i in 0..81 {
        print!("-");
    }

    // Полес учетом объектов
    for y_field in 0..24 {
        print!("\n|");

        for x_field in 0..79 {
            if ball_coordinates[0] == x_field && ball_coordinates[1] == y_field {
                print!("*");
            } else if (y_left_player_racket == y_field ||
                y_left_player_racket == (y_field + 1) ||
                y_left_player_racket == (y_field - 1)) && x_field == 2 {
                print!("|");
            } else if (y_right_player_racket == y_field ||
                y_right_player_racket == (y_field + 1) ||
                y_right_player_racket == (y_field - 1)) && x_field == 77 {
                print!("|");
            } else {
                print!(" ");
            }
        }
        print!("|");
    }

    print!("\n");

    // Нижняя граница
    for _i in 0..81 {
        print!("-");
    }
    print!("\n");
}

// Очистка командной строки перед последующщим кадром
pub fn clear_field() {
    let mut command = Command::new("clear");
    let output = command.output().expect("msg");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}