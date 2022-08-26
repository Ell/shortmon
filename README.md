# Shortmon

## What is it?

Easily switch between monitor inputs with the press of a button(s) or click

![Monitor list](https://i.ell.dev/uIZzw7w1.png)

## Linux setup

On Linux, monitor detection and control is done via I2C. In order for I2C devices to be available it may be necessary to explicitly load the appropriate kernel module (e.g. via `# modprobe i2c-dev` or adding that module to a configuration file for automatic loading).

## Building

Requirments:

-   Rust
-   Node

```
> npm install
> npm run tauri dev
```
