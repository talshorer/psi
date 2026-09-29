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
