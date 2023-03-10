
use rand;
extern crate matrix_display;
use matrix_display::*;


fn next_gen(board: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let row_count = board.len();
    let col_count = board[0].len();

    let mut new: Vec<Vec<i8>> = vec![vec![0; row_count]; col_count];

    for i in 0..row_count {
        for j in 0..col_count {
           
            let current_state = board[i][j];
            let mut neighbours_alive = 0;

            for x in -1i8..=1 {
                for y in -1i8..=1 {

                  
                    let mut new_x = (i as i8) + x;
                    let mut new_y = (j as i8) + y;
                    new_x = (new_x + row_count as i8) % row_count as i8;
                    new_y = (new_y + col_count as i8) % col_count as i8;
                    neighbours_alive += board[new_x as usize][new_y as usize];
                    
                


                    
                }
            }
            neighbours_alive -= current_state;
            if current_state == 1 && neighbours_alive < 2 {
                new[i][j] = 0;
            } else if current_state == 1 && neighbours_alive > 3 {
                new[i][j] = 0;
            } else if current_state == 0 && neighbours_alive == 3 {
                new[i][j] = 1;
            } else {
                new[i][j] = current_state;
            }



        }
    }
    new
}


fn display_matrix(board_grid: &Vec<Vec<i8>>)  {
    let rows = board_grid.len();

    let format = Format::new(2,2);
    let bg_flat: Vec<_> = board_grid.into_iter().flatten().collect();
    let bg_flat_str = bg_flat.into_iter().enumerate().map(|(i,x)| { x.to_string()}).collect::<Vec<_>>();
    
    let bg_ =  bg_flat_str.iter()
    .enumerate()
    .map(|(i, x)| {
        let ansi_fg = 0;
        let mut ansi_bg = 0;
        if x == "1"{

            ansi_bg = 1;
        } else {
            ansi_bg = 0;
        }
        cell::Cell::new(x.clone(), ansi_fg, ansi_bg)
        }).collect::<Vec<_>>();

    let mut data = matrix::Matrix::new(rows, bg_);
    let mut display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);

}

fn main() {
    
    const gens: u8 = 20;

    let (rows,cols) = (10,10);
    let mut board_grid: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    for (i, row) in board_grid.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            let rand: i8 = if rand::random() { 1 } else { 0 };
            *col = rand;
        }
    }
    //blinker 
    // board_grid[3][2] = 1;
    // board_grid[2][2] = 1;
    // board_grid[1][2] = 1;

    //glider
    // board_grid[2][1] = 1;
    // board_grid[2][2] = 1;
    // board_grid[2][3] = 1;
    // board_grid[1][3] = 1;
    // board_grid[0][2] = 1;
    // board_grid[3][2] = 1;
  
    println!("Initial grid:");

    println!("");
    display_matrix(&board_grid);
    
    for i in 0..gens {
        board_grid = next_gen(&board_grid);

        println!("Generation {}:", i+1);
        display_matrix(&board_grid);
        println!("");
    }

  
    
}
