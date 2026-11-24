import type { Board } from "./types";

const SVG_NS = "http://www.w3.org/2000/svg";

function createBoardElement(board: Board, svgMarkup: string): SVGGElement {
    const group = document.createElementNS(SVG_NS, "g");
    group.dataset.boardId = board.id;
    group.classList.add("board");

    const sourceDoc = new DOMParser().parseFromString(
        svgMarkup,
        "image/svg+xml",
    );
    const root = sourceDoc.documentElement as unknown as SVGSVGElement;
    if (root.localName !== "svg") {
        throw new Error("Could not parse rendered SVG");
    }
    const { width, height } = root.viewBox.baseVal;

    const image = document.createElementNS(SVG_NS, "image");
    image.setAttribute("href", URL.createObjectURL(new Blob([svgMarkup], { type: "image/svg+xml" })));
    image.setAttribute("x", `${-width / 2}`);
    image.setAttribute("y", `${-height / 2}`);
    image.setAttribute("width", `${width}`);
    image.setAttribute("height", `${height}`);
    group.appendChild(image);

    updateBoardElement(group, board);
    return group;
}

function updateBoardElement(group: SVGGElement, board: Board): void {
    group.setAttribute(
        "transform",
        `translate(${board.x} ${board.y}) rotate(${board.rotation})`,
    );
}

export function renderBoards(boards: Board[], renderSvg: (board: Board) => string): void {
    const svg = document.querySelector<SVGSVGElement>("#boards");

    if (!svg) {
        throw new Error("Could not find #boards");
    }

    const oldUrls = [...svg.querySelectorAll("image")]
        .map((image) => image.getAttribute("href"))
        .filter((url): url is string => url?.startsWith("blob:") ?? false);

    svg.replaceChildren();

    for (const url of oldUrls) {
        URL.revokeObjectURL(url);
    }

    for (const board of boards) {
        svg.appendChild(createBoardElement(board, renderSvg(board)));
    }
}


export function getMapElement(): SVGSVGElement {
    const svg = document.querySelector<SVGSVGElement>("#map");

    if (!svg) {
        throw new Error("Could not find #map");
    }

    return svg;
}

export function updateBoard(board: Board): void {
    const group = getMapElement().querySelector<SVGGElement>(
        `[data-board-id="${board.id}"]`,
    );

    if (!group) {
        throw new Error(`Could not find board ${board.id}`);
    }

    updateBoardElement(group, board);
}

export function setBoardHeld(boardId: string, held: boolean): void {
    const group = getMapElement().querySelector<SVGGElement>(
        `[data-board-id="${boardId}"]`,
    );

    if (!group) {
        return;
    }

    group.classList.toggle("held", held);
}
