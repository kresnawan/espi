# espi

A simple program to generate bare-minimum C project folder for ESP8266-RTOS-SDK development, specifically on VSCode.

## Installation
Simply download the binary from release page and add it to your `/bin` directory, it is supported in all linux distros since it has no dependency other than glibc. The binary in the release page is compiled with target `x86_64-unknown-linux-musl`. Or, compile the source code yourself.

## Usage
Like how we usually make a project directory:
```bash
espi project_name
```
Or with specific location:
```bash
espi path/to/your/project
```