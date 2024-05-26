import React, { Component } from 'react'

class TelemetryData extends Component {
  constructor (props) {
    super(props)
    this.state = {
      labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
      telemerty: [
        { id: 0, temperature: 22, humidity: 10 },
        { id: 1, temperature: 23, humidity: 11 },
        { id: 2, temperature: 24, humidity: 12 },
        { id: 3, temperature: 20, humidity: 11 },
        { id: 4, temperature: 21, humidity: 10 },
        { id: 5, temperature: 22, humidity: 10 },
        { id: 6, temperature: 23, humidity: 11 }
      ]
    }
  }

  render () {
    return (
      <div>{this.props.render(this.state.labels, this.state.telemerty)}</div>
    )
  }
}

export default TelemetryData
