import { mapState } from "./state";
import { renderBoards } from "./map";
import "./interaction";
import initializeWasm, { layout_svg } from "./wasm/pkg/psi_wasm.js"

await initializeWasm()
renderBoards(mapState.boards, (board) => layout_svg({ layout: board.layout, scale: 150 }));
