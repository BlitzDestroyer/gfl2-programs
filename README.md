# GFL2-Programs

A small collection of utilities for **Girls’ Frontline 2**, including an automated solver for **Leva’s Memory Puzzle** and optional **gacha rolling**.

This repo currently provides one binary:

```
bin/auto_solve_leva_memory_puzzle.rs
```

This tool solves the daily memory puzzle automatically and can optionally roll the built-in gacha using your account’s authentication token.

---

## Features

- Automatically detects ongoing puzzle state  
- Resumes existing puzzle runs  
- Solves all remaining plays or just a single one  
- Automatically rolls the puzzle gacha if requested  
- Works with an authentication token provided either via CLI or config file

---

## Installation

Clone the repository:

```
git clone https://github.com/BlitzDestroyer/gfl2-programs.git
cd gfl2-programs
```

Build:

```
cargo build --release
```

The binary will be located at:

```
target/release/auto_solve_leva_memory_puzzle
```

---

## Authentication Token

The program needs your **Leva Memory Puzzle auth token**.

You may provide it in either of two ways:

### 1. Pass it directly via CLI

```
auto_solve_leva_memory_puzzle --auth-token "<your_token_here>"
```

### 2. Place it in a config file

Create:

```
leva_puzzle_config.json
```

With:

```
{
    "auth_token": "YOUR_TOKEN_HERE"
}
```

In the same directory as the executable.

If the file is missing, the program will ask you to type the token.

### Obtaining the Authentication Token

To get your authentication token, log in to the official event page, open your browser’s developer tools, and check the **Network** tab. Refresh the page if needed, then look for a request to `/info` (for example: `https://gf2-h5ump45gacha-us-api.sunborngame.com/info`).  
Select that request and copy the exact value of its **Authorization** header.  
Make sure you copy it **without any leading or trailing spaces or newlines**.

---

## Usage

```
auto_solve_leva_memory_puzzle [OPTIONS] [attempts]
```

### Arguments

| Argument   | Description                                                  |
|------------|--------------------------------------------------------------|
| `attempts` | Number of puzzle attempts: `none`, `one`, or `all` (default: `one`) |

### Options

| Flag                     | Description                                                |
|--------------------------|------------------------------------------------------------|
| `-a`, `--auth-token`     | Auth token used for requests (default: config or prompt)   |
| `-g`, `--gacha <attempts>` | Number of gacha rolls: `none`, `one`, or `all`            |

---

## Examples

### Solve only one puzzle
```
auto_solve_leva_memory_puzzle one
```

### Solve all remaining puzzles
```
auto_solve_leva_memory_puzzle all
```

### Solve puzzles and roll one gacha
```
auto_solve_leva_memory_puzzle all --gacha one
```

### Roll all gacha attempts without solving puzzles
```
auto_solve_leva_memory_puzzle none --gacha all
```

---

## How It Works

- Fetches game state from the official puzzle API  
- Detects uncovered cards and already matched pairs  
- Determines optimal click order using memory-matching heuristics  
- Delays between requests to avoid spamming  
- Handles puzzle continuation and new puzzle runs  
- Rolls gacha using the game’s `/gacha` endpoint  

All requests are made through `reqwest` with the authentication header automatically included.

---

## Disclaimer

This project interacts with official servers.  
Use responsibly.  
You are fully responsible for any consequences.
