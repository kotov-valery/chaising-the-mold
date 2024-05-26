import './App.css'
import React from 'react'
import TelemetryChart from './components/TelemetryChart'
import TelemetryData from './components/TelemetryData'
import { SettngsProvier } from './components/settingsContext'

function App () {
  return (
    <div className='App'>
      <h1>Show my telemetry data</h1>
      <hr />
      <SettngsProvier value='chart'>
        <TelemetryData
          render={(labels, telemetry) => (
            <TelemetryChart labels={labels} telemetry={telemetry} />
          )}
        />
      </SettngsProvier>
    </div>
  )
}

export default App
