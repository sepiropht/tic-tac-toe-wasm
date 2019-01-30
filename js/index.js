import React from "react";
import ReactDOM from "react-dom";
import BoardJS from "./board";
import aiJS from "./ai";
import { useState } from "react";

import("../crate/pkg").then(({ Board, Ai }) => {
  let ai;
  class TicTacToe extends React.Component {
    constructor(props) {
      super(props);
      let board = [];
      this.board = new Board(props.width);
      for (let i = 0; i < props.width; i++) {
        let row = [];
        for (let j = 0; j < props.width; j++) {
          row.push(0);
        }
        board.push(row);
      }
      ai = props.isWasm ? new Ai() : aiJS;
      this.state = {
        player: 1,
        freezeBoard: false,
        winner: false,
        board,
        isWasm: props.isWasm
      };
    }

    nextPlayer() {
      return this.state.player === 1 ? 2 : 1;
    }

    // Place a move on the board and check for a winner.
    move(x, y, player, callback) {
      this.board.playerMove(x, y, player);
      const winner = this.board.checkWin();
      if (isNaN(winner)) {
        winner = winner === 0;
      }
      if (winner) {
        this.setState({ winner, freezeBoard: true });
      } else {
        callback();
      }
    }

    // Handle a player's move, and switch to the next player.
    playerMove(event) {
      let [x, y] = event.target.dataset.cell.split("_");
      x = parseInt(x);
      y = parseInt(y);
      //console.log('getCell', this.board.getCell(x, y))
      const cellEmpty = this.board.getCell(x, y) === 0;
      if (cellEmpty) {
        this.move(x, y, this.state.player, () => {
          this.state.board[x][y] = this.state.player;
          if (this.props.singlePlayer) {
            this.setState(
              { player: this.nextPlayer(), freezeBoard: true },
              this.aiMove
            );
          } else {
            this.setState({ player: this.nextPlayer() });
          }
        });
      }
    }

    // Make an AI move, with a small delay for a more natural response time.
    aiMove() {
      //console.log("BBBBBBBBoard", board);
      const point = ai.aiMove(this.board, this.state.player);
      let x, y;
      if (this.props.isWasm) {
        x = point.getX();
        y = point.getY();
      } else {
        [x, y] = point;
      }
      this.state.board = this.state.board.map((rows, rowind) =>
        rows.map((cell, cellind) =>
          x === rowind && y === cellind ? this.state.player : cell
        )
      );
      this.setState(({ board }) => ({
        board: board.map((rows, rowind) =>
          rows.map((cell, cellind) =>
            x === rowind && y === cellind ? this.state.player : cell
          )
        )
      }));
      //this.state.board[x][y] = this.state.player;
      this.move(x, y, this.state.player, () => {
        this.setState({ player: this.nextPlayer(), freezeBoard: false });
      });
    }

    // Determine which player will be the AI in single player mode,
    // and make the first move if appropriate.
    aiInit() {
      if (this.props.singlePlayer) {
        const aiPlayer = Math.floor(Math.random() * 2) + 1;
        if (aiPlayer === 1) {
          this.aiMove();
        }
      }
    }

    reset() {
      this.board = this.props.isWasm
        ? new Board(this.props.width)
        : new BoardJS(this.props.width);
      const board = [];

      for (let i = 0; i < this.props.width; i++) {
        let row = [];
        for (let j = 0; j < this.props.width; j++) {
          row.push(0);
        }
        board.push(row);
      }
      this.state.board = board;
      this.setState(({ player, freezeBoard, winner, board }) => ({
        player: 1,
        freezeBoard: false,
        winner: false,
        board
      }));
      this.aiInit();
    }
    componentDidUpdate(prevProps, prevState, snapshot) {
      if (this.props.isWasm !== prevProps.isWasm) {
        ai = this.props.isWasm ? new Ai() : aiJS;
        this.reset();
      }
    }

    componentDidMount() {
      this.aiInit();
    }

    render() {
      let board;
      if (this.props.isWasm) {
        board = this.state.board;
      } else {
        board = this.board.board || [];
      }
      let announcement;

      if (this.state.winner) {
        const msg =
          this.state.winner > 2
            ? "It's a tie!"
            : `Player ${this.state.winner} wins!`;
        announcement = (
          <div className="announcement">
            <p>{msg}</p>
            <button onClick={this.reset.bind(this)}>Reset</button>
          </div>
        );
      }
      //console.log({board})
      const grid = board.map((row, rowInd) => {
        const cells = row.map((cell, cellInd) => {
          const classString =
            cell > 0 ? (cell === 1 ? "cell-p1" : "cell-p2") : "cell";
          const coords = `${rowInd}_${cellInd}`;
          let clickHandler;

          if (!this.state.freezeBoard) {
            clickHandler = this.playerMove.bind(this);
          }

          return (
            <div
              className={classString}
              key={cellInd}
              onClick={clickHandler}
              data-cell={coords}
            />
          );
        });

        return (
          <div className="row" key={rowInd}>
            {cells}
          </div>
        );
      });

      return (
        <div className="grid">
          {grid}
          {announcement}
        </div>
      );
    }
  }

  function App() {
    // Set Wasm by default
    const [selectedOption, setOption] = useState("wasm");
    const handleChange = e => setOption(e.target.value);

    // 3*3 grid by default
    const [width, setWidth] = useState(3);
    const handleChangeWidth = e =>
      e.target.value > 2 && e.target.value < 11
        ? setWidth(parseInt(e.target.value))
        : width;
    const Game = (
      <TicTacToe
        width={width}
        isWasm={selectedOption === "wasm"}
        singlePlayer={true}
      />
    );
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
        {Game}
      </>
    );
  }

  ReactDOM.render(<App />, document.getElementById("app"));
});
