import React from 'react';
import ReactDOM from 'react-dom';
import("../crate/pkg").then(module => {
  ReactDOM.render(
      <div>Hello Wasm!</div>,
      document.getElementById('app')
  )
  debugger;
  const board = module.Board.new(3);
  const ai = module.Ai;
});
