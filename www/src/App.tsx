import { Box, Grid } from "@mui/material";
import React, { useState } from "react";
import { AgentsForm } from "./components/agentsForm/agentsForm";
import { RankingTable } from "./components/rankingTable/rankingTable";
import { WorldCanvas } from "./components/WorldCanvas/worldCanvas";
import { WorldForm } from "./components/worldFrom/worldForm";
import {
  //DefaultAgentsSettings,
  IAgentsSettings,
} from "./components/agentsForm/IAgentsSettings";
import {
  //DefaultWorldSettings,
  IWorldSettings,
} from "./components/worldFrom/IWorldSettings";

//import { World } from "./components/world/world";

//const agentsSettings: IAgentsSettings = DefaultAgentsSettings();
//const worldSettings: IWorldSettings = DefaultWorldSettings();
//let world: World;

export function initWorld(): void {
  console.log("intializing world");
  //world = new World(worldSettings, agentsSettings);
}

function App() {
  const [raysCount, setRaysCount] = useState<number>(5);
  const [raysLength, setRaysLength] = useState<number>(120);
  const [raysSpread, setRaysSpread] = useState<number>(0.75);
  const [hiddenLayers, setHiddenLayers] = useState<Uint32Array>(
    new Uint32Array()
  );

  const [lanesCount, setLanesCount] = useState<number>(3);
  const [laneIndex, setLaneIndex] = useState<number>(1);
  const [agentsCount, setAgentsCount] = useState<number>(500);
  const [mutationRate, setMutationRate] = useState<number>(0.9);

  const simulationSettings: IAgentsSettings & IWorldSettings = {
    raysCount,
    raysLength,
    raysSpread,
    hiddenLayers,
    onRaysCountChange: (n: number): void => {
      setRaysCount(n);
    },
    onRaysLengthChange: (n: number): void => {
      setRaysLength(n);
    },
    onRaysSpreadChange: (n: number): void => {
      setRaysSpread(n);
    },
    onHiddenLayersChange: (a: Uint32Array): void => {
      setHiddenLayers(a);
    },

    lanesCount,
    laneIndex,
    agentsCount,
    mutationRate,
    onLanesCountChange: (n: number): void => {
      setLanesCount(n);
    },
    onLaneIndexChange: (n: number): void => {
      setLaneIndex(n);
    },
    onAgentsCountChange: (n: number): void => {
      setAgentsCount(n);
    },
    onMutationRateChange: (n: number): void => {
      setMutationRate(n);
    },
  };

  /*   const worldSettings = {
    lanesCount,
    laneIndex,
    agentsCount,
    mutationRate,
    onLanesCountChange: (n: number): void => {
      setLanesCount;
    },
    onLaneIndexChange: (n: number): void => {
      setLaneIndex;
    },
    onAgentsCountChange: (n: number): void => {
      setAgentsCount;
    },
    onMutationRateChange: (n: number): void => {
      setMutationRate;
    },
  }; */

  return (
    <Grid
      container
      sx={{
        minHeight: "100vh",
        display: "flex",
        flexDirection: "row",
        backgroundColor: "gray",
      }}
      spacing={1}
    >
      <Grid item xs={8} md={8} lg={8} sx={{ backgroundColor: "gold" }}>
        <Grid
          container
          sx={{
            minHeight: "100vh",
            display: "flex",
            flexDirection: "column",
            backgroundColor: "cyan",
            alignContent: "stretch",
            justifyContent: "center",
          }}
        >
          <Grid item sx={{ backgroundColor: "yellowgreen", minHeight: "40vh" }}>
            <Grid container spacing={1}>
              <Grid item xs={4}>
                <Box sx={{ backgroundColor: "limegreen" }}>
                  <RankingTable />
                </Box>
              </Grid>
              <Grid item xs={4}>
                <Box sx={{ backgroundColor: "greenyellow" }}>
                  <WorldForm {...simulationSettings} />
                </Box>
              </Grid>
              <Grid item xs={4}>
                <Box sx={{ backgroundColor: "lime" }}>
                  <AgentsForm />
                </Box>
              </Grid>
            </Grid>
          </Grid>
          <Grid item sx={{ backgroundColor: "yellow", minHeight: "60vh" }}>
            NeuralNetwor
          </Grid>
        </Grid>
      </Grid>
      <Grid item xs={4} md={4} lg={4} sx={{ backgroundColor: "purple" }}>
        <WorldCanvas {...simulationSettings} />
      </Grid>
    </Grid>
  );
}

export default App;

//{...world}
/* 
lanesCount={3}
                    laneIndex={1}
                    agentsCount={500}
                    mutationRate={0.75} */
