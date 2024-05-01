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

> [!NOTE]
> I haven't done this before and I'm just hoping that these are the correct steps.

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

### 0.1.0

-   [x] Custom Discord rich presence
-   [x] Detect target processes
-   [ ] Spotify integration
-   [x] Template variables
-   [ ] An actual CLI
-   [ ] (useful) CLI based help

##### 0.2.0?

-   [ ] IPC for in-depth process data
-   [ ] Custom defined template strings (used with above)

##### 0.3.0?

-   [ ] GUI

###### Literally never

-   [ ] actually writing good code

## Supported Platforms

-   [x] Linux (Arch)
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
