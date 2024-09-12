# COSMIC Application

## TODO
- [x] Send control info to the device
    - [x] Handle the device better, store a ref in the app state?
    - [ ] Handle other control types
        - [x] Bool
        - [x] Menu
        - [ ] Button?
- [x] Figure out scrolling the viewport
    - [x] Figure out wonky slider stuff
- [x] Persist the settings
    - [ ] Persist last state of the app?
        - [ ] Is there a better way to identify a video device?
            - Currently relying on the device path.
        - [ ] Load settings on start
    - [ ] Save settings named/profile?
        - [ ] Profile picker
- [ ] Reset to default control values
- [ ] Close to systray
- [ ] Show video feed
- [ ] Other things like colorspace, resolution
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
