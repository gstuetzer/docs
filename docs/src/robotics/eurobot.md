# Eurobot

`robotic`

[Homepage](https://blog.florencepaul.com/creating-a-robot-for-eurobot-part-1-context)
[Github](https://github.com/gbip?tab=repositories)

## Creating a robot for Eurobot.

This is my first post to announce an upcoming serie of posts about creating a robot in Rust.
I am part of a team of students participating in Eurobot and this year we have decided that our firmware will be written in Rust. The goal is to write most of it in Rust but if we lack some time we will use C. I will also write about both higher level concepts (AI) and lower level concepts (hardware design).
I will try to use this blog as a logbook for our organization and for tutorials.
But first, why do we build this robot ?

## Hardware

Embedded Hardware: `Nucleo-f446RE`  `stm32f446RE`
The Raspberry PI communicates through a serial (or USART) link with an electronic card. We use a Nucleo f446RE which is based on the stm32f446RE microcontroller. It features :

* Cortex M4 running @ 180 MHz with an FPU
* 4 USART and 2 UART
* 3 CAN
* More timers than we will ever use