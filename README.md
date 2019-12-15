# Valo: A Program to Control Backlights (and other Hardware Lights) in GNU/Linux

- [Introduction](#introduction)
- [Usage](#usage)

## Introduction

Valo is a program to control backlights and other lights under GNU/Linux.

## Usage

Get the current `keyboard` backlight brightness

    valo keyboard get

Get the current `keyboard` backlight brightness in percent

    valo keyboard get-percentage

Increase `keyboard` backlight brightness by 5 percent

    valo keyboard up 5

Decrease `keyboard` backlight brightness by 5 percent

    valo keyboard down 5

Set `keyboard` backlight brightness as max

    valo keyboard max

Turn off `keyboard` backlight

    valo keyboard off

Set `keyboard` backlight brightness as 42 percentage

    valo keyboard set-percentage 42
