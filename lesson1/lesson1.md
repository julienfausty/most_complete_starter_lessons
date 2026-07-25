# Lesson 1 - Installing libraries

This lesson is mostly about setting up the serial monitor. We can just put our user into the `uucp` group for interacting with serial ports in archlinux.

## Add user to uucp

```shell
sudo usermod -a -G uucp $USER
```
