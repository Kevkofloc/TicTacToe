use std::io;

fn main() {
    println!("Hello, world!");
    let mut matrix = vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]];
    let mut player = 1;
    let mut count = 0;
    let mut playfield = vec![vec![' ',' ',' '],vec![' ',' ',' '] ,vec![' ',' ',' ']];
    println!("{:?}", matrix);
    'game: loop{
        if player == 1{
            println!("Spieler X ist dran");
        }
        else { println!("Spieler O ist dran") }
        let mut coordinatex = String::new();
        let mut coordinatey = String::new();
        println!("Enter your move:");
        println!("Enter x (0-2):");
        io::stdin()
            .read_line(&mut coordinatex)
            .expect("Fehler");
        let coordinatex = coordinatex.trim().parse::<i32>().unwrap();
        println!("Enter y (0-2):");
        io::stdin()
            .read_line(&mut coordinatey)
            .expect("Fehler");
        let coordinatey = coordinatey.trim().parse::<i32>().unwrap();
        if coordinatex <0 || coordinatex >2 || coordinatey<0||coordinatey>2{
            println!("Fehler");
            continue 'game;
        }
        if matrix[coordinatey as usize][coordinatex as usize] != 0{
            println!("Besetzt");
            continue 'game;
        }
        matrix[coordinatey as usize][coordinatex as usize] = player;
        if player == 1{
            playfield[coordinatey as usize][coordinatex as usize] = 'X';
        }
        else { playfield[coordinatey as usize][coordinatex as usize] = 'O'; }
        for x in 0..3{
            for y in 0..3 {
                if y != 2{print!("  {}  |", playfield[x][y]);}
                else{print!("  {}  ", playfield[x][y]);}
            }
            println!("");
            if x != 2{
            println!("-----------------");}
        }
        let test3 = matrix[0][0]+matrix[1][1]+matrix[2][2];
        let test4 = matrix[0][2]+matrix[1][1]+matrix[2][0];
        for i in 0..3{
            let test = matrix[i][0] + matrix[i][1] + matrix[i][2];
            let test2 = matrix[0][i] + matrix[1][i] + matrix[2][i];
            let tests  = vec![test, test2, test3, test4];
            for i in tests{
                if i == 3{
                    println!("Spieler X hat gewonnen");
                    break 'game;
                }
                if i == -3{
                    println!("Spieler O hat gewonnen");
                    break 'game;
                }
            }

        }


        player = player*(-1);
        count +=1;
        if count == matrix.len()*matrix[0].len(){
            println!("Unentschieden");
            break 'game
        }

    }

}
