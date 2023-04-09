# PISS (Priconne Inventory Scanning System)

## What the heck is this?
PISS is a tool that nobody asked for! 
This tool uses OpenCV and Tesseract OCR to allow you to quickly and easily scan your inventory from screenshots files or clipboard. You can import images into the software and it will automatically analyze the contents of the images to identify equipments and the amount of them that you have.

## How is it doin'?
This project is still in **WORK IN PROGRESS** EARLY EARLY ALPHA NOT READY FOR TESTING OR PRODUCTION.

Below is a list of planned features that will be implemented in this project.

 - [ ] Identify equipment and pieces from your inventory page
 - [ ] Identify the amount of it that you have
 - [ ] Identify equipment and the amount that you have (or get) from other places like character screen, drops from quests, etc.
 - [ ] ... can't think of anything else right now
 
I don't plan to add features like inventory or equiptment management, integrating piss to something that already exist like Spugn's PQH made more sense. So I won't add those.

## What it look like?
No, you can't have a look. Not yet. There's nothing to show yet. You can build the project yourself if you want, there's a build instruction below, but I won't handhold you through the entire process.

## But why though?
This project is just an excuse for me to learn how to create a 🚀 **blazingly fast** 🚀 desktop app with Rust, to improve my Rust programming skill, and just to have another repo in my github profile.

## Building this project
To build you will need Rust nightly, I haven't tried to build this with stable Rust.

Before building the project, you need to download and install vcpkg and the dependencies needed. With the help of cargo-vcpkg, this command below should do just that.
```
set VCPKG_DEFAULT_TRIPLET=x64-windows-static
set VCPKGRS_DYNAMIC=0

cargo vcpkg build
```
Those two environment variables might not be necessary because it's already explicitly told to use `x64-windows-static` triplet in the cargo.toml file, and build.rs also told rustc to build static binary. But set those env if you're having trouble building.

After that, build the project with cargo build. the `--target` is necessary to apply the rustflags in `.cargo/config` file. 

```
cargo build --release --target x86_64-pc-windows-msvc
```

There's should be a `piss.exe` file in `target/x86_64-pc-windows-msvc/release/` directory. Since this is a static build, the executable file should be pretty huge in size (currently ~60MB), but there won't be any .dlls needed for it to run.

# Legal stuff
This is a non-commercial fan-made software project based on the game "Princess Connect Re:Dive", which is copyrighted and developed by Cygames. This project is not affiliated with, endorsed, sponsored, or specifically approved by Cygames. The use of the game's copyrighted material is for non-commercial and educational purposes only. All trademarks, characters, and images used in this fan project are the property of their respective owners. The creators of this fan project do not claim any ownership or endorsement of the copyrighted material used in this project. By using this software, you acknowledge that you have read this disclaimer and agree to abide by all applicable copyright laws.