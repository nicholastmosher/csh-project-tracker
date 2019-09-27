use std::env;
use std::io;
use dotenv::dotenv;

fn main() -> io::Result<()> {
    dotenv().ok();
    let sys = actix::System::new("csh-project-tracker");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    csh_project_tracker::app::launch(database_url)?;
    sys.run()?;
    Ok(())
}
