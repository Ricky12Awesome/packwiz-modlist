const CURSEFORGE_API: &str = "https://addons-ecs.forgesvc.net/api/v2/addon/search?gameId=432&pageSize=10&categoryId=0&sectionId=6";
const MODRINTH_API: &str = "https://api.modrinth.com/v2/project";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let req = reqwest::get(CURSEFORGE_API).await?.text().await?;

  println!("{req}");

  Ok(())
}
