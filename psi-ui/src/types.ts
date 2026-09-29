export type BoardId = string;

export interface Board {
    id: BoardId;
    x: number;
    y: number;
    rotation: number;
}

export interface MapState {
    boards: Board[];
}

export interface InteractionState {
    heldBoard: BoardId | null;
    dragOffset: { x: number; y: number } | null;
}
