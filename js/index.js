import React from 'react';
import ReactDOM from 'react-dom';
import("../crate/pkg").then(({Board, Ai}) => {
  ReactDOM.render(
      <div>Hello Wasm! </div>,
      document.getElementById('app')
  )
  const board = new Board(3);
  const ai =  new Ai();
  debugger
  ai.aiMove(board, 1);
  console.log(board, ai);
});
