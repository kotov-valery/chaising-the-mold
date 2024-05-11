import logo from './logo.svg';
import './App.css';
import React from 'react';

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';
import { Line } from 'react-chartjs-2';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

export const options = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'top',
    },
    title: {
      display: true,
      text: 'Telemetry Data',
    },
  },
};

const labels = ['January', 'February', 'March', 'April', 'May', 'June', 'July'];

const telemetry = [
  { id: 0, temperature: 22, humidity: 10 },
  { id: 1, temperature: 23, humidity: 11 },
  { id: 2, temperature: 24, humidity: 12 },
  { id: 3, temperature: 20, humidity: 11 },
  { id: 4, temperature: 21, humidity: 10 },
  { id: 5, temperature: 22, humidity: 10 },
  { id: 6, temperature: 23, humidity: 11 },
];

export const data = {
  labels,
  datasets: [
    {
      label: 'Temperature',
      data: labels.map((_, index) => telemetry[index].temperature),
      borderColor: 'rgb(255, 99, 132)',
      backgroundColor: 'rgba(255, 99, 132, 0.5)',
    },
    {
      label: 'Humidity',
      data: labels.map((_, index) => telemetry[index].humidity),
      borderColor: 'rgb(53, 162, 235)',
      backgroundColor: 'rgba(53, 162, 235, 0.5)',
    },
  ],
};

function App() {
  return (
    <div className="App">
      <h1>Show my telemetry data</h1>
      <img src={logo} className="App-logo" alt="logo" />

      <hr />

      <div id="TelemetryChart" style={{ width: '100%', height: '100%' }}>
        <Line options={options} data={data} />
      </div>


    </div>
  );
}

export default App;