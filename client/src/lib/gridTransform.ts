export type GridTransform = {
  originX: number;
  originY: number;
  columnStepX: number;
  columnStepY: number;
  rowStepX: number;
  rowStepY: number;
};

export type ScenePoint = { x: number; y: number };
export type GridCoordinate = { column: number; row: number };

export function gridToScene(transform: GridTransform, column: number, row: number): ScenePoint {
  return {
    x: transform.originX + column * transform.columnStepX + row * transform.rowStepX,
    y: transform.originY + column * transform.columnStepY + row * transform.rowStepY
  };
}

export function sceneToGrid(transform: GridTransform, x: number, y: number): GridCoordinate {
  const dx = x - transform.originX;
  const dy = y - transform.originY;
  const determinant = transform.columnStepX * transform.rowStepY - transform.rowStepX * transform.columnStepY;
  if (Math.abs(determinant) < 0.0001) throw new Error('Grid vectors are parallel and cannot be inverted.');
  return {
    column: (dx * transform.rowStepY - transform.rowStepX * dy) / determinant,
    row: (transform.columnStepX * dy - dx * transform.columnStepY) / determinant
  };
}

export function isInvertible(transform: GridTransform): boolean {
  return Math.abs(transform.columnStepX * transform.rowStepY - transform.rowStepX * transform.columnStepY) >= 0.0001;
}

export function gridCellCorners(transform: GridTransform, column: number, row: number): ScenePoint[] {
  return [
    gridToScene(transform, column - 0.5, row - 0.5),
    gridToScene(transform, column + 0.5, row - 0.5),
    gridToScene(transform, column + 0.5, row + 0.5),
    gridToScene(transform, column - 0.5, row + 0.5)
  ];
}
