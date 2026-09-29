import { mapState } from "./state";
import { renderBoards } from "./map";
import "./interaction";

renderBoards(mapState.boards);
