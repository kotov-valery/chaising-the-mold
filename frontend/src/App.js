import logo from './logo.svg'
import './App.css'
import React from 'react'
import TelemetryChart from './components/TelemetryChart'
import TelemetryData from './components/TelemetryData'

function App () {
  return (
    <div className='App'>
      <h1>Show my telemetry data</h1>
      <img src={logo} className='App-logo' alt='logo' />
      <hr />
      <TelemetryData
        render={(labels, telemetry) => (
          <TelemetryChart labels={labels} telemetry={telemetry} />
        )}
      />
    </div>
  )
}

export default App
