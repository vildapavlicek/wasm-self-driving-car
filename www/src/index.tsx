import React from "react";
import ReactDOM from "react-dom/client";
import reportWebVitals from "./reportWebVitals";
import App, { initWorld } from "./App";
import init, { test } from "wasm-self-driving-car";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

console.log("index called");

init()
  .then(
    () => {
      test();
      initWorld();

      root.render(
        <React.StrictMode>
          <App />
        </React.StrictMode>
      );
    },
    () => {
      console.log("failed");
    }
  )
  .catch((e) => {
    console.error(`Failed to init WASM, error: ${e}`);
    root.render(
      <body>
        Failed to initialize WASM package which is critical to run this app.
      </body>
    );
  });

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
