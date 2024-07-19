param location string
param environmentName string

resource containerAppEnv 'Microsoft.App/managedEnvironments@2022-03-01' = {
  name: environmentName
  location: location
  properties: {
  }
}

output id string = containerAppEnv.id
