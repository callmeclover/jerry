# jerry

stupid robot nuisance.
i built this to be that though, so...

presses keys and f\*cks with your mouse. occasionally talks.

## versions

currently, there are 3 versions (not including crosses of any):

- default (basic movement and window)
- invisibility (no console window)
- advanced (advanced movement feature)
- microphone (pipe jerry into microphone)

**the `advanced` feature requires administrator privileges. this is why it is seperate from `default`. it also depends on the `windows` crate, so it does not support linux/macos.**

**the `microphone` feature requires [virtual audio cable.](https://vac.muzychenko.net/en/) it only supports windows currently.**

**the `invisibility` feature has no console window, so it cannot be regularly exited out of. you'll need to kill its process.**

i will also not be publishing any invisible or advanced builds for jerry beyond [v0.3.4](https://github.com/callmeclover/jerry/releases/tag/v0.3.4) and [v1.0.0](https://github.com/callmeclover/jerry/releases/tag/v1.0.0) respectively. build them yourself with `cargo build --features invisible` or `cargo build --features advanced`.

(advanced builds may be released later, if it can work cross-platform)

## little notice

i, @callmeclover, hold 0 responsibility over what this thing does, [**especially forks**](#on-the-topic-of-forks). you are on your own if he does anything bad.

## on the topic of forks

jerry is, technically, considered malware. as the [wiki page](https://wikipedia.com/wiki/Malware) states, _"[Malware] is any software intentionally designed to cause disruption to a computer, server, client, or computer network,"_ which is exactly what jerry does.

i have no intent to cause any amount of harm with this program. this is purely a passion project to learn more rust, and test my coding abilities.

when you fork this project, or a fork of this project, you take all responsibility for that fork. i, @callmeclover, hold no responsibility over the actions of forks.

## developing

jerry runs natively on windows, macos, and linux, though currently only windows supports the `advanced` feature.

you can make a pull request if you have an implementation of anything, or make an issue if you have a bug report or feature request.

on linux you will need [`libspeechd-dev`](https://github.com/brailcom/speechd), `libclang1`, and `clang`. view the repo for speechd's build instructions, but you can also check your package manager, e.g. `sudo apt install libspeechd-dev libclang1 clang` on a debian/ubuntu based distro.
