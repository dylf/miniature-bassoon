# COSMIC Application

## TODO
- Controls
    - [x] Show control values
    - [x] Reset to default control values
        - [ ] Icon button?
    - [ ] Other things like colorspace, resolution
- Saving
    - [ ] Persist last state of the app?
        - [ ] Load settings on start
    - [ ] Save settings named/profile?
        - [ ] Profile picker
- App
    - [ ] Close to systray
    - [ ] Show video feed
- Project
    - [ ] GitHub actions
    - [ ] Test coverage
    - [ ] Update the README.md with correct info
    - [ ] Distribution stuff

## Install

To install your COSMIC application, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your application:

```sh
just build-release
sudo just install
```
