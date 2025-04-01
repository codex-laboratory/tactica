# Tactica Contribution Guide

Welcome to the Tactica project! This guide will help you get started with setting up the project, building the engine, and making contributions. Follow these steps to ensure a smooth development experience.

## Prerequisites
Make sure you have the following installed:
- **Rust** (latest stable version)
- **Cargo** (comes with Rust)
- **CMake** (version 3.15 or newer)
- **MinGW** (for Windows, to build the C++ components)
- **Visual Studio Code** (or your preferred IDE)
- **Git** (for version control)

### Cloning the Repository
Before making any changes, please fork the repository. 
Clone the repository to your local machine:
```bash
git clone https://github.com/yourusername/tactica.git
cd tactica
```

Set the upstream:
```bash
git remote add upstream https://github.com/codex-laboratory/tactica.git
```

Update the fork with the main repo if needed:
```bash
git fetch upstream
git merge upstream/main
```

---

## Setting Up the Project
### Step 1: Build the C++ Library
The engine relies on a dynamic library written in C++ for advanced calculations.

#### Building on Windows
```bash
cd calculations
mkdir build
cd build
cmake -G "MinGW Makefiles" ..
mingw32-make
```
This will generate the following files in `calculations/build/Release`:
- `libcalc.dll` - The dynamic library
- `libcalc.dll.a` - The import library

#### Building on Linux
```bash
cd calculations
mkdir build
cd build
cmake ..
make
```
This will generate:
- `libcalc.so` - The shared object library

### Step 2: Move the DLL to the Debug Directory (Windows)
Copy the DLL manually or using a script:
```bash
cp calculations/build/Release/libcalc.dll engine-core/target/debug/
```
Or use PowerShell:
```powershell
Copy-Item calculations\build\Release\libcalc.dll engine-core\target\debug\
```

### Step 3: Build the Rust Project
Navigate to the Rust core directory and build the project:
```bash
cargo run --manifest-path engine-core/Cargo.toml
```
If everything is set up correctly, the project should run.

---

## Development Workflow
1. Make sure you are on the latest version of the `main` branch:
   ```bash
   git pull origin main
   ```
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/my-awesome-feature
   ```
3. Make your changes and commit them:
   ```bash
   git add .
   git commit -m "prefix/my-commit-description"
   ```
4. Push your branch to the repository:
   ```bash
   git push origin feature/my-awesome-feature
   ```
5. Open a pull request on GitHub.

---

## Code Guidelines
- **Code should be self-explanatory**, minimizing the need for comments. Refactor instead of commenting every line.

- Comments are allowed, as long as they are respectful and not explaining every single line of code (keep it simple). Make sure **comments are more about the “why” than the “how”, or even a playful joke**.

- Use **CMake** and **Rust build scripts** effectively to manage dependencies.

- Keep your changes modular and easy to integrate, **follow SRP**.

- **Don't ignore warnings in code**. We know some people just ignore them because is not a big deal or just for the memes and that's fine as long as it does not affect your project, however, every warning has a reason to exist and that reason can be harmful for a critical software product like engines. So don't overlook them: **find the reason of it and try to solve it**, even if they don’t break your code right now, they might in the future, so yes, it is a *big deal*.

-  Each commit should ideally address a single issue or feature. This makes it easier to review changes and roll back if necessary. **Make commits atomic size**.

## Pull Request Guidelines
- Try to choose an issue for your contribution.
- If you don't find any issue for your bug or feature, open it yourself. 

---

## Troubleshooting
### DLL Not Found Error (Windows)
If you encounter:
```
STATUS_DLL_NOT_FOUND
```
Make sure the DLL file (`libcalc.dll`) is present in the executable directory:
```
engine-core/target/debug/libcalc.dll
```

Alternatively, add the DLL path to your system’s `PATH` variable.

### Linker Errors (Rust)
If Rust cannot find the library, double-check your `build.rs` script:
```rust
fn main() {
    println!("cargo:rustc-link-lib=dylib=calc");
    println!("cargo:rustc-link-search=native=../calculations/build/Release");
}
```

---

## License
This project is licensed under the MIT License. Feel free to use, modify, and contribute!

Happy coding! 💪🦀

