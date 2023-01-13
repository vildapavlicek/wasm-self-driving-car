import { Box, Grid, TextField } from "@mui/material";
import React, { FC, ReactElement, useState } from "react";
import { IWorldSettings } from "./IWorldSettings";

export const WorldForm: FC<IWorldSettings> = (
  props: IWorldSettings
): ReactElement => {
  const {
    lanesCount,
    laneIndex,
    agentsCount,
    mutationRate,
    onLanesCountChange,
    onLaneIndexChange,
    onAgentsCountChange,
    onMutationRateChange,
  } = props;

  return (
    <Grid container>
      <Grid item xs={12} md={12} lg={12}>
        <h3>World Configuration Options</h3>
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Number of lanes
        <TextField
          fullWidth
          size="small"
          defaultValue={lanesCount}
          helperText="Number of Lanes drawn on the street"
          //onChange={updateLanesCount}
        />
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Starting Lane Index
        <TextField
          fullWidth
          size="small"
          defaultValue={laneIndex}
          helperText="First lane is left one with index of 0"
        />
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Number of Agents
        <TextField
          fullWidth
          size="small"
          defaultValue={agentsCount}
          helperText="Sets how many agent's will be spawn on the world"
        />
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Mutation Rate
        <TextField
          fullWidth
          size="small"
          defaultValue={mutationRate}
          helperText="When spawning new agents we will randomly mutate all values"
        />
      </Grid>
    </Grid>
  );
};
