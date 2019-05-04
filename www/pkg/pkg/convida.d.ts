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
  static new(width: number, height: number): Universe;
  static mynew(): Universe;
  set_size(width: number, height: number): Universe;
  render(): string;
  width(): number;
  height(): number;
  cells(): number;
  set_width(width: number): void;
  set_height(height: number): void;
  toggle_cell(row: number, column: number): void;
  reset(): void;
  clear(): void;
  glider(row: number, column: number): void;
  pulsar(row: number, column: number): void;
}
