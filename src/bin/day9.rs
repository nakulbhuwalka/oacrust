use std::fs::read_to_string;
fn main() {
    let data = read_to_string("data/day9.txt").unwrap();
    let blocks = data
        .chars()

        
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("{}", blocks.iter().sum::<u32>());

    let mut is_space = false;

    let mut block_to_move: usize = blocks.len() - 1;
    let mut moved_blocks: u32 = 0;
    let mut position = 0;

    let mut checksum: u64 = 0;
    println!("{}", block_to_move);
    'outer: for (index, block) in blocks.iter().enumerate() {
        if is_space {
            let empty_len = *block;
            let mut block_len = blocks[block_to_move];
            for _ in 0..empty_len {
                if moved_blocks == block_len {
                    block_to_move -= 2;
                    block_len = blocks[block_to_move];
                    moved_blocks = 0;
                }
                let id: u64 = u64::try_from(block_to_move).unwrap() / 2;
                // println!(
                //     "emp {} {} {} {} {}, {}",
                //     block_to_move, index, i, moved_blocks, empty_len, id
                // );
                if index >= block_to_move {
                    break 'outer;
                }
                checksum += position * id;
                position = position + 1;
                moved_blocks += 1;

                // println!("{} {}", id, checksum);
            }
        } else {
            let file_len = *block;
            let id: u64 = u64::try_from(index).unwrap() / 2;
            for i in 0..file_len {
                // println!(
                //     "ful {} {} {} {} {}",
                //     block_to_move, index, i, moved_blocks, file_len
                // );
                if index >= block_to_move && i + moved_blocks >= file_len {
                    break 'outer;
                }
                checksum += position * id;
                position = position + 1;
                // println!("{} {}", id, checksum);
            }
        }
        is_space = !is_space;
    }
    println!("checksum {}", checksum);
}
