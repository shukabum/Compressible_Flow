# Isentropic Flow Calculator

This repository hosts an Isentropic Flow Calculator application that calculates and visualizes the isentropic flow through a converging nozzle. It allows users to input parameters such as initial conditions and nozzle geometry to obtain a visualization of how Mach number, temperature, and pressure vary along the length of the nozzle.

## Features

- **Input Parameters**: Users can input initial conditions like inlet Mach number, inlet temperature, inlet pressure, and geometry parameters such as throat area and exit area.
- **Graphical Output**: The application generates a graph illustrating the variation of Mach number, temperature, and pressure along the length of the nozzle based on the provided input parameters.
- **React Frontend**: The frontend of the application is developed using React, providing a smooth and interactive user interface.
- **Rust Backend**: The backend logic is implemented in Rust, ensuring fast computation and efficient handling of calculations.

## Usage

To run the Isentropic Flow Calculator locally:

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Install dependencies for both the frontend and backend:

```bash
cd frontend
npm install
cd ../backend
cargo build
