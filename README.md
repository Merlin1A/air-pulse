# Air-Pulse: Real-time Air Quality Monitoring & Alerting

> Air-Pulse is a real-time air quality monitoring and alerting system written in Rust. The project aims to provide users with timely and accurate air quality information based on their location. It leverages the AirNow API for data and Twilio's SMS API for alerting, ensuring a reliable and scalable solution.


## Features

- Real-time air quality data fetching from the Google Air Quality API
- Customizable air quality alerts based on user-defined thresholds
- SQLite-backed data storage for user preferences and historical air quality data
- Forecast accuracy calculation for better reliability
- SMS-based user interaction for commands, alerts, and location 

## Supported Platforms

- Linux

## Installation

You can compile it from the source if your machine is installed with [Rust](https://www.rust-lang.org/en-US/install.html).

```bash
$ git clone https://github.com/YourUsername/air-pulse.git
$ cd air-pulse
$ cargo build --release
