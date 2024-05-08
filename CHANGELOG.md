# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- IPC socket for custom data (1.0.0)
- Custom defined template variables (1.0.0)
- TUI interface
- CLI interface
	- Proper output messages
	- Discord set and get
	- Processes management
	- Spotify account and client management
- Logging to files
- Timestamps

### Changed
- Must be given flag to actually log anything (to stdout or file)
- Will not ping Spotify API under certain conditions:
	- Program received --no-spotify
	- No {{spotify}} variables found
- Program handles error & prints error messages in main loop

### Deprecated

### Fixed

### Removed
- Unused CLI arguments

### Security

## [0.1.3] - 2024-05-07

### Added
- AppState to hold changing data in the program, separate of Config which shouldn't change under normal circumstances
	- DiscordState to hold Discord IPC client and previous activity data for comparisons
	- Will be used for TUI as well so big +
- stupid_type_parameters_damnit() to convert Result<(), ()> to Result<Option\<AppState>, ()> as a temporary fix for return types because my error handling is scuffed

### Changed
- Will only write to config during explicit write operations (setting config fields)
- Will only read config on program startup (may reimplement daemon for update command and detachable TUI/GUI interface)
- All data from Config or AppState is only borrowed, never moved (probably, probably forgot to change it in some unused function)

### Fixed
- Program will use Spotify fallback fields if client secret or id is missing or an error occurs during authorization

## [0.1.2] - 2024-05-07

### Fixed
- Fields can be blank, if a button field is missing or an image asset is missing the button/asset will not show (intended)

## [0.1.1] - 2024-05-06

### Added
- SerializeConfig trait for Config types
	- May need to reimplement as it currently reads config from the file

### Fixed
- Can now use nested template variables
	- Will reevaluate all template variable's values once so that infinite nesting is not possible, but fallback nesting is
	- if no value is available (eg. template variable's value is another template variable referencing itself) a template variable string (eg. {{spotify.track.name}}) will show instead

## [0.1.0] - 2024-05-05

### Added
- CHANGELOG.md
- Barely functional CLI
- Customizable Discord rich presence client
- Detect chosen processes and display custom text & image on the Discord rich presence
- Detect chosen Spotify song and display custom text & image on the Discord rich presence
- idk what else tbh