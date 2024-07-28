# The cross platform game syncer!

Sync your games between MACOS, WINDOWS and LINUX easily!

![Screenshot 2024-07-28 at 14 11 54](https://github.com/user-attachments/assets/6ba255d3-2fb0-420b-8b22-c850f3417567)

There are games on steam that don't support cross platform save file synchronization.

For me, playing Lies of P on both my macbook and windows PC could be annoying to manually sync the files.

So I made this software: __PLATFORM-SYNCER__

## Important Info

The serverside component of this software platform must be self-hosted, whether that is from a server provider, or your home server. The server is built to run on MacOS or Linux.

The clientside can run on MACOS, WINDOWS and LINUX! It was built using the Tauri framework (https://tauri.app/) for the UI.

For both the server and the client, you can build it yourself with cargo, or use the provided executables.

## Instructions

### Client

* Download the client files
* Run the executable
* Add the URL of your server (it MUST include "http://")

![Screenshot 2024-07-28 at 14 21 47](https://github.com/user-attachments/assets/840453b2-dee9-464a-920a-d23ef4f9cbfd)

* Add a game
* Play!

### Server
* Download the server files
* In the server root directory, edit the "url.txt" file to contain the port you want to expose the server on (the default should be fine)
* Run the executable!
* Play!
