# Rust ls

## A custom ls command written in rust. 

> looking to rename this as to not get confused with rls (rust language server)

### Build from source

Clone the repo:
- `git clone git@github.com:MikeDupree/rls.git`

cd into the project root
- `cd rls`

Run the build command
- `cargo build --release`

Copy the build file into a directory under your PATH
Be sure to replace `SOME/PATH` with a PATH for your bin files

- `cp target/release/rls SOME/PATH`

If your using a linux os that has `~/.local/bin/` in it's PATH already you can run

- `pnpm build:bin`

or if you use npm or yarn

- `npm run build:bin`
- `yarn build:bin`

### Download

< Pre Built Download Coming Soon >

### Contributions
- mdupree (maintainer)
