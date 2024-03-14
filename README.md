# Blue Crab Shells: Getting Started with Offensive Rust Programming

This repository contains example code for the live training provided for [BSides San Diego](https://bsidessd.org).


## Project Plan

We are building a lil baby Rust C2! We won't do anything crazy with encrypted comms, but we will be writing a command handler. 

## Learning Objectives

### Skills

* Cross-compilation
* Windows crate usage
* Network programming

### Concepts

* Advantages (and drawbacks) of using Rust for offensive code
* Windows API interoperability
* Rust types
* The Rust build pipeline

## Prerequisites

Ideally, you've done some programming before. Rust experience is not required, but fluency in one language will be very helpful.

Familiarity with some Windows API concepts will also be helpful, but isn't a firm requirement.

## Required Materials

* Computer capable of compiling Rust programs. Linux is easier, but Windows will do. I recommend using [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)

## Structure

This repository is built in stages, with each stage given a different branch. You can follow along by switching between stages, or simply seeing the final result on `main`.

But the point is for you to create this code, or something like it, during the training. This is merely a reference point.

## Setup

### Install Rust

Head to [rustup.rs](https://rustup.rs) and follow the instructions.

### Get a Text Editor

I recommend [VS Code](https://code.visualstudio.com).

### Have a Victim

If you're using Windows as your dev environment, this can be the same machine. Otherwise, you should have a Windows computer that we'll use as our victim. It should be on the same network as the dev machine.

### Have Netcat

If you're on Linux, you're sorted. On Windows, I recommend downloading [Nmap](https://nmap.org/download.html) and making sure you install `NCat` along with everything else.

### Have the Docs Open

Get the [Rust Standard Library](https://doc.rust-lang.org/std/) open right now. I promise you'll need a few ~~dozen~~ tabs of this thing open.

## Stages 

### Stage 0

Our initial prototype to prove communication over TCP. Nothing fancy, but a few things to note:

- The syntax for importing modules
- The `Result` type, and how it's handled
- The clarity of the TCP stack

### Stage 1

Before we do more with that network connection, let's get comfy with the Windows API. We'll simply pop a MessageBox when the agent launches. But "simple" isn't really how Rust rolls. 

Observe the imports from the `windows` crate. That's just what we needed to make a simple popup work! Also note how we're wrapping values in the Windows structs (e.g. `PCSTR`) to make the typing work. 

Also, you can see that this interop is, by definition, unmanaged. That's why we need to wrap our Windows function calls in `unsafe`. It doesn't mean what you think it means.

### Stage 2

Now we'll establish two-way communication over the TCP socket. This introduces the `BufWriter` and `BufReader` structs for handling the transmission and receipt of data.

This stage also introduces the `match` statement, a core flow control structure in Rust. We use it to handle what we're reading from the socket, thereby checking whether the connection has been closed.

### Stage 3

Now that we have both send and receive working, it's time to kick off some commands! We'll start with basic Powershell, since it's easy enough. We introduce `std::process` and all its fun tricks here.

### Stage 4

In this stage, we begin to build out our `C2Command` enum and handler. Shell commands are cool, but we'd like to not rely on them. By prepending our input with `!`, we indicate it is a proper C2 command.

But most importantly, we do all this in `cmd` module, so now we know how to modularize our code. 

We haven't implemented either of our starter C2 commands, but we're about to.

### Stage 5

A little more restructuring and cleanup here. Many of the changes you see at this stage are the result of `cargo clippy` making sure we're doing things with best practices. We also add submodules for our two commands, and a handler function.

We've also structured the output to the stream to differentiate between an error and a success.

### Stage 6

Implementation! We use the `winreg` crate to easily access the Windows registry to establish persistence via a common technique. Is it the stealthiest? No, but it beats the command line!

### Stage 7

Persistence is one thing, but wouldn't it be cool if we could get `SYSTEM` privileges. We can, using the Token Duplication technique! This involves a good deal of Windows API programming, but that's what we're here for. We do fudge a little bit and use a third party crate for enumerated processes, because that is an unreasonable faff.


### Stage 8

In this version, we're going to take our whole project, and make it usable as a...DLL! That's right, Rust can make DLLs! 

To make the project buildable as both a library and a binary, we've heavily restructured the modules for both. Also, look at what we've added to `Cargo.toml`.
