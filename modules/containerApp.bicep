param location string
param appName string
param environmentId string
@secure()
param hashSecret string
@secure()
param jwtSecret string
param databaseUrl string
param registryUsername string
@secure()
param registryPassword string
param imageUrl string

resource containerApp 'Microsoft.App/containerApps@2022-03-01' = {
  name: appName
  location: location
  properties: {
    managedEnvironmentId: environmentId
    configuration: {
      ingress: {
        external: true
        targetPort: 8080
      }
      secrets: [
        {
          name: 'hash-secret'
          value: hashSecret
        }
        {
          name: 'jwt-secret'
          value: jwtSecret
        }
        {
          name: 'registry-username'
          value: registryUsername
        }
        {
          name: 'registry-password'
          value: registryPassword
        }
      ]
      registries: [
        {
          server: 'ghcr.io'
          username: registryUsername
          passwordSecretRef: 'registry-password'
        }
      ]
    }
    template: {
      containers: [
        {
          name: appName
          image: imageUrl
          env: [
            {
              name: 'HASH_SECRET'
              secretRef: 'hash-secret'
            }
            {
              name: 'JWT_SECRET'
              secretRef: 'jwt-secret'
            }
            {
              name: 'DATABASE_URL'
              value: databaseUrl
            }
          ]
        }
      ]
      scale: {
        minReplicas: 1
        maxReplicas: 1
      }
    }
  }
}
