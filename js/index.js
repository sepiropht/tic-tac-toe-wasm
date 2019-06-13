import React from "react";
import ReactDOM from "react-dom";
import BoardJS from "./board";
import aiJS from "./ai";
import { useState } from "react";
import "babel-polyfill";

(async () => {

  const im = await import("./tictactoe");
  const mod = await im.default;
  const TicTacToe = mod.TicTacToe;

  function App() {
    // Set Wasm by default
    const [selectedOption, setOption] = useState("wasm");
    const [dumb, setDumb] = useState(0);
    const handleChange = e => setOption(e.target.value);
    const [width, setWidth] = useState(10);
    const handleChangeWidth = e =>
      e.target.value > 2 && e.target.value < 11
        ? setWidth(parseInt(e.target.value))
        : width;

    function reset() {setDumb(dumb + 1)}
    return (
      <>
        <h1> TicTacToe in wasm </h1>
        <form>
          Choose an implementation in JavaScript or Rust/Wasm
          <div className="radio">
            <label>
              <input
                type="radio"
                value="wasm"
                checked={selectedOption === "wasm"}
                onChange={handleChange}
              />
              Wasm
            </label>
          </div>
          <div className="radio">
            <label>
              <input
                type="radio"
                value="js"
                checked={selectedOption === "js"}
                onChange={handleChange}
              />
              JavaScript
            </label>
          </div>
          <div>
            <h2> Size of the board </h2>
            <label>
              Size of the board:
              <input
                name="width"
                type="number"
                value={width}
                onChange={handleChangeWidth}
              />
            </label>
          </div>
        </form>
        <TicTacToe
          width={width}
	  dumb={dumb}
          isWasm={selectedOption === "wasm"}
          reset={reset}
          singlePlayer={true}
        />{" "}
      </>
    );
  }

  ReactDOM.render(<App />, document.getElementById("app"));
})();
