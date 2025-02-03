use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about="Tracks time spent on tasks", long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Add a task")]
    add: Option<String>,
    #[arg(short, long, help = "Delete a task by ID")]
    delete: Option<u16>,
    #[arg(short, long, help = "Mark a task as completed by ID")]
    complete: Option<u16>,
    #[arg(short, long, help = "List all tasks")]
    list: bool,
    #[arg(short, long, help = "Set the status of a task by ID")]
    status: Option<u16>,
}

impl Cli {
    pub fn get_add(&self) -> Option<&str> {
        self.add.as_deref()
    }

    pub fn get_delete(&self) -> Option<u16> {
        self.delete
    }

    pub fn get_complete(&self) -> Option<u16> {
        self.complete
    }

    pub fn get_list(&self) -> bool {
        self.list
    }

    pub fn get_status(&self) -> Option<u16> {
        self.status
    }
}
