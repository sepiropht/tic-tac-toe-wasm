export default function reducer(state, action) {
  switch (action.type) {
    case "updateBoard":
      const { x, y, player } = action.payload;
      return {
        ...state,
        board: state.board.map((rows, rowind) =>
          rows.map((cell, cellind) =>
            x === rowind && y === cellind ? player : cell
          )
        )
      };
    case "toogleFreezeBoard":
      return {
        ...state,
        freezeBoard: action.payload
      };
    case "setWinner":
      return {
        ...state,
        winner: action.payload
      };
    /*case "nextPlayer":
      return {
        ...state,
        player: state.player === 1 ? 2 : 1
      };
    */
    case "initboard":
      return {
        ...state,
        board: action.payload
      };
    case "init": {
      return {
        ...state,
        board: action.payload,
        winner: 0,
        freezeBoard: false
      };
    }
  }
  return state;
}


