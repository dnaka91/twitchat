# Twitchat

A twitch chat overlay written in Rust.

## Try it out

This project is hosted at <https://dnaka91.github.io/twitchat>.

It runs completely on your local PC and doesn't talk to any custom servers. The only exception Twitch for the actual chat and in the future BetterTTV and FrankerFacez for additional emotes.

Options are set through url query parameters like:

<https://dnaka91.github.io/twitchat?color=black&channels=bobross>

The full list of options is as follows:

| Name      | Default | Description                                                     |
| --------- | ------- | --------------------------------------------------------------- |
| color     | white   | Text color for chat messages. Can be either `white` or `black`. |
| channels  |         | List of channels to connect to, sparated by commas.             |
| font_size | 16.0    | Font size for all messages.                                     |

## Build

Have the latest Rust toolchain installed (preferably with [rustup](https://rustup.rs/)) and follow
the instruction for [trunk](https://github.com/thedodd/trunk) afterwards. As this project uses
_wasm-opt_ you'll have to install binaryen as well (details in the trunk readme).

Finally with everything installed run `trunk serve --release` in the terminal and keep it open. Now
you can test out the overlay at <http://localhost:8080>. Apply options as show above like
<http://localhost:8080?color=black&channels=bobross>.

## License

This project is licensed under the [AGPL-3.0 License](LICENSE) (or
<https://www.gnu.org/licenses/agpl-3.0.html>).
