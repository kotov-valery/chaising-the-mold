import React, { Component } from 'react'

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

export const options = {
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

export const data = {
  labels: [],
  datasets: [
    {
      label: 'Temperature',
      data: [], //labels.map((_, index) => telemetry[index].temperature),
      borderColor: 'rgb(255, 99, 132)',
      backgroundColor: 'rgba(255, 99, 132, 0.5)'
    },
    {
      label: 'Humidity',
      data: [], //labels.map((_, index) => telemetry[index].humidity),
      borderColor: 'rgb(53, 162, 235)',
      backgroundColor: 'rgba(53, 162, 235, 0.5)'
    }
  ]
}

class TelemetryChart extends Component {
  render () {
    console.log(this.props.labels)
    data.labels = this.props.labels
    const telemetry = this.props.telemetry
    data.datasets[0].data = telemetry.map(
      (_, index) => telemetry[index].temperature
    )
    data.datasets[1].data = telemetry.map(
      (_, index) => telemetry[index].humidity
    )
    return (
      <div id='TelemetryChart' style={{ width: '100%', height: '100%' }}>
        <Line options={options} data={data} />
      </div>
    )
  }
}

export default TelemetryChart
