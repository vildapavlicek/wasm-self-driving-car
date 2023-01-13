export interface IAgentsSettings {
  raysCount: number;
  raysLength: number;
  raysSpread: number;
  hiddenLayers: Uint32Array;
  onRaysCountChange: (n: number) => void;
  onRaysLengthChange: (n: number) => void;
  onRaysSpreadChange: (n: number) => void;
  onHiddenLayersChange: (a: Uint32Array) => void;
}

/* export const DefaultAgentsSettings = (): IAgentsSettings => {
  const hiddenLayers = new Uint32Array(3);
  hiddenLayers.set([8, 5]);

  return {
    raysCount: 5,
    raysLength: 120.0,
    raysSpread: 0.75,
    hiddenLayers,
  };
}; */
