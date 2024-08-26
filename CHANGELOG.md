# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - YYYY-MM-DD

### Added

- Lightswitch to toggle between dark mode and light mode
- Settings page
- Prompt library page
  - Ability to disable/enable prompts
  - Ability to edit prompts
  - Ability to delete prompts
  - Ability to add prompts
- Credit for Icons

### Changed

### Deprecated

### Removed

### Fixed

- Fixed [#4](https://github.com/CyprienGille/UbiClippy/issues/4). Window now correctly hides when the shortcut is pressed for the first time and it never lost focus before.
- Fixed [#5](https://github.com/CyprienGille/UbiClippy/issues/5). Colors of the clipboard icon are inverted in dark mode.

### Security

## [0.6.0] - 2024-07-31

### Added

- Tray icon
- Closing app to tray
- Showing app with tray left click or shortcut (Ctrl+Alt+C)

### Changed

- Larger default window width
- Renamed `system_tray.rs` to `system.rs`

### Removed

- Direct summon and hide window tauri commands

### Fixed

- Summoning app now un-minimizes it too

## [0.5.0] - 2024-07-30

### Added

- Ability to pull the application into focus with Ctrl+Alt+C shortcut
- Ability to quick-select a prompt using a single letter

### Changed

- Prompts UI now respects line breaks
- Roadmap is now better organized

### Fixed

- Going back to prompt selection now clears chat history on the backend

## [0.4.0] - 2024-07-26

### Added

- Default prompts
- Display and selection of prompt
- Clipboard usage in a prompt
- Model selection before prompt selection
- Ability to go back to prompt selection from the chat interface

### Changed

- Tailwind CSS class order according to Prettier plugin
- User Messages can also be copied now

### Removed

- Ollama api/generate functionality (used at the start for testing)

### Fixed

- Typo is summarization prompt

## [0.3.0] - 2024-07-15

### Added

- Chat interface and functionality
- Ability to copy any response

## [0.2.0] - 2024-07-14

### Added

- Clipboard interactions

## [0.1.0] - 2024-07-14

### Added

- Dummy request to Ollama server
- Button to trigger request
- Display of model response on the UI

## [0.0.0] - 2024-07-12

### Added

- Initial scaffolding of the project
- Github repository
