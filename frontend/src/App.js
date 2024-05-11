import logo from './logo.svg';
import './App.css';
import React from 'react';

function App() {
  const telemetry = [
    { id: 0, temperature: 22, humidity: 10 },
    { id: 1, temperature: 23, humidity: 11 },
    { id: 2, temperature: 24, humidity: 12 },
    { id: 3, temperature: 20, humidity: 11 },
    { id: 4, temperature: 21, humidity: 10 },
  ];

  
  return (
    <div className="App">
      <h1>Show my telemetry data</h1>
      <img src={logo} className="App-logo" alt="logo" />

      <hr />

      <List list={telemetry} />

    </div>
  );
}

const List = props =>
  props.list.map((item) => (
    <div key={item.id}>
      <h3>Temperature: {item.temperature}</h3>
      <h3>Humidity: {item.humidity}</h3>
      <hr />
    </div>
  ));

export default App;