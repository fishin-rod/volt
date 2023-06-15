# Volt

Volt is an API wrapper for the revolt API 
written in rust!

**This project is still in its very early stages! So there may be bugs or unexpected behavior!** - If you find any bugs/ glitchs please report it to me (Bird#9223) on revolt, or you can report it directly to the repository

If you would like to help the project by contributing features or improvements all help is welcome! So feel free to create a pull request for anything you would like to add, or contact me directly!

## Getting Started

First you will need to add the volt-rs crate to your `Cargo.toml`:
```toml
[dependencies]
volt-rs = "0.1.1"
```

To enable the experimental branch of volt add this to your dependencies:
```toml
[dependencies]
volt-rs = {version = "0.1.1", features = ["experimental"]}
```

**The experimental branch contains features that are not fully tested and may not work correctly, use with caution!**

## Current TODO list:
1: Finish all bot API endpoints, and document them.
2: Improve the cacheing system to work for all types of calls.
3: Improve error handeling to return the error code recived, and soltions to fix it.
4: Improve the websocket with custom structs for the responces, and better ways to send and recive the events while pinging the server to keep your bot online.