# COSMIC Application

## TODO
- [x] Send control info to the device
    - [x] Handle the device better, store a ref in the app state?
    - [ ] Handle other control types
        - [x] Bool
        - [ ] Menu
- [ ] Figure out scrolling the viewport
    - [ ] Figure out wonky slider stuff
- [ ] Persist the settings
- [ ] Close to systray
- [ ] Show video feed
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
