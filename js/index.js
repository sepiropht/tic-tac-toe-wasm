import React from 'react';
import ReactDOM from 'react-dom';
import("../crate/pkg").then(({Board, Ai}) => {
  ReactDOM.render(
      <div>Hello Wasm!</div>,
      document.getElementById('app')
  )
  const board = new Board(3);
  const ai = Ai;
  console.log(board, ai);
});
