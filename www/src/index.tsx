import React from "react";
import ReactDOM from "react-dom/client";
import reportWebVitals from "./reportWebVitals";
import { Grid, Box } from "@mui/material";
import { minHeight } from "@mui/system";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

root.render(
  <React.StrictMode>
    <Grid
      container
      sx={{
        minHeight: "100vh",
        display: "flex",
        flexDirection: "row",
        backgroundColor: "gray",
      }}
    >
      <Grid item xs={8} md={8} sx={{ backgroundColor: "gold" }}>
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
                table
              </Grid>
              <Grid item xs={4}>
                world config
              </Grid>
              <Grid item xs={4}>
                agent config
              </Grid>
            </Grid>
          </Grid>
          <Grid item sx={{ backgroundColor: "yellow", minHeight: "60vh" }}>
            NeuralNetwor
          </Grid>
        </Grid>
      </Grid>
      <Grid item xs={4} md={4} sx={{ backgroundColor: "purple" }}>
        Road
      </Grid>
    </Grid>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();

/* 

<Grid container xs={12} md={8}>
        <Grid item xs={8} md={8} sx={{ backgroundColor: "gold" }}>
          config space
        </Grid>
        <Grid item xs={4} md={4} sx={{ backgroundColor: "purple" }}>
          world space
        </Grid>
      </Grid>

      <Grid
        item
        xs={8}
        md={4}
        sx={{
          display: "flex",
          justifyContent: "center",
          backgroundColor: "cyan",
        }}
      >
        Network Content
      </Grid>


*/
