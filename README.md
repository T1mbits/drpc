# Dynamic Discord Rich Presence Customizer

This is the 8th revival and hopefully the final iteration of this project.

The purpose of this program is to allow the user to set custom Discord rich presence content while being able to dynamically change fields with running programs or your actively playing Spotify track.

## Quirks to be fixed

-   logging system is goofy
    -   IPC parser on thread has a 100ms delay to compensate for the logging system writing on top of itself and mangling the data
    -   may be fixed by spawning a temporary thread to parse IPC message
