use crate::diskutil::{
    compact, compact_blocks, compute_checksum, compute_checksum_block, load_disk, load_disk_blocks,
};
use anyhow::*;
use aoc2024::*;

const DAY: &str = "09";
const SOLUTION_PART_1: &str = "6384282079460";
const SOLUTION_PART_2: &str = "6408966547049";

mod diskutil {
    use crate::diskutil::AtomicBlock::{FileBlock, FreeBlock};
    use crate::diskutil::DiskBlock::{File, FreeSpace};

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum AtomicBlock {
        FreeBlock,
        FileBlock(usize),
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum DiskBlock {
        FreeSpace(usize),   // size
        File(usize, usize), // size, file ID
    }

    pub fn load_disk_blocks(disk: &str) -> Vec<DiskBlock> {
        let mut blocks: Vec<DiskBlock> = Vec::new();

        let mut file_id = 0usize;
        for (index, char) in disk.trim().chars().enumerate() {
            if let Some(block_length) = char.to_digit(10) {
                let size = block_length as usize;

                let block = if index % 2 == 0 {
                    file_id += 1;
                    File(size, file_id - 1)
                } else {
                    FreeSpace(size)
                };

                blocks.push(block);
            }
        }

        blocks
    }

    pub fn load_disk(diskmap: &str) -> Vec<AtomicBlock> {
        let mut blocks: Vec<AtomicBlock> = Vec::new();

        let mut fileid = 0usize;
        for (index, char) in diskmap.trim().chars().enumerate() {
            if let Some(block_length) = char.to_digit(10) {
                let block = if index % 2 == 0 {
                    fileid += 1;
                    FileBlock(fileid - 1)
                } else {
                    FreeBlock
                };

                for _ in 0..block_length {
                    blocks.push(block);
                }
            }
        }
        blocks
    }

    pub fn compact_blocks(blocks: &[DiskBlock]) -> Vec<DiskBlock> {
        let mut compacted = blocks.to_vec();

        let mut index = compacted.len();
        let mut last_fileid_checked = usize::MAX;

        // Move files
        while index > 0 {
            index -= 1;
            if let File(file_size, file_id) = compacted[index] {
                if file_id < last_fileid_checked {
                    last_fileid_checked = file_id;
                    // println!("Checking if file #{} can be moved...", file_id);

                    let mut moving_index = 0;
                    while moving_index < index {
                        moving_index += 1;

                        if let FreeSpace(space_size) = compacted[moving_index] {
                            if space_size >= file_size {
                                // println!(
                                //     "-> File #{} can be moved to position #{}",
                                //     file_id, moving_index
                                // );
                                // Replace the space by the file
                                compacted[moving_index] = File(file_size, file_id);
                                // Replace the file by some space
                                compacted[index] = FreeSpace(file_size);
                                // Add space some extra space if needed
                                if space_size > file_size {
                                    compacted.insert(
                                        moving_index + 1,
                                        FreeSpace(space_size - file_size),
                                    );
                                    index += 1;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Merge spaces
        index = 1;
        while index < compacted.len() - 1 {
            if let FreeSpace(space1_size) = compacted[index] {
                if let FreeSpace(space2_size) = compacted[index + 1] {
                    // println!(
                    //     "Merging Freespace({}) with Freespace({})...",
                    //     space1_size, space2_size
                    // );
                    compacted[index] = FreeSpace(space1_size + space2_size);
                    compacted.remove(index + 1);
                    continue;
                }
            }
            index += 1;
        }

        compacted
    }

    pub fn compact(disk: &[AtomicBlock]) -> Vec<AtomicBlock> {
        let mut compacted: Vec<AtomicBlock> = disk.to_vec();

        let mut index = 0;

        while index < compacted.len() {
            // If current block if file, nothing to do
            if compacted[index] != FreeBlock {
                index += 1;
                continue;
            }

            match compacted.pop().unwrap() {
                FreeBlock => {}
                FileBlock(file_id) => {
                    compacted[index] = FileBlock(file_id);
                }
            }
        }

        compacted
    }

    pub fn compute_checksum(disk: &[AtomicBlock]) -> usize {
        disk.iter()
            .enumerate()
            .fold(0, |acc, (index, block)| match block {
                FreeBlock => 0,
                FileBlock(file_id) => acc + *file_id * index,
            })
    }

    pub fn compute_checksum_block(disk: &[DiskBlock]) -> usize {
        let mut total = 0;

        let mut index = 0;
        for block in disk.iter() {
            match block {
                FreeSpace(size) => {
                    index += size;
                }
                File(size, file_id) => {
                    for _ in 0..*size {
                        total += file_id * index;
                        index += 1;
                    }
                }
            }
        }
        total
    }
}

//region Part 1

fn solve_part_1(input_data: &str) -> Result<String> {
    let disk = load_disk(input_data);
    let compacted = compact(&disk);
    let checksum = compute_checksum(&compacted);
    Ok(format!("{}", checksum))
}
//endregion

//region Part 2

fn solve_part_2(input_data: &str) -> Result<String> {
    let disk = load_disk_blocks(input_data);
    let compacted = compact_blocks(&disk);
    let checksum = compute_checksum_block(&compacted);
    Ok(format!("{}", checksum))
}
//endregion

fn main() -> Result<()> {
    start_day(DAY);

    let (input_data, duration) = time_function!(get_input_data(DAY)?);
    println!("Input data loaded in {}", pretty_duration(duration));

    let (answer_part_1, part1_duration) = time_function!(solve_part_1(&input_data)?);
    println!(
        "Part 1: {} (solved in {})",
        answer_part_1,
        pretty_duration(part1_duration)
    );
    check_result(&answer_part_1, SOLUTION_PART_1);

    let (answer_part_2, part2_duration) = time_function!(solve_part_2(&input_data)?);
    println!(
        "Part 2: {} (solved in {})",
        answer_part_2,
        pretty_duration(part2_duration)
    );
    check_result(&answer_part_2, SOLUTION_PART_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::diskutil;
    use crate::diskutil::AtomicBlock::{FileBlock, FreeBlock};
    use crate::diskutil::DiskBlock::{File, FreeSpace};
    use crate::diskutil::{compute_checksum, compute_checksum_block};

    #[test]
    fn should_load_disk() {
        // Test setup
        let expected_disk = [
            FileBlock(0),
            FileBlock(0),
            FreeBlock,
            FreeBlock,
            FreeBlock,
            FileBlock(1),
            FileBlock(1),
            FileBlock(1),
            FreeBlock,
            FreeBlock,
            FreeBlock,
            FileBlock(2),
            FreeBlock,
            FreeBlock,
            FreeBlock,
            FileBlock(3),
            FileBlock(3),
            FileBlock(3),
            FreeBlock,
            FileBlock(4),
            FileBlock(4),
            FreeBlock,
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FreeBlock,
            FileBlock(6),
            FileBlock(6),
            FileBlock(6),
            FileBlock(6),
            FreeBlock,
            FileBlock(7),
            FileBlock(7),
            FileBlock(7),
            FreeBlock,
            FileBlock(8),
            FileBlock(8),
            FileBlock(8),
            FileBlock(8),
            FileBlock(9),
            FileBlock(9),
        ];

        // Given a disk map
        let diskmap = "2333133121414131402";

        // When load the disk
        let disk = diskutil::load_disk(diskmap);

        // Then the loaded disk should correspond to the expected one
        assert_eq!(
            disk, expected_disk,
            "Expected disk: {:?} (actual: {:?})",
            expected_disk, disk
        );
    }

    #[test]
    fn should_load_disk_blocks() {
        // Test setup
        let expected_disk = [
            File(2, 0),
            FreeSpace(3),
            File(3, 1),
            FreeSpace(3),
            File(1, 2),
            FreeSpace(3),
            File(3, 3),
            FreeSpace(1),
            File(2, 4),
            FreeSpace(1),
            File(4, 5),
            FreeSpace(1),
            File(4, 6),
            FreeSpace(1),
            File(3, 7),
            FreeSpace(1),
            File(4, 8),
            FreeSpace(0),
            File(2, 9),
        ];

        // Given a disk map
        let diskmap = "2333133121414131402";

        // When load the disk
        let disk = diskutil::load_disk_blocks(diskmap);

        // Then the loaded disk should correspond to the expected one
        assert_eq!(
            disk, expected_disk,
            "Expected disk: {:?} (actual: {:?})",
            expected_disk, disk
        );
    }

    #[test]
    fn should_compact() {
        // Test setup
        let expected_result = [
            FileBlock(0),
            FileBlock(0),
            FileBlock(9),
            FileBlock(9),
            FileBlock(8),
            FileBlock(1),
            FileBlock(1),
            FileBlock(1),
            FileBlock(8),
            FileBlock(8),
            FileBlock(8),
            FileBlock(2),
            FileBlock(7),
            FileBlock(7),
            FileBlock(7),
            FileBlock(3),
            FileBlock(3),
            FileBlock(3),
            FileBlock(6),
            FileBlock(4),
            FileBlock(4),
            FileBlock(6),
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FileBlock(6),
            FileBlock(6),
        ];

        // Given a fragmented disk
        let disk = [
            FileBlock(0),
            FileBlock(0),
            FreeBlock,
            FreeBlock,
            FreeBlock,
            FileBlock(1),
            FileBlock(1),
            FileBlock(1),
            FreeBlock,
            FreeBlock,
            FreeBlock,
            FileBlock(2),
            FreeBlock,
            FreeBlock,
            FreeBlock,
            FileBlock(3),
            FileBlock(3),
            FileBlock(3),
            FreeBlock,
            FileBlock(4),
            FileBlock(4),
            FreeBlock,
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FreeBlock,
            FileBlock(6),
            FileBlock(6),
            FileBlock(6),
            FileBlock(6),
            FreeBlock,
            FileBlock(7),
            FileBlock(7),
            FileBlock(7),
            FreeBlock,
            FileBlock(8),
            FileBlock(8),
            FileBlock(8),
            FileBlock(8),
            FileBlock(9),
            FileBlock(9),
        ];

        // When the disk is defragmented
        let compacted = diskutil::compact(&disk);

        // Then the defragmented disk should correspond to the one expected
        assert_eq!(expected_result.to_vec().len(), compacted.len());
        assert_eq!(
            expected_result.to_vec(),
            compacted,
            "Expected disk: {:?} (actual: {:?})",
            expected_result,
            compacted
        );
    }
    #[test]
    fn should_compact_blocks() {
        // Test setup
        let expected_compacted = [
            File(2, 0),
            File(2, 9),
            File(1, 2),
            File(3, 1),
            File(3, 7),
            FreeSpace(1),
            File(2, 4),
            FreeSpace(1),
            File(3, 3),
            FreeSpace(4),
            File(4, 5),
            FreeSpace(1),
            File(4, 6),
            FreeSpace(5),
            File(4, 8),
            FreeSpace(2),
        ];

        // Given a fragmented disk
        let disk = [
            File(2, 0),
            FreeSpace(3),
            File(3, 1),
            FreeSpace(3),
            File(1, 2),
            FreeSpace(3),
            File(3, 3),
            FreeSpace(1),
            File(2, 4),
            FreeSpace(1),
            File(4, 5),
            FreeSpace(1),
            File(4, 6),
            FreeSpace(1),
            File(3, 7),
            FreeSpace(1),
            File(4, 8),
            File(2, 9),
        ];

        // When the disk is defragmented
        let compacted = diskutil::compact_blocks(&disk);

        // Then the defragmented disk should correspond to the one expected
        assert_eq!(expected_compacted.to_vec().len(), compacted.len());
        assert_eq!(
            expected_compacted.to_vec(),
            compacted,
            "Expected disk: {:?} (actual: {:?})",
            expected_compacted,
            compacted
        );
    }

    #[test]
    fn should_compute_checksum() {
        // Test setup
        let expected_result = 1928;

        // Given a disk
        let disk = [
            FileBlock(0),
            FileBlock(0),
            FileBlock(9),
            FileBlock(9),
            FileBlock(8),
            FileBlock(1),
            FileBlock(1),
            FileBlock(1),
            FileBlock(8),
            FileBlock(8),
            FileBlock(8),
            FileBlock(2),
            FileBlock(7),
            FileBlock(7),
            FileBlock(7),
            FileBlock(3),
            FileBlock(3),
            FileBlock(3),
            FileBlock(6),
            FileBlock(4),
            FileBlock(4),
            FileBlock(6),
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FileBlock(5),
            FileBlock(6),
            FileBlock(6),
        ];

        // When calculating it checksum
        let checksum = compute_checksum(&disk);

        // Then it should correspond to the one expected
        assert_eq!(
            expected_result, checksum,
            "Expected checksum: {:?} (actual: {:?})",
            expected_result, checksum
        );
    }

    #[test]
    fn should_compute_checksum_blocks() {
        // Test setup
        let expected_result = 2858;

        // Given a disk
        let disk = [
            File(2, 0),
            File(2, 9),
            File(1, 2),
            File(3, 1),
            File(3, 7),
            FreeSpace(1),
            File(2, 4),
            FreeSpace(1),
            File(3, 3),
            FreeSpace(4),
            File(4, 5),
            FreeSpace(1),
            File(4, 6),
            FreeSpace(5),
            File(4, 8),
            FreeSpace(2),
        ];

        // When calculating it checksum
        let checksum = compute_checksum_block(&disk);

        // Then it should correspond to the one expected
        assert_eq!(
            expected_result, checksum,
            "Expected checksum: {:?} (actual: {:?})",
            expected_result, checksum
        );
    }
}
