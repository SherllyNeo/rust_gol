
use rand;

fn next_gen(board: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let row_count = board.len();
    let col_count = board[0].len();

    let mut new: Vec<Vec<i8>> = vec![vec![0; row_count]; col_count];

    for i in 0..row_count {
        for y in 0..col_count {
           
            let current_state = board[i][y];
            let mut neighbours_alive = 0;

            for x in -1i8..=1 {
                for y in -1i8..=1 {

                  
                    let new_x = ((i as i8) + x + row_count as i8) % row_count as i8;
                    let new_y = ((y as i8) + y + col_count as i8) % col_count as i8;

                    neighbours_alive += board[new_x as usize][new_y as usize];
                    neighbours_alive -= current_state;



                    
                }
            }
            if current_state == 1 && neighbours_alive < 2 {
                new[i][y] = 0;
            } else if current_state == 1 && neighbours_alive > 3 {
                new[i][y] = 0;
            } else if current_state == 0 && neighbours_alive == 3 {
                new[i][y] = 1;
            } else {
                new[i][y] = current_state;
            }



        }
    }
    new
}


fn main() {
    const gens: u8 = 5;

    let (rows,cols) = (5,5);
    let mut board_grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    for (i, row) in board_grid.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            let rand: i8 = if rand::random() { 1 } else { 0 };
            *col = rand;
        }
    }
    println!("Initial grid:");
    board_grid.iter().for_each(|i| {
        println!("{:?}", i);
    });
    println!("");
    for i in 0..gens {
        board_grid = next_gen(&board_grid);

        println!("Generation {}:", i+1);
        board_grid.iter().for_each(|i| {
            println!("{:?}", i);
        });
        println!("");
    }

  
    
}
