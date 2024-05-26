import React from 'react'

const SettingsContext = React.createContext()

const SettngsProvier = SettingsContext.Provider
const SettingsConsumer = SettingsContext.Consumer

export { SettngsProvier, SettingsConsumer }
