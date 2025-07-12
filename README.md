# Blinkion

Blinkion is a cross-platform desktop app built with [Dioxus](https://dioxuslabs.com/) that helps you maintain healthy screen habits by reminding you to blink and correct your posture at configurable intervals.

## Features
- **Blink Reminders:** Periodic popups to remind you to blink and rest your eyes.
- **Posture Reminders:** Timed posture correction prompts with animated SVG illustrations.
- **Animated SVGs:** Smooth, animated graphics for both blink and posture reminders using `dioxus_motion`.
- **Tray Icon:** Persistent tray icon for quick access and settings.
- **Settings Window:** Easily configure blink and posture intervals and durations.
- **Customizable Styles:** Uses Tailwind CSS and custom SVG/CSS for a polished look.

## Quick Start
1. **Install Rust** (if you haven't): https://rustup.rs/
2. **Clone the repo:**
   ```bash
   git clone <repo-url>
   cd blinkion
   ```
3. **Run the app:**
   ```bash
   cargo run
   ```
   Or, for Dioxus desktop hot-reload:
   ```bash
   dx serve --platform desktop
   ```
4. **Configure settings:**
   - Click the tray icon and select "Settings" to adjust reminder intervals and durations.

## Project Structure

```
project/
├─ assets/         # App assets (SVGs, CSS, icons)
├─ src/
│  ├─ main.rs      # App entry point, window/tray logic
│  ├─ components/  # UI components (AnimatedBlink, AnimatedPosture, etc.)
│  ├─ reminder.rs  # Reminder logic
│  ├─ shared_state.rs # Global state and settings
│  ├─ signals.rs   # Signals for inter-component communication
├─ Cargo.toml      # Dependencies and features
```

---

# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css" # also customize the location of the out file!
```

### Tailwind Manual Install

To use tailwind plugins or manually customize tailwind, you can can install the Tailwind CLI and use it directly.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

