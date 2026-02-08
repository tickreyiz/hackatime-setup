{
  description = "Hackatime setup tool";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          platformMap = {
            x86_64-linux = "linux-x86_64";
            aarch64-linux = "linux-aarch64";
            x86_64-darwin = "macos-x86_64";
            aarch64-darwin = "macos-aarch64";
          };
          assetName = "hackatime_setup-${platformMap.${system}}.tar.gz";
          
          
          sha256Map = {
            x86_64-linux = "17v1zhxqqqc5sha9k9z0d9w9cpcrfhc8l93kq3slyccjm6jfwc8c";
            aarch64-linux = "15lqbygas6g089icjscp9vxh33sfgfszshymc3l2wx73nipf1ahn";
            x86_64-darwin = "0xlfghdiwyfgr3pl18ssb9y44vcd0galxmdlrrx2j1q0qi56x3dc";
            aarch64-darwin = "1v39h7ag1zazsh9h4ndk2r7wiqm5nk44kiakdsvp32y3i2mgiimr";
          };
        in
        {
          default = pkgs.stdenv.mkDerivation rec {
            pname = "hackatime-setup";
            version = "latest";

            src = pkgs.fetchurl {
              url = "https://github.com/hackclub/hackatime-setup/releases/download/v${version}/${assetName}";
              sha256 = sha256Map.${system} or (throw "Unsupported system: ${system}");
            };

            unpackPhase = "tar -xzf $src";
            
            nativeBuildInputs = pkgs.lib.optional pkgs.stdenv.isLinux pkgs.autoPatchelfHook;
            buildInputs = [ pkgs.openssl pkgs.zlib pkgs.stdenv.cc.cc.lib ];

            installPhase = ''
              mkdir -p $out/bin
              install -Dm755 hackatime_setup $out/bin/hackatime-setup
            '';

            meta = with pkgs.lib; {
              description = "Installer for Hackatime coding time tracker";
              homepage = "https://github.com/hackclub/hackatime-setup";
              license = licenses.mit;
              platforms = supportedSystems;
              mainProgram = "hackatime-setup";
            };
          };
        });
    };
}