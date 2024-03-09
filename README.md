# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/main/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm create svelte@latest

# create a new project in my-app
npm create svelte@latest my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

## Building
https://zenn.dev/kubocker/articles/dc6b3b211c77cd

install Rust
(aptで入れるとwasm-packでエラーする：https://blog.hergo.jp/softwares/rust-wasm32-unknown-unknown/)
https://www.rust-lang.org/ja/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


install wasm-pack
https://qiita.com/i-masaki/items/c968f01da691c990aa20
 sudo apt install gcc
 cargo install wasm-pack

 (一応場合によって エラーケース)
 https://stackoverflow.com/questions/61767863/wasm-bindgen-command-not-found-even-though-wasm-pack-is-installed-0-8-1
 https://phoenixnap.com/kb/install-rust-ubuntu

rust-analyzer error
https://zenn.dev/razokulover/scraps/17844b5b5c7147

svelte format
https://blog.webcreativepark.net/2023/02/14-175342.html
