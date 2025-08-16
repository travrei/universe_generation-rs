# Universe Generation Demo in Rust
This is a Rust implementation of a **procedurally generated universe**, inspired by [Jonkero's video on Youtube](https://www.youtube.com/@Jonkero).

## About
After watching Jonkero's excellent video on this matter, I decided to try recreate but in Rust.
>💡 Some things I had to figure out on my own — for example, imgui required using unsafe and I wanted to avoid that.

>💡 I also couldn't figure out how to integrate egui into the project...

>💡 Additionally, I had to tweak the perfectly_hash_them function a bit to make it work properly with Rust.

## 📹 Inspiration

Original concept by **Jonkero** – [Watch the video here](https://www.youtube.com/watch?v=el7p-HC77g8)

> Big thanks to Jonkero for sharing the knowledge and inspiration!

## 🔧 Tech Stack

- Language: **Rust**
- Libraries: Just **[Raylib](https://github.com/raylib-rs/raylib-rs)**

## 📦 How to Run

```bash
# Clone the repository
git clone https://github.com/travrei/universe_generation-rs.git
cd universe_generation-rs

# Run the project
cargo run
