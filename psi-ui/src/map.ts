import type { Board } from "./types";

const SVG_NS = "http://www.w3.org/2000/svg";

const BOARD_RADIUS = 150;

const SIN_60 = Math.sin(Math.PI / 3);
const COS_60 = Math.cos(Math.PI / 3);
const SQRT3 = Math.sqrt(3);
const SIN_60_REDUCED = SIN_60 / SQRT3;
const COS_60_REDUCED = COS_60 / SQRT3;

const BOARD_POLYGON_POINTS = [
    [-COS_60_REDUCED * BOARD_RADIUS, SIN_60_REDUCED * BOARD_RADIUS],
    [SIN_60 * BOARD_RADIUS, COS_60 * BOARD_RADIUS],
    [COS_60_REDUCED * BOARD_RADIUS, -SIN_60_REDUCED * BOARD_RADIUS],
    [-SIN_60 * BOARD_RADIUS, -COS_60 * BOARD_RADIUS],
];

function createBoardElement(board: Board): SVGGElement {
    const group = document.createElementNS(SVG_NS, "g");
    group.dataset.boardId = board.id;
    group.classList.add("board");

    const polygon = document.createElementNS(SVG_NS, "polygon");

    const points = BOARD_POLYGON_POINTS.map(([x, y]) => `${x},${y}`).join(" ");

    polygon.setAttribute("points", points);
    group.appendChild(polygon);

    updateBoardElement(group, board);

    return group;
}

function updateBoardElement(group: SVGGElement, board: Board): void {
    group.setAttribute(
        "transform",
        `translate(${board.x} ${board.y}) rotate(${board.rotation})`,
    );
}

export function renderBoards(boards: Board[]): void {
    const svg = document.querySelector<SVGSVGElement>("#boards");

    if (!svg) {
        throw new Error("Could not find #boards");
    }

    svg.replaceChildren();

    for (const board of boards) {
        svg.appendChild(createBoardElement(board));
    }
}
