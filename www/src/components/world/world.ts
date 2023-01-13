import { Config, Simulation } from "wasm-self-driving-car";
import { IWorldSettings } from "../worldFrom/IWorldSettings";
import { IAgentsSettings } from "../agentsForm/IAgentsSettings";

export class World {
  simulation: Simulation;

  constructor(worldConfig: IWorldSettings, agentsConfig: IAgentsSettings) {
    console.log("creating config");
    const config: Config = new Config(
      worldConfig.lanesCount,
      worldConfig.laneIndex,
      worldConfig.agentsCount,
      agentsConfig.raysCount,
      agentsConfig.raysLength,
      agentsConfig.raysSpread,
      agentsConfig.hiddenLayers,
      worldConfig.mutationRate
    );

    console.log("creating simulation");
    this.simulation = new Simulation(500, window, config);
  }
}
