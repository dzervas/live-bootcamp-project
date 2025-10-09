{ pkgs, ... }: {
  languages.rust.enable = true;

  packages = with pkgs; [
    watchexec
  ];

  processes = {
    auth.exec = "watchexec cargo run -p auth-service";
    app.exec = "watchexec cargo run -p app-service";
  };
}
