import { Box } from "@mui/system";
import React, { FC, ReactElement } from "react";
import { IAgentsSettings } from "../agentsForm/IAgentsSettings";
import { World } from "../world/world";
import { IWorldSettings } from "../worldFrom/IWorldSettings";

export const WorldCanvas: FC<IAgentsSettings & IWorldSettings> = (
  props: IAgentsSettings & IWorldSettings
): ReactElement => {
  // console.log(`WorldCanvas world's state is${props?.simulation.state}`);
  return (
    <Box
      sx={{ display: "flex", justifyContent: "center", alignContent: "center" }}
    >
      <canvas
        style={{
          minHeight: "200px",
          maxWidth: "300px",
          minWidth: "200px",
          height: "99vh",
          backgroundColor: "gray",
        }}
      ></canvas>
    </Box>
  );
};

//
