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
- Will not read/write to file while running (including logging when implemented) unless specifically asked to
	- Will only write to file during explicit write operations (setting config fields)
	- Will only read on program startup (may reimplement daemon for update command and detachable TUI/GUI interface)
	- Must be given flag to actually log anything (to stdout or file)
- Will not ping Spotify API under certain conditions:
	- No client id/secret
	- No Spotify flag
	- No {{spotify}} variables found

### Deprecated

### Fixed
- Allow idle fields to be blank

### Removed
- Unused CLI arguments

### Security

## [0.1.2] - 2024-05-07

### Fixed
- Idle fields will be given a value if none is provided because the program doesn't work otherwise for some reason

## [0.1.1] - 2024-05-07

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