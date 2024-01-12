import "./App.css";
import MandelbrotCanvas from "./components/mandelbrotcanvas";

function App() {
  return (
    <div className="App-body">
      <MandelbrotCanvas
        api="get" // "websocket" or else runs get
        width={800}
        height={500}
        url="localhost"
        port="8080"
        className="App-canvas"
      ></MandelbrotCanvas>
    </div>
  );
}

export default App;
