use crate::shared::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
enum EntryType
{
    Directory,
    File
}

#[derive(Debug)]
struct TreeEntry
{
    entry: EntryType,
    size: i32,
    name: String,
    contents: HashMap<String, TreeEntry>
}

impl TreeEntry
{
    pub fn add_item_at_location(&mut self, item: String, directory: &Vec<String>)
    {
        let segments: Vec<&str> = item.split(" ").collect();
        let entry: TreeEntry = TreeEntry {
            entry: if segments[0] == "dir" { EntryType::Directory } else { EntryType::File },
            size: if segments[0] == "dir" { 0 } else { segments[0].parse::<i32>().unwrap() },
            name: segments[1].to_string(),
            contents: HashMap::new()
        };
        if directory.len() == 0 {
            self.contents.insert(segments[1].to_string(), entry);
            return ();
        } else {
            match self.contents.get_mut(&directory[0]) {
                Some(thing) => {
                    thing.insert_or_pass_down(directory, 1, segments[1].to_string(), entry);
                    return ();
                },
                _ => {
                    println!("Shouldn't see this either!");
                }
            }
        }
        println!("Should never see this!");
        ()
    }

    fn insert_or_pass_down(&mut self, directory: &Vec<String>, key: usize, name: String, entry: TreeEntry)
    {
        if key == directory.len() && self.name == directory[key - 1] {
            self.contents.insert(name, entry);
            return ();
        } else {
            let target: &mut TreeEntry = self.contents.get_mut(&directory[key]).unwrap();
            target.insert_or_pass_down(directory, key + 1, name, entry);
            return ();
        }
    }

    pub fn get_total(&self) -> i32
    {
        let mut total: i32 = 0;
        for item in self.contents.iter() {
            match item.1.entry {
                EntryType::Directory => {
                    let dir_total = item.1.get_total();
                    total = total + dir_total;
                }
                EntryType::File => {
                    if self.name == "/" {
                        continue;
                    }
                    total = total + item.1.size;
                }
            }
        }
        total
    }

    pub fn crawl_for_dirs(&self, mut dirs: Vec<(String, i32)>) -> Vec<(String, i32)>
    {
        let size = self.get_total();
        dirs.push((self.name.clone(), size));
        for item in self.contents.iter() {
            match item.1.entry {
                EntryType::Directory => {
                    dirs = item.1.crawl_for_dirs(dirs);
                }
                EntryType::File => {

                }
            }
        }
        dirs
    }

    pub fn count_entries_inside(&self) -> i32
    {
        let mut total = 0;
        for item in self.contents.iter() {
            match item.1.entry {
                EntryType::Directory => {
                    total = total + 1 + item.1.count_entries_inside();
                }
                EntryType::File => {
                    total = total + 1;
                }
            }
        }
        total
    }
}

pub fn run()
{
    let structure: HashMap<String, TreeEntry> = read();
    // dbg!(&structure);

    let top = structure.get("/").unwrap();
    let dirs: Vec<(String, i32)> = Vec::new();
    let output = top.crawl_for_dirs(dirs);
    dbg!(&output.len());

    // dbg!(&output);
    let mut total = 0;
    let mut count = 0;
    for item in output {
        if item.1 <= 100000 && item.0 != "/" {
            count = count + 1;
            dbg!(&item);
            total = total + item.1;
        }
    }
    dbg!(total);
}

fn read() -> HashMap<String, TreeEntry>
{
    let mut structure: HashMap<String, TreeEntry> = HashMap::new();
    structure.insert(String::from("/"), TreeEntry {
            entry: EntryType::Directory,
            size: 0,
            name: String::from("/"),
            contents: HashMap::new()
        });

    let mut directory: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./src/data/7/data.txt") {
        let mut dir_count = 0;
        let mut total_count = 0;
        for line in lines {
            if let Ok(item) = line {
                if item.chars().next().unwrap() == '$' {
                    // This is a command
                    let command: Vec<&str> = item.split(" ").collect();
                    match command[1] {
                        "cd" => {
                            if command[2] == ".." {
                                directory.pop();
                            } else {
                                if command[2] == "/" {
                                    continue;
                                }
                                directory.push(command[2].to_string());
                            }
                            // println!("Going to /{}", directory.join("/"));
                        },
                        _ => {
                            continue;
                        }
                    }
                } else {
                    if item.contains("dir") {
                        dir_count = dir_count + 1;
                    }
                    total_count = total_count + 1;
                    structure.get_mut("/").unwrap().add_item_at_location(item, &directory);
                }
            }
        }
        println!("Total dirs: {dir_count}");
        println!("Total entires: {total_count}");
    }
    structure
}
