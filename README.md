# Simple Pendulum in Rust

This repository contains a Rust implementation of a simple pendulum using the `speedy2d` crate for graphics. The project demonstrates how to create a window, handle window events, update and draw the pendulum, and perform vector operations.

## Dependencies

To run this project, you'll need the following dependencies:

- Rust programming language
- `speedy2d` crate

## How to Run

To run the project, navigate to the project directory in the terminal and execute the following command:

```
cargo run
```

This will compile and run the project, opening a window with the simple pendulum simulation.

## Overview

The project is structured as follows:

- `main()` function: Sets up the window and runs the main event loop
- `MyWindowHandler` struct: Handles window events and contains the pendulum objects
- `Pendulum` struct: Represents a pendulum with position, angle, angular velocity, angular acceleration, length, mass, and gravity
- `vector` module: Contains the `Vector` struct used for position calculations

## Customization

You can customize the pendulum simulation by modifying the following parameters in the `Pendulum` struct:

- `r`: Length of the pendulum
- `m`: Mass of the ball
- `g`: Gravity value (increase or decrease to see the effect on the pendulum)

Feel free to experiment with different values to observe their impact on the pendulum's behavior.

## License

This project is provided under the MIT License. See [LICENSE](LICENSE) for more information.
