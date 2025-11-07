{ pkgs, ... }:
{
  languages.rust.enable = true;

  packages = with pkgs; [
    sqlx-cli
    watchexec
  ];

  env.DATABASE_URL = "sqlite://auth-service.db";

  processes = {
    auth.exec = "watchexec cargo run -p auth-service";
    app.exec = "watchexec cargo run -p app-service";
  };
}
