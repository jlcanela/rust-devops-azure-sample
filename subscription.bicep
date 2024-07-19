targetScope = 'subscription'

param location string = 'francecentral'
param resourceGroupName string = 'ResourceGroupRustApp'
param environmentName string = 'DevEnv'
param appName string = 'rust-app'
@secure()
param hashSecret string
@secure()
param jwtSecret string
param databaseUrl string
param registryUsername string = 'jlcanela'
@secure()
param registryPassword string
param imageUrl string = 'ghcr.io/jlcanela/rust-azure-webapp-sample@sha256:9d7b795d638a1aa24bfb46b6eea8cbc3a4b64f71706311940426d686c3e77c4f'

resource resourceGroup 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: resourceGroupName
  location: location
}

module containerAppEnv 'modules/containerAppEnv.bicep' = {
  scope: resourceGroup
  name: 'containerAppEnv'
  params: {
    location: location
    environmentName: environmentName
  }
}

module containerApp 'modules/containerApp.bicep' = {
  scope: resourceGroup
  name: 'containerApp'
  params: {
    location: location
    appName: appName
    environmentId: containerAppEnv.outputs.id
    hashSecret: hashSecret
    jwtSecret: jwtSecret
    databaseUrl: databaseUrl
    registryUsername: registryUsername
    registryPassword: registryPassword
    imageUrl: imageUrl
  }
}
