import "./App.css";
import React from "react";
import TelemetryChart from "./components/TelemetryChart";
import TelemetryData from "./components/TelemetryData";
import { SettngsProvier } from "./components/settingsContext";
import FetchPosts from "./components/FetchPosts";

function App() {
  return (
    <div className="App" style={{ width: "100vw", height: "100vh" }}>
      <h1>Show my telemetry data</h1>
      <hr />
      <FetchPosts />
      <hr />
      <SettngsProvier value="chart">
        <TelemetryData
          render={(labels, telemetry) => (
            <TelemetryChart labels={labels} telemetry={telemetry} />
          )}
        />
      </SettngsProvier>
    </div>
  );
}

export default App;
