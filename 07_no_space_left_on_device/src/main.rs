use std::{collections::HashMap, error::Error, fs::read_to_string, path::PathBuf, str::FromStr};

enum Command {
    ChangeDirectory(String),
    List(Vec<String>),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(command_str: &str) -> Result<Self, Self::Err> {
        match &command_str[..2] {
            "cd" => {
                let (_, directory_name) = command_str
                    .split_once(' ')
                    .ok_or("The cd command should contain a directory name after it")?;

                Ok(Self::ChangeDirectory(directory_name.to_string()))
            }
            "ls" => {
                let listed_entries = match command_str.split_once('\n') {
                    None => Vec::new(),
                    Some((_, output)) => output.split('\n').map(String::from).collect::<Vec<_>>(),
                };

                Ok(Self::List(listed_entries))
            }
            _ => Err(format!("Invalid command \"{}\"", command_str).into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct FileSystemEntry {
    name: String,
    size: usize,
    kind: FileSystemEntryKind,
}

#[derive(Debug, Clone, Default)]
enum FileSystemEntryKind {
    #[default]
    Directory,
    File,
}

impl FileSystemEntry {
    fn new_directory(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            size: 0,
            kind: FileSystemEntryKind::Directory,
        }
    }

    fn new_file(name: impl ToString, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            kind: FileSystemEntryKind::File,
        }
    }

    fn is_directory(&self) -> bool {
        match self.kind {
            FileSystemEntryKind::File => false,
            FileSystemEntryKind::Directory => true,
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    root: PathBuf,
    size: usize,
    current_directory: PathBuf,
    entries: HashMap<PathBuf, FileSystemEntry>,
}

impl FromStr for FileSystem {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut fs = FileSystem::new("/");

        let commands = value[2..]
            .split("\n$ ")
            .map(str::parse::<Command>)
            .collect::<Result<Vec<_>, _>>()?;

        for command in commands {
            match command {
                Command::ChangeDirectory(name) => fs.change_current_directory(name)?,
                Command::List(listed_entries) => listed_entries
                    .iter()
                    .filter_map(|entry| entry.split_once(' '))
                    .for_each(|(prefix, name)| {
                        if prefix == "dir" {
                            fs.create_directory(name);
                        } else {
                            fs.create_file(name, prefix.parse::<usize>().unwrap());
                        }
                    }),
            }
        }

        Ok(fs)
    }
}

impl FileSystem {
    fn new(root_directory_name: impl ToString) -> Self {
        let mut entries = HashMap::new();

        let root_path = PathBuf::from(root_directory_name.to_string());

        let root_directory = FileSystemEntry::new_directory(root_path.to_string_lossy());

        entries.insert(root_path.clone(), root_directory);

        Self {
            root: root_path.clone(),
            size: 70000000,
            entries,
            current_directory: root_path,
        }
    }

    fn create_directory(&mut self, name: impl ToString) {
        let entry = FileSystemEntry::new_directory(name);

        let new_entry_path = self.current_directory.join(entry.name.clone());

        self.entries.insert(new_entry_path, entry);
    }

    fn create_file(&mut self, name: impl ToString, size: usize) {
        let entry = FileSystemEntry::new_file(name, size);

        let new_entry_path = self.current_directory.join(entry.name.clone());

        for path in self.current_directory.ancestors() {
            self.entries
                .entry(path.to_path_buf())
                .and_modify(|entry| entry.size += size)
                .or_default();
        }

        self.entries.insert(new_entry_path, entry);
    }

    fn change_current_directory<N: AsRef<str> + Into<PathBuf>>(
        &mut self,
        directory_name: N,
    ) -> Result<(), Box<dyn Error>> {
        match directory_name.as_ref() {
            ".." => {
                self.current_directory.pop();
                Ok(())
            }
            name if name == self.root.to_str().unwrap() => {
                self.current_directory = self.root.clone();
                Ok(())
            }
            name => {
                let path = self.current_directory.join(name);

                match self.entries.get(&path) {
                    Some(entry) => match entry.kind {
                        FileSystemEntryKind::File => Err("Tried to cd into a file".into()),
                        FileSystemEntryKind::Directory => {
                            self.current_directory = path;
                            Ok(())
                        }
                    },
                    None => Err("Tried to cd into a non-existent directory".into()),
                }
            }
        }
    }

    fn free_space(&self) -> usize {
        let used_space = self
            .entries
            .get(&self.root)
            .map(|root_entry| root_entry.size)
            .unwrap_or(0);

        self.size - used_space
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("./input.txt")?;

    let file_system = input.trim().parse::<FileSystem>()?;

    let directories = file_system
        .entries
        .values()
        .filter(|entry| entry.is_directory())
        .collect::<Vec<_>>();

    let size_limit = 100000;
    let sum_of_directory_sizes_under_limit: usize = directories
        .iter()
        .filter(|entry| entry.size < size_limit)
        .map(|entry| entry.size)
        .sum();

    println!(
        "The sum of all directory total sizes that are under {} is {}",
        size_limit, sum_of_directory_sizes_under_limit
    );

    let space_needed_for_update = 30000000;
    let space_to_clear = space_needed_for_update - file_system.free_space();

    let smallest_directory_to_delete_for_update = directories
        .iter()
        .filter(|entry| entry.size > space_to_clear)
        .min_by(|a, b| a.size.cmp(&b.size))
        .ok_or("No directory is big enough to make room for the update")?;

    println!(
        "The smallest directory that can be deleted to make room for the update has size {}",
        smallest_directory_to_delete_for_update.size
    );

    Ok(())
}
