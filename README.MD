# ENV MANAGER

This is a simple env manager written on Rust. It allows you switch between several .env files and apply rules for testing separate envs. 

# How To

## Install

WIP

## Switch

Switch option allows you to switch between several env files. The utility takes chose env and copies into the main ```.env``` file in the root of your application.

1. Create folder ```env```.
2. Place env files inside the folder following the next name convention: ```.{name}.env``` For example: ```.superbrand.env``` and ```.anothersuperbrand.env```.
3. Run ```cargo run``` within your app root directory
4. Follow the prompt within your terminal

If you want you can use shortcuts to 

**Options**

```-- switch``` - starts switching process
```-- test``` - starts testing process

Switch also contains a second argument, where you can specify known name of env you want to use. For example: ```cargo run -- switch -- superbrand```

## Test

WIP


