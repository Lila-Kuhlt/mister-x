mod args;

use args::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);

    match cli.command {
        Commands::Team(team) => handle_team_command(team),
        Commands::Game(game) => handle_game_command(game),
        Commands::Gadget(gadget) => handle_gadget_command(gadget),
    }
}

fn handle_team_command(team: TeamCommand) {
    match team {
        TeamCommand::List {} => println!("List teams"),
        TeamCommand::Create { name, role , color} => println!("Create team {} with role {:?} and color {:?}", name, role, color),
        TeamCommand::Show { name } => println!("Show team {}", name),
        TeamCommand::Delete { name } => println!("Delete team {}", name),
        TeamCommand::Edit { name, new_name, color, role } => println!("Edit team {} with new name {:?}, color {:?} and role {:?}", name, new_name, color, role),
    }
}

fn handle_game_command(game: GameCommand) {
    match game {
        GameCommand::Pause { duration } => println!("Pause game for {:?}", duration),
        GameCommand::Resume {resume_time} => println!("Resume game at {:?}", resume_time),
        GameCommand::Start { start_time } => println!("Start game at {:?}", start_time),
        GameCommand::Stop {} => println!("Stop game"),
        GameCommand::Show {} => println!("Show game"),
    }
}

fn handle_gadget_command(gadget: GadgetCommand) {
    match gadget {
        GadgetCommand::Disable { name, team } => println!("Disable gadget {} for team {:?}", name, team),
        GadgetCommand::Enable { name, team } => println!("Enable gadget {} for team {:?}", name, team),
        GadgetCommand::List { active, on_cooldown, team } => println!("List gadgets with active {:?}, on_cooldown {:?} and team {:?}", active, on_cooldown, team),
        GadgetCommand::Reset { name, team } => println!("Reset gadget {} for team {:?}", name, team),
        GadgetCommand::Show { name } => println!("Show gadget {}", name),

    }
}
