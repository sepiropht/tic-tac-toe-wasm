import React from "react";
import ReactDOM from "react-dom";
import BoardJS from "./board";
import aiJS from "./ai";
import reducer from "./reducer";
import { useState, useEffect, useRef, useReducer } from "react";

export default new Promise(async $export  => {
  const { Board, Ai } = await import("../crate/pkg");

  $export({TicTacToe})

  function TicTacToe({ isWasm, width, singlePlayer, reset , dumb}) {
    const modelBoard = useRef(isWasm ? new Board(width) : new BoardJS(width));
    const ai = useRef(isWasm ? new Ai() : aiJS);

    useEffect(() => {
      const board = [];
      for (let i = 0; i < width; i++) {
        let row = [];
        for (let j = 0; j < width; j++) {
          row.push(0);
        }
        board.push(row);
      }
      modelBoard.current = isWasm ? new Board(width) : new BoardJS(width);
      ai.current = isWasm ? new Ai() : aiJS;
      dispatch({ type: "init", payload: board });
      aiInit();
    }, [width, isWasm, ai, dumb]);

    const initialState = {
      isWasm,
      player: 1
    };

    const [state, dispatch] = useReducer(reducer, initialState);
    const [player, setPlayer] = useState(1);

    // Place a move on the board and check for a winner.
    function move(x, y, player, callback) {
      modelBoard.current.playerMove(x, y, player);

      let winner = modelBoard.current.checkWin();
      if (isNaN(winner)) {
        winner = winner === 0;
      }
      if (winner) {
        dispatch({ type: "setWinner", payload: winner });
        dispatch({ type: "toogleFreezeBoard", payload: true });
      } else {
        callback();
      }
    }

    // Handle a player's move, and switch to the next player.
    function playerMove(event) {
      let [x, y] = event.target.dataset.cell.split("_");
      x = parseInt(x);
      y = parseInt(y);

      const cellEmpty = modelBoard.current.getCell(x, y) === 0;
      if (cellEmpty) {
        move(x, y, player, () => {
          dispatch({
            type: "updateBoard",
            payload: { x, y, player }
          });
          if (singlePlayer) {
            setPlayer(player === 1 ? 2 : 1);
            dispatch({ type: "toogleFreezeBoard", payload: true });
            aiMove(player === 1 ? 2 : 1);
          } else {
            setPlayer(player === 1 ? 2 : 1);
            //dispatch({ type: "nextPlayer"});
          }
        });
      }
    }

    function aiMove(player = 1) {
      const point = ai.current.aiMove(modelBoard.current, player);

      let x, y;

      if (isWasm) {
        x = point.getX();
        y = point.getY();
      } else {
        [x, y] = point;
      }

      dispatch({
        type: "updateBoard",
        payload: { x, y, player }
      });

      move(x, y, player, () => {
        setPlayer(player === 1 ? 2 : 1);
        dispatch({ type: "toogleFreezeBoard", payload: false });
      });
    }

    // Determine which player will be the AI in single player mode,
    // and make the first move if appropriate.
    function aiInit() {
      if (singlePlayer) {
        const aiPlayer = Math.floor(Math.random() * 2) + 1;
        if (aiPlayer === 1) {
          aiMove(1);
        }
      }
    }

    let boardView = isWasm ? state.board || [] : modelBoard.current.board || [];
    let announcement;

    if (state.winner) {
      const msg =
        state.winner > 2 ? "It's a tie!" : `Player ${state.winner} wins!`;
      announcement = (
        <div className="announcement">
          <p>{msg}</p>
          <button onClick={reset}>Reset</button>
        </div>
      );
    }

    const grid = boardView.map((row, rowInd) => {
      const cells = row.map((cell, cellInd) => {
        const classString =
          cell > 0 ? (cell === 1 ? "cell-p1" : "cell-p2") : "cell";
        const coords = `${rowInd}_${cellInd}`;
        let clickHandler;
        if (!state.freezeBoard) {
          clickHandler = playerMove;
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
});
