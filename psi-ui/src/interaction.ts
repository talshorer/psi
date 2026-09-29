import { interactionState, mapState } from "./state";
import { setBoardHeld as mapSetBoardHeld, getMapElement, updateBoard } from "./map";

const svg = getMapElement();

function setBoardHeld(boardId: string, held: boolean): void {
    interactionState.heldBoard = held ? boardId : null;
    svg.classList.toggle("dragging", held);
    mapSetBoardHeld(boardId, held);
}

function boardAtPoint(target: EventTarget | null): string | null {
    if (!(target instanceof Element)) {
        return null;
    }

    return target.closest<SVGGElement>(".board")?.dataset.boardId ?? null;
}

svg.addEventListener("pointerdown", (event) => {
    const boardId = boardAtPoint(event.target);

    if (!boardId) {
        return;
    }

    const board = mapState.boards.find((board) => board.id === boardId);

    if (!board) {
        return;
    }

    setBoardHeld(boardId, true);

    svg.setPointerCapture(event.pointerId);

    interactionState.dragOffset = {
        x: board.x - event.clientX,
        y: board.y - event.clientY,
    };
});

svg.addEventListener("pointermove", (event) => {
    const boardId = interactionState.heldBoard;

    if (!boardId) {
        return;
    }

    const board = mapState.boards.find((board) => board.id === boardId);

    if (!board) {
        return;
    }

    if (interactionState.dragOffset) {
        board.x = event.clientX + interactionState.dragOffset.x;
        board.y = event.clientY + interactionState.dragOffset.y;
        updateBoard(board);
    }
});

function releaseBoard(event: PointerEvent): void {
    const boardId = interactionState.heldBoard;

    if (!boardId) {
        return;
    }

    setBoardHeld(boardId, false);

    if (svg.hasPointerCapture(event.pointerId)) {
        svg.releasePointerCapture(event.pointerId);
    }

    interactionState.dragOffset = null;
}

svg.addEventListener("pointerup", releaseBoard);
svg.addEventListener("pointercancel", releaseBoard);

svg.addEventListener(
    "wheel",
    (event) => {
        const boardId = interactionState.heldBoard;

        if (!boardId) {
            return;
        }

        event.preventDefault();

        const board = mapState.boards.find((board) => board.id === boardId);

        if (!board) {
            return;
        }

        board.rotation += event.deltaY > 0 ? 15 : -15;
        updateBoard(board);
    },
    { passive: false },
);
