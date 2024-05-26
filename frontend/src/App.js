import logo from './logo.svg'
import './App.css'
import React from 'react'
import TelemetryChart from './components/TelemetryChart'
import { TELEMETRY_LABELS, TELEMETRY_DATA } from './components/TelemetryData'

function App () {
  return (
    <div className='App'>
      <h1>Show my telemetry data</h1>
      <img src={logo} className='App-logo' alt='logo' />
      <hr />
      <TelemetryChart labels={TELEMETRY_LABELS} telemetry={TELEMETRY_DATA} />
    </div>
  )
}

export default App
