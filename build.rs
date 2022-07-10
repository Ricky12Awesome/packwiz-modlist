use dotenv_build::Config;

fn main() {
  dotenv_build::output(Config::default()).unwrap();
}
