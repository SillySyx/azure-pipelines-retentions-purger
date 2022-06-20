This tool will load all builds from a azure devops pipeline and remove all of their retentions leases.

# Setup
* generate pat in dev.azure.com
* set env vars

# How to use
```
export AZURE_ORG=
export AZURE_PROJECT=
export AZURE_PIPELINE=
export AZURE_USERNAME=
export AZURE_PAT=
./azure-pipelines-retentions-purger
```

# Resources
https://docs.microsoft.com/en-us/rest/api/azure/devops/build/builds/list?view=azure-devops-rest-6.0
https://docs.microsoft.com/en-us/rest/api/azure/devops/build/builds/get-retention-leases-for-build?view=azure-devops-rest-7.1
https://docs.microsoft.com/en-us/rest/api/azure/devops/build/leases/delete?view=azure-devops-rest-6.0