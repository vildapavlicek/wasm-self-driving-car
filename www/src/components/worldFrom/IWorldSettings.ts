export interface IWorldSettings {
  lanesCount: number;
  laneIndex: number;
  agentsCount: number;
  mutationRate: number;
  onLanesCountChange: (n: number) => void;
  onLaneIndexChange: (n: number) => void;
  onAgentsCountChange: (n: number) => void;
  onMutationRateChange: (n: number) => void;
}

/* export const DefaultWorldSettings = (): IWorldSettings => {
  return {
    lanesCount: 3,
    laneIndex: 1,
    agentsCount: 500,
    mutationRate: 0.75,
  };
}; */
