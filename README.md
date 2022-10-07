# rimca
A CLI mineraft launcher which allows launching sans authentication

## motivation
Minecraft is just one of those games that's been in my life for as long as I can remember at this point, and at the time of conception I felt there was a lack of a simple CLI launcher which allowed launching in `offline-mode` & was actively maintained. 

As I run my servers in `offline-mode` due to the fact I often play in areas with no internet connection, I wanted a client that was able to also launch without relying on external servers.

In addition, I do not like the direction in which Mojang is going and/or has taken, especially after the 1.19.1 update, which is just extra motivation to not rely on their services as much as possible.

## usage
```
rimca 0.2.1

USAGE:
    rimca <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete      Delete a minecraft instance
    download    Download minecraft version as an instance
    help        Prints this message or the help of the given subcommand(s)
    launch      Launch minecraft instance
    list        List installed minecraft instances
    login       Login a user
```