use std::fs::read_to_string;

#[derive(Clone)]
enum DiskEntity {
    File { id: u64 },
    FreeSpace,
}

#[derive(Clone)]
struct Disk {
    disk: Vec<DiskEntity>,
}

impl Disk {
    fn compress_fragmented(&mut self) {
        let mut i = 0;
        while i < self.disk.len() {
            match self.disk[i] {
                DiskEntity::File { .. } => {}
                DiskEntity::FreeSpace => {
                    let mut j = self.disk.len() - 1;
                    while j > i {
                        match self.disk.pop().unwrap() {
                            DiskEntity::File { id } => {
                                self.disk[i] = DiskEntity::File { id };
                                break;
                            }
                            DiskEntity::FreeSpace => {}
                        }
                        j -= 1;
                    }
                }
            }
            i += 1;
        }
    }

    fn biggest_id(&self) -> u64 {
        match self.disk.last().unwrap() {
            DiskEntity::File { id } => *id,
            DiskEntity::FreeSpace => panic!("Uncompressed disk should end with a file"),
        }
    }

    fn calculate_file_size(&self, id: u64, file_end: usize) -> usize {
        let mut size = 1;
        let mut i = file_end;
        while i > 0 {
            match self.disk[i - 1] {
                DiskEntity::File { id: other_part_id } => {
                    if other_part_id != id {
                        break;
                    }
                    size += 1;
                }
                DiskEntity::FreeSpace => {
                    break;
                }
            }
            i -= 1;
        }
        size
    }

    fn find_free_space_of_size(&self, size: usize) -> Option<usize> {
        let mut found_length = 0;
        let mut section_index = 0;
        for i in 0..self.disk.len() {
            match self.disk[i] {
                DiskEntity::File { .. } => {
                    if found_length >= size {
                        break;
                    } else {
                        found_length = 0;
                    }
                }
                DiskEntity::FreeSpace => {
                    if found_length == 0 {
                        section_index = i;
                    }
                    found_length += 1;
                }
            }
        }
        if found_length >= size {
            Some(section_index)
        } else {
            None
        }
    }

    fn compress_unfragmented(&mut self) {
        let mut i = self.disk.len() - 1;
        let mut current_id = Some(self.biggest_id());
        loop {
            match self.disk[i] {
                DiskEntity::File { id } => {
                    if current_id == Some(id) {
                        let file_size = self.calculate_file_size(id, i);
                        if let Some(free_space_index) = self.find_free_space_of_size(file_size) {
                            if free_space_index < i {
                                let file_begin = i + 1 - file_size;
                                for offset in 0..file_size {
                                    self.disk
                                        .swap(free_space_index + offset, file_begin + offset);
                                }
                            }
                        }

                        current_id = if id == 0 { None } else { Some(id - 1) };
                    }
                }
                DiskEntity::FreeSpace => {}
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    fn print(&self) {
        for entity in &self.disk {
            let char = match entity {
                DiskEntity::File { id } => id.to_string(),
                DiskEntity::FreeSpace => ".".to_string(),
            };
            print!("{}", char)
        }
        println!();
    }

    fn checksum(&self) -> u64 {
        self.disk
            .iter()
            .enumerate()
            .map(|(i, entity)| match entity {
                DiskEntity::File { id } => i as u64 * id,
                DiskEntity::FreeSpace => 0,
            })
            .sum()
    }
}

fn puzzle_1(disk: &Disk) {
    let disk = &mut disk.clone();
    disk.print();
    disk.compress_fragmented();
    disk.print();
    let checksum = disk.checksum();
    println!("Puzzle 1 compressed disk checksum is {checksum}");
}

/*
    Using different representation - structure representing entire block instead of N structs for a block - would make the second
    part of the puzzle WAY easier. There wouldn't be a noot for the weird helper methods that iterate over a disk to check size of 
    free space or a file.
 */
fn puzzle_2(disk: &Disk) {
    let disk = &mut disk.clone();
    disk.print();
    disk.compress_unfragmented();
    disk.print();
    let checksum = disk.checksum();
    println!("Puzzle 2 compressed disk checksum is {checksum}");
}

fn main() {
    let file_content = read_to_string("input.txt").unwrap();
    let disk_content = file_content
        .chars()
        .enumerate()
        .flat_map(|(i, block_size)| {
            let block_size = block_size.to_digit(10).unwrap();
            let disk_entity = if i % 2 == 0 {
                DiskEntity::File { id: (i / 2) as u64 }
            } else {
                DiskEntity::FreeSpace
            };
            std::iter::repeat_n(disk_entity, block_size as usize)
        })
        .collect();
    let disk = Disk { disk: disk_content };

    puzzle_1(&disk);
    puzzle_2(&disk);
}
