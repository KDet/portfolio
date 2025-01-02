# 🚀 Deploying a Flutter Web App on Azure Static Web Apps

> November 10, 2024

Flutter provides a versatile way to build web applications, and Azure Static Web Apps offers a good platform for hosting them. This guide will walk you through building a Flutter web app using Azure DevOps and deploying it to Azure Static Web Apps, ensuring a clear understanding of each step.

## Step 1: Prepare build-time variables

To build a Flutter web app, use the following command pattern in your Azure DevOps pipeline:

```yaml
flutter build web 
  --build-name=$(VERSION) 
  --build-number=$(BUILD_NUMBER) 
  --dart-define=BASE_API=$(BASE_API) 
  --web-renderer html --release
```

**Explanation:**

- `--web-renderer html`: Specifies the HTML renderer for Flutter. Flutter provides two web renderers: html and canvaskit. The HTML renderer is lightweight and ideal for text-heavy apps, while canvaskit is better for complex graphics. Using the HTML renderer results in smaller app sizes, faster load times, and better browser compatibility.

Flutter also supports compiling web apps to WebAssembly (Wasm), which enables near-native performance in the browser and provides significant performance boosts for computation-heavy tasks.

- `--dart-define`: This flag allows you to pass environment-specific variables at build time. In this example, BASE_API defines different API URLs for development, testing, and production, allowing you to control the app's behavior without modifying the source code.

For instance, in your Flutter app, you can use BASE_API like this:

```dart
Dio networkClient() {
  const baseUrl = String.fromEnvironment('BASE_API', defaultValue: '');
  final dio = Dio(BaseOptions(baseUrl: baseUrl));
  return dio..transformer = BackgroundTransformer();
}
```

## Step 2: Make Azure DevOps Build Pipeline

Below is an example of a basic YAML file for Azure DevOps pipeline:

```yaml
trigger:
- main

pool:
  vmImage: 'ubuntu-latest'

variables:
- group: some-other-variables-for-pipeline
- name: BUILD_NUMBER
  value: '$(Build.BuildId)'
- name: VERSION
  value: '1.0.0'
- name: PROJECT_DIRECTORY
  value: 'src/test_app'
- name: BUILD_ARTEFACTS_PATH
  value: 'build/web/'
- name: BASE_API
  value: 'https://{server-name}/api/'

jobs:
- job: BuildTestWebApp
  displayName: 'Build Test Web App'
  steps:
  - task: FlutterInstall@0
    inputs:
      mode: 'auto'
      channel: 'stable'
      version: 'latest'
  - task: FlutterCommand@0
    inputs:
      projectDirectory: '$(PROJECT_DIRECTORY)'
      arguments: 'build web --build-name=$(VERSION) --build-number=$(BUILD_NUMBER) --dart-define=BASE_API=$(BASE_API) --web-renderer html --release'
  - task: AzureStaticWebApp@0
    displayName: 'Deploy Flutter Web Drop'
    inputs:
      app_location: '$(PROJECT_DIRECTORY)/$(BUILD_ARTEFACTS_PATH)'
      output_location: '$(PROJECT_DIRECTORY)/$(BUILD_ARTEFACTS_PATH)'
      skip_app_build: true
      skip_api_build: true
      verbose: true
      azure_static_web_apps_api_token: '$(DEPLOYMENT_TOKEN)'
```

**Explanation:**
- `FlutterInstall@0`: Installs Flutter on the build agent.
- `FlutterCommand@0`: Builds the web app with the specified parameters, generating optimized files for deployment.
- `AzureStaticWebApp@0`: Deploys the built Flutter web app to Azure Static Web Apps using a deployment token you find bellow.

`FlutterInstall@0`, `FlutterCommand@0` steps ensure that Flutter web app is built in a consistent variables, with all dependencie managed by the pipeline.

## Step 3: Create Azure Static Web App

After building Flutter web app, the next step is to deploy it to Azure Static Web Apps:
1. Go to the [Azure Portal](https://portal.azure.com/).
2. Search for **Azure Static Web Apps** and click **Create**.
3. Enter the subscription, resource group, and app name details. Under **Deployment details**, select **Other** to integrate with Azure DevOps instead of GitHub.

![Hosting Plan](https://media.licdn.com/dms/image/v2/D4E12AQHMbx7zMSQB2A/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1731267908639?e=1737590400&v=beta&t=nLgJsthlnG4beusbCvYHU-qaBDrgAfznOQ-BOItx9E0)

After creating the resource, you'll need to press Manage deployment token and copy the deployment token for use in the deployment script. This token allows Azure DevOps to authenticate and push the build artifacts. See, `AzureStaticWebApp@0` line `azure_static_web_apps_api_token: '$(DEPLOYMENT_TOKEN)'`.

![Manage Deployment Token](https://media.licdn.com/dms/image/v2/D4E12AQHLUGgh31H-dA/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1731265410558?e=1737590400&v=beta&t=_rmJGk171mAkm1eq94uBkaPq8NvpS-4MDcMBdyfnF6w)

You can now run the pipeline and should see results similar to this.

![Pipeline Results ](https://media.licdn.com/dms/image/v2/D4E12AQEj7hOZWEOlaw/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1731313761489?e=1737590400&v=beta&t=2I3U0q_dT6a7VoLdfCsPZfBr-CUQpEdE0RtPX1QZb8U)

## Step 4: Handling CORS Issues

A common challenge when hosting web apps is **Cross-Origin Resource Sharing (CORS)**. Browsers enforce CORS to restrict how resources are accessed across different domains, which can lead to errors when frontend makes requests to an API hosted on a different domain.

Since you cannot disable CORS in a Flutter app directly, consider these options:

1. Use a Proxy Server

Configure a proxy server to forward API requests, effectively bypassing CORS restrictions. In Azure Static Web Apps, this can be done using a [staticwebapp.config.json](https://learn.microsoft.com/en-us/azure/static-web-apps/configuration) file in `$(PROJECT_DIRECTORY)/web` directory like this:

```json
{
  "routes": [
    {
      "route": "/api/*",
      "allowedRoles": ["anonymous"],
      "rewrite": "https://{server-name}/api/*"
    }
  ]
}
```

This allows requests to **/api/* ** to be proxied to the backend server, avoiding CORS issues.

2. CORS Proxy Service

For development purposes, you can use a third-party CORS proxy like cors-anywhere, though it's not recommended for production due to security concerns. You can also pass `BASE_API=https://cors-anywhere.herokuapp.com/https://{server-name}/api/` to the Azure pipeline to bypass CORS restrictions.

3. Local Reverse Proxy for Development

Use tools like **http-proxy-middleware** in a Node.js server or configure Nginx to create a reverse proxy that forwards requests to the backend. This allows you to develop locally without worrying about CORS issues.

## Summary

In this guide, we covered how to build and deploy a Flutter web app using Azure DevOps and Azure Static Web Apps. We discussed handling CORS issues, setting up build pipelines, and deploying efficiently.

Azure Static Web Apps is a solid option for scalable hosting with CI/CD integration, offering features like auto-scaling, easy integration with Azure Functions, and support for the free plan. By using it, you can deploy your Flutter app smoothly and make it accessible across different environments.
