# Dynamic Discord Rich Presence Customizer

<details>
	<summary>Table of Contents</summary>
	<ol>
		<li><a href="#about">About</a></li>
		<li>
			<a href="#getting-started">Getting Started</a>
			<ul>
				<li><a href="#installation">Installation</a></li>
				<li><a href="#build">Build</a></li>
			</ul>
		</li>
		<li><a href="#usage">Usage</a></li>
		<li>
			<a href="#featuresroadmap">Features/Roadmap</a>
			<ul>
				<li><a href="#010">0.1.0</a></li>
				<li><a href="#020">0.2.0</a></li>
				<li><a href="#030">0.3.0</a></li>
			</ul>
		</li>
		<li><a href="#supported-platforms">Supported Platforms</a></li>
		<!-- <li><a href=""></a></li> -->
	</ol>
</details>

## About

This is the 8th revival and hopefully the final iteration of this project.

The purpose of this program is to allow the user to set custom Discord rich presence content while being able to dynamically change fields with running programs or your actively playing Spotify track.

## Getting Started

### Installation

### Build

You can compile the source code yourself, just make sure you have [Rust](https://rustup.rs) installed.

1. Clone the repo:

```bash
git clone https://github.com/T1mbits/ddrpc.git
```

2. Change directories:

```bash
cd ddrpc
```

3. Build the binary

```bash
cargo build --release
```

4. The compiled binary should be in the `target/release` directory.

## Usage

jokes on you I've been editing the config file to work this thing

I'll finish the CLI later...

## Features/Roadmap

-   [x] Custom Discord rich presence
-   [x] Detect target processes
-   [x] Spotify integration
-   [ ] Template variables
-   [ ] Functional CLI with proper I/O
-   [ ] (useful) CLI based help
-	[ ] TUI
-   [ ] IPC for detailed custom data
-   [ ] Custom defined template variables (used with IPC)
-   [ ] GUI

## Supported Platforms

-   [x] Linux
<details>
<summary>Tested on</summary>
	<ul>
		<li>Arch</li>
	</ul>
If you would like to add on other Linux OSes, please feel free to create an issue with the OS you used and if it's compatible or not, as well as any fixes to make the program work on the OS of choice.
</details>

-   [ ] Windows
-   [ ] MacOS

## Contributing

If you have any suggestions or improvements, feel free to fork the repo and create a pull request. You can also just create an issue with your suggestion or improvement.

1. Fork the Project
2. Create your Feature Branch

```bash
git checkout -b feature/AmazingFeature
```

3. Commit your Changes

```bash
git commit -m 'Add some AmazingFeature'
```

4. Push to the Branch

```bash
git push origin feature/AmazingFeature
```

5. Open a Pull Request
