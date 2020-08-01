# kubectl-reveal-secret

`kubectl-reveal-secret` is a kubectl plugin that reveals Kubernetes Secret data to allow you see raw data.

## Requirements

- `kubectl`
- `jq`

## Installation

```console
$ git clone git@github.com:micnncim/kubectl-reveal-secret.git
$ cp kubectl-reveal-secret/kubectl-reveal-secret /usr/local/bin
```

## Usage

```console
$ kubectl reveal secret --help

  kubectl-reveal-secret reveals Kubernetes Secret data.

  Usage:
      kubectl reveal secret SECRET_NAME [optional flags]

  Options:
      -n, --namespace   specify secrets by key=value format

  Examples:

    # Reveal Secret data in the current namespace.
    $ kubectl reveal secret my-secret

    # Reveal Secret data in the specified namespace.
    $ kubectl reveal secret my-secret -n my-namespace

```

## Examples

```
$ value=$(echo -n 'hello' | base64)
$ cat secret.yaml <<EOF
apiVersion: v1
kind: Secret
metadata:
  name: my-secret
type: Opaque
data:
  key: $value
EOF
$ kubectl apply -f secret.yaml
$ kubectl reveal secret
key     hello
```
