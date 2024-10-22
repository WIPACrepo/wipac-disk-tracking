# README.md
wipac-disk-tracking example client (auf Python)

## Installation
You should first create a Python virtual environment:

    python3 -m venv env

Then you should activate the Python virtual environment:

    source env/bin/activate

Then you should install the libraries required by the client:

    pip3 install -r requirements.txt

Finally, you'll need to create the file `keycloak-client-secret` with the client secret.


## Usage
When run, the client will:

- display information about the keycloak token
- get the /health route to check if the service is healthy
- post an example event to the wipac-disk-tracking service
- get the /health route to check if the service is healthy
- get and display the example event posted to the service

You can run the client with a helper script:

    client.sh
