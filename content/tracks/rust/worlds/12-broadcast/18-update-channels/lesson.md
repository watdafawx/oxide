rl-hud updates four things on their own: the app, the UI build, team logos and player cuts. A logo fix should not need a new installer, and an app fix should not wait for the next event.

**Versions that sort.** Semantic versions (`26.10.2`) compare part by part; rl-hud adds a build number (`26.10.2-3`) so each release on the same day is newer than the last.

**Offer or force.** Normally a new version shows up in the Updates menu and the operator picks when. "Force" points a channel at a build that every HUD PC applies within about 30 seconds: for fixes that cannot wait, chosen on purpose by a producer.

**Tauri's updater.** `tauri-plugin-updater` reads a JSON feed (version, notes, a download URL and a signature per platform), downloads the installer, checks its signature, and installs. The signature uses a key pair made once: the private key signs releases on the build machine, and the public key is built into the app. A file someone tampered with fails the check, even when it comes from the right URL.

**Assets are data.** Logos and cuts need no installer at all: a zip, its checksum, and a folder to unpack into.

**Never mid-play.** Whatever the channel, applying an update restarts something. On a broadcast PC the safe moment is between matches, which is why "offer" is the default.
