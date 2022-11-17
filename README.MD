# ENV MANAGER

This is a simple env manager written on Rust. 

It allows you switch between several `.env` files. 

# How To

## Install

WIP

## Switch

Switch option allows you to switch between several env files. The utility takes the chosen env and copies in the root of your application as `.env`

1. Create folder ```env```.
2. Place env files inside the folder following the next name convention: ```.{name}.env``` For example: ```.superbrand.env``` and ```.anothersuperbrand.env```.
3. Run ```cargo run``` within your app root directory
4. Follow the prompt within your terminal

### Terminal preview:

<div style="width:650px">

![Terminal](readme/terminal.png?raw=true "Terminal")

</div>



### Folder example:
<div style="margin-top:10px; width:250px">

![Folder](readme/folder.png?raw=true "Folder")

</div>


## Shortcuts:

**Options**

- ```-- switch``` - starts switching process
- ```-- test``` - starts testing process

WIP.
Switch also contains a second argument, where you can specify known name of env you want to use. For example: ```cargo run -- switch -- superbrand```

## Test

WIP


