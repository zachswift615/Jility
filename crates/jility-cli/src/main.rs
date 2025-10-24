use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod output;

use commands::{init, ticket};

#[derive(Parser)]
#[command(name = "jility")]
#[command(about = "AI-native project management for humans and agents", long_about = None)]
#[command(version)]
struct Cli {
    /// Run as MCP (Model Context Protocol) server for AI agents
    #[arg(long)]
    mcp_server: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Jility project
    Init,
    
    /// Manage tickets
    Ticket {
        #[command(subcommand)]
        command: TicketCommands,
    },
}

#[derive(Subcommand)]
enum TicketCommands {
    /// Create a new ticket
    Create {
        /// Ticket title
        #[arg(short, long)]
        title: String,
        
        /// Ticket description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Story points
        #[arg(short = 'p', long)]
        story_points: Option<i32>,
        
        /// Assignees (comma-separated)
        #[arg(short, long)]
        assignees: Option<String>,
        
        /// Labels (comma-separated)
        #[arg(short, long)]
        labels: Option<String>,
        
        /// Priority (low, medium, high, urgent)
        #[arg(long)]
        priority: Option<String>,
    },
    
    /// List tickets
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        
        /// Filter by assignee
        #[arg(short, long)]
        assignee: Option<String>,
        
        /// Output format (table or json)
        #[arg(short = 'f', long, default_value = "table")]
        format: String,
    },
    
    /// Show ticket details
    Show {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
        
        /// Output format (pretty or json)
        #[arg(short = 'f', long, default_value = "pretty")]
        format: String,
    },
    
    /// Update ticket metadata
    Update {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
        
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        
        /// Story points
        #[arg(short = 'p', long)]
        story_points: Option<i32>,
        
        /// Priority
        #[arg(long)]
        priority: Option<String>,
        
        /// Labels (comma-separated)
        #[arg(short, long)]
        labels: Option<String>,
    },
    
    /// Edit ticket description with $EDITOR
    Edit {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
    },
    
    /// Change ticket status
    Move {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
        
        /// Target status
        #[arg(short, long)]
        to: String,
    },
    
    /// Assign ticket to someone
    Assign {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
        
        /// Assignee to add
        #[arg(short, long)]
        to: String,
        
        /// Remove assignee instead of adding
        #[arg(short, long)]
        remove: bool,
    },
    
    /// Add a comment to a ticket
    Comment {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
        
        /// Comment text
        text: String,
    },
    
    /// Show ticket description history
    History {
        /// Ticket number (e.g., TASK-1)
        ticket_number: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check if running as MCP server
    if cli.mcp_server {
        return jility_mcp::run_mcp_server().await;
    }

    // Initialize tracing for normal CLI usage
    tracing_subscriber::fmt::init();

    match cli.command {
        Some(Commands::Init) => {
            init::run().await?;
        }
        Some(Commands::Ticket { command }) => {
            match command {
                TicketCommands::Create {
                    title,
                    description,
                    story_points,
                    assignees,
                    labels,
                    priority,
                } => {
                    ticket::create(title, description, story_points, assignees, labels, priority).await?;
                }
                TicketCommands::List { status, assignee, format } => {
                    ticket::list(status, assignee, format).await?;
                }
                TicketCommands::Show { ticket_number, format } => {
                    ticket::show(ticket_number, format).await?;
                }
                TicketCommands::Update {
                    ticket_number,
                    title,
                    story_points,
                    priority,
                    labels,
                } => {
                    ticket::update(ticket_number, title, story_points, priority, labels).await?;
                }
                TicketCommands::Edit { ticket_number } => {
                    ticket::edit(ticket_number).await?;
                }
                TicketCommands::Move { ticket_number, to } => {
                    ticket::move_ticket(ticket_number, to).await?;
                }
                TicketCommands::Assign { ticket_number, to, remove } => {
                    ticket::assign(ticket_number, to, remove).await?;
                }
                TicketCommands::Comment { ticket_number, text } => {
                    ticket::comment(ticket_number, text).await?;
                }
                TicketCommands::History { ticket_number } => {
                    ticket::history(ticket_number).await?;
                }
            }
        }
        None => {
            println!("Use --help for usage information");
            println!("Use --mcp-server to run as MCP server for AI agents");
        }
    }

    Ok(())
}
