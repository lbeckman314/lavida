/* tslint:disable */
/**
*/
export enum Cell {
  Dead,
  Alive,
}
/**
*/
export class Universe {
  free(): void;
  tick(): void;
  static new(): Universe;
  set_size(width: number, height: number): Universe;
  render(): string;
  width(): number;
  height(): number;
  cells(): number;
  set_width(width: number): void;
  set_height(height: number): void;
  toggle_cell(row: number, col: number): void;
  reset(): void;
  clear(): void;
  glider(row: number, col: number): void;
  pulsar(row: number, col: number): void;
  cells_from_pattern(arr: Uint32Array, min: number, max: number, row_translate: number, col_translate: number, limit: number): void;
}
