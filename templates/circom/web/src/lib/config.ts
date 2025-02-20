function isEnvVarDefined(name: string, value: unknown) {
  if (value === '') throw new Error(`Missing environment variable ${name}`)
}

function getEnvVar<T extends string>(name: T) {
  const value = import.meta.env[name] ?? ''
  isEnvVarDefined(name, value)
  return value
}

interface Config {
  appName: string
  dynamic: { environmentId: string }
}

export const config: Config = {
  appName: 'web-app-template',
  dynamic: { environmentId: getEnvVar('VITE_DYNAMIC_ENVIRONMENT_ID') },
}
