# Dummy rust 'operator'

This is a quick and dirty rust experiment to learn about how to use the k8s api
using rust from within a namespace using the service account token in the
container.

## Quick setup

```bash
# start minikube
minikube start
# put minikube docker context into scope
eval $(minikube docker-env)
# build image (in minikube context)
docker build -t poc-operator .
# apply the resources
kubectl create ns poc-operator
kubectl apply -f setup.yaml
```
