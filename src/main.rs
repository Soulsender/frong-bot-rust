use poise::serenity_prelude as serenity;
use dotenv::dotenv;

struct Data {} // user data
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// create slash command
#[poise::command(slash_command)]
async fn test(ctx: Context<'_>) -> Result<(), Error> {
    // send response
    let response = "This is a test!";
    ctx.say(response.to_string()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // get token from .env file
    dotenv::from_path(".env").expect("[!] No path found to .env");
    dotenv().expect("[!] Error loading .env file");

    // set token variable
    let token = std::env::var("TOKEN").expect("[!] No Bot Token Provided");

    // set bot intents
    let intents = serenity::GatewayIntents::all();

    // setup pose framework
    // this includes slash commands and guild(s)
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![test()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, Into::into(860417200426188820)).await?;
                Ok(Data {})
            })
        })
        .build();

    // create bot client
    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    // start the client
    client.unwrap().start().await.unwrap();
}
