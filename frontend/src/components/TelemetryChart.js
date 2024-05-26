import React, { Component } from 'react'
import { SettingsConsumer } from './settingsContext'

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js'
import { Line } from 'react-chartjs-2'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
)

class TelemetryChart extends Component {
  options = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'top'
      },
      title: {
        display: true,
        text: 'Telemetry Data'
      }
    }
  }

  render () {
    const { labels, telemetry } = this.props
    const data = {
      labels: labels,
      datasets: [
        {
          label: 'Temperature',
          data: telemetry.map(entry => entry.temperature),
          borderColor: 'rgb(255, 99, 132)',
          backgroundColor: 'rgba(255, 99, 132, 0.5)'
        },
        {
          label: 'Humidity',
          data: telemetry.map(entry => entry.humidity),
          borderColor: 'rgb(53, 162, 235)',
          backgroundColor: 'rgba(53, 162, 235, 0.5)'
        }
      ]
    }

    return (
      <SettingsConsumer>
        {componentName => {
          if (componentName !== 'chart') return null
          return (
            <div id='TelemetryChart' style={{ width: '100%', height: '100%' }}>
              <Line options={this.options} data={data} />
            </div>
          )
        }}
      </SettingsConsumer>
    )
  }
}

export default TelemetryChart
