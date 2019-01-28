import React from "react";
import ReactDOM from "react-dom";
import Board from "./board";
import TicTacToeJS from "./ui";
import ai from "./ai";
import { useState } from "react";

import("../crate/pkg").then(({ Board, Ai }) => {
  const ai = new Ai();

  class TicTacToe extends React.Component {
    constructor(props) {
      super(props);
      let board = [];
      this.board = new Board(props.width);
      for (let i = 0; i < props.width; i++) {
        let row = [];
        for(let j = 0; j < props.width; j++) {
            row.push(0);
        }
        board.push(row);
      }
      this.state = { player: 1, freezeBoard: false, winner: false, board };
    }

    nextPlayer() {
      return this.state.player === 1 ? 2 : 1;
    }

    // Place a move on the board and check for a winner.
    move(x, y, player, callback) {
      this.board.playerMove(x, y, player);
      const winner = this.board.checkWin();

      if (winner !== 0) {
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
      const point = ai.aiMove(this.board, this.state.player);
      let x = point.getX();
      let y = point.getY();
      this.state.board[x][y] = this.state.player;
      setTimeout(() => {
        this.move(x, y, this.state.player, () => {
          this.setState({ player: this.nextPlayer(), freezeBoard: false });
        });
      }, 200);
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
      this.board = new Board(this.props.width);
      const board = [];
      for (let i = 0; i < props.width; i++) {
        let row = [];
        for(let j = 0; j < props.width; j++) {
            row.push(0);
        }
        board.push(row);
      }
      this.setState({ player: 1, freezeBoard: false, winner: false, board });
      this.aiInit();
    }

    componentDidMount() {
      this.aiInit();
    }

    render() {
      const { board } = this.state;
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
          // console.log('aa', cell, cellInd)
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
    const handleChangeWidth = e => setWidth(parseInt(e.target.value));

    const Game =
      selectedOption === "wasm" ? (
        <TicTacToe width={width} singlePlayer={true} />
      ) : (
        <TicTacToeJS width={width} singlePlayer={true} />
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
                  onChange={handleChangeWidth} />
                  </label>
              </div>
        </form>
        {Game}
      </>
    );
  }

  ReactDOM.render(<App/>, document.getElementById("app"));
});
