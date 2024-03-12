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
