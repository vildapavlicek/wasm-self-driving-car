import React, { FC, ReactElement, useState } from "react";
import { Box, Grid, TextField } from "@mui/material";

export const AgentsForm: FC = (): ReactElement => {
  const [raysCount, setRaysCount] = useState<number>(5);
  const [raysLength, setRaysLength] = useState<number>(120);
  const [raysSpread, setRaysSpread] = useState<number>(0.75);
  const [hiddenLayers, setHiddenLayers] = useState<number>(5);

  return (
    <Grid container>
      <Grid item xs={12} md={12} lg={12}>
        <h3>Agents&apos; Configuration Options</h3>
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Number of Rays
        <TextField
          fullWidth
          size="small"
          inputProps={{ inputMode: "numeric", pattern: "[0-9]*" }}
          defaultValue={raysCount}
          helperText="Configures how many rays will car use for measurements."
        />
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Ray&apos;s Length
        <TextField
          fullWidth
          size="small"
          defaultValue={raysLength}
          helperText="Sets how long measurement rays will be."
        />
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Rays&apos; Spread
        <TextField
          fullWidth
          size="small"
          defaultValue={raysSpread}
          helperText="Defines the spread of rays"
        />
      </Grid>

      <Grid item xs={12} md={12} lg={12}>
        Hidden Layers
        <TextField
          fullWidth
          size="small"
          defaultValue={hiddenLayers}
          helperText="List of integers separated by ',' specifying size of hidden layers"
        />
      </Grid>
    </Grid>
  );
};
