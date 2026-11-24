export type BoardId = string;

export type LayoutId = "A" | "B";

export interface Board {
    id: BoardId;
    layout: LayoutId;
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
