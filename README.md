# Rust Full Stack Mandelbrot App

This repository contains the Rust Full Stack Mandelbrot App, an interactive web application for generating and displaying images of the Mandelbrot set. The application leverages Rust's performance for backend computations and React for a dynamic frontend user interface.

## Overview

The Mandelbrot set is a famous fractal known for its intricate and fascinating patterns. This application allows users to explore different views of the Mandelbrot set in real-time, with the ability to zoom in and out and navigate across the fractal landscape.

### Backend (Rust)

- **Framework**: Built with the Actix-Web framework, ensuring robust and efficient HTTP and WebSocket handling.
- **WebSocket Support**: Real-time communication with the frontend for streaming Mandelbrot set images.
- **Mandelbrot Generation**: Efficient computation of Mandelbrot sets with Rust.
- **Image Processing**: Conversion of fractal data into images.
- **RESTful API**: Endpoints for fetching Mandelbrot images with specific parameters.
- **CORS and Logging**: Configured for cross-origin resource sharing and equipped with logging for monitoring.

### Frontend (React)

- **Interactive UI**: Users can manipulate the view of the Mandelbrot set for a personalized experience.
- **WebSocket Integration**: Real-time data streaming from the backend.
- **Responsive Design**: Adapts to various devices and screen sizes.
- **State Management**: Manages dynamic data and UI state for a seamless experience.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust and Cargo (latest stable version)
- Node.js and npm

### Installing

1. **Clone the Repository**:

   ```
   git clone https://github.com/yourusername/rust-mandelbrot-app.git
   cd rust-mandelbrot-app
   ```

2. **Running the Backend**:

   Navigate to the backend directory and run:

   ```
   cd backend
   cargo run
   ```

3. **Running the Frontend**:

   In a separate terminal, navigate to the frontend directory:

   ```
   cd frontend
   npm install
   npm start
   ```

   Access the application at `http://localhost:3000`.

## Usage

- Explore different areas of the Mandelbrot set using mouse controls.
- Zoom in and out for detailed views.
- Adjust parameters to change the appearance of the fractal.

## Contributing

Contributions are welcome! For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.


