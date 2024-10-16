#!/usr/bin/env python3
# client.py

import asyncio
import json
import logging
import os
from typing import cast, Dict

from rest_tools.client import ClientCredentialsAuth, RestClient


async def main() -> None:
    config: Dict[str, str] = {
        "BASE_URL": os.getenv("BASE_URL", None),
        "CLIENT_ID": os.getenv("CLIENT_ID", None),
        "CLIENT_SECRET": os.getenv("CLIENT_SECRET", None),
        "OAUTH_URL": os.getenv("OAUTH_URL", None),
        "REST_URL": os.getenv("REST_URL", None),
    }

    with open('smartctl.json', 'r') as file:
        data = json.load(file)

    # http://localhost:8080/
    base_rc: RestClient = RestClient(config["BASE_URL"])

    # http://localhost:8080/ + Keycloak Credentials
    token_rc: RestClient = ClientCredentialsAuth(address=cast(str, config["BASE_URL"]),
                                                 token_url=cast(str, config["OAUTH_URL"]),
                                                 client_id=cast(str, config["CLIENT_ID"]),
                                                 client_secret=cast(str, config["CLIENT_SECRET"]))

    # http://localhost:8080/api/v1 + Keycloak Credentials
    rest_rc: RestClient = ClientCredentialsAuth(address=cast(str, config["REST_URL"]),
                                                token_url=cast(str, config["OAUTH_URL"]),
                                                client_id=cast(str, config["CLIENT_ID"]),
                                                client_secret=cast(str, config["CLIENT_SECRET"]))

    response = await token_rc.request("GET", "/token")
    print(response, end="\n\n")

    response = await base_rc.request("GET", "/health")
    print(response, end="\n\n")

    response = await rest_rc.request("POST", "/events/sighted", data)
    print(response, end="\n\n")

    response = await base_rc.request("GET", "/health")
    print(response, end="\n\n")

    response = await rest_rc.request("GET", "/events/ZRS1NWBL")
    print(response, end="\n\n")

if __name__ == '__main__':
    logging.basicConfig(level=logging.DEBUG)
    asyncio.run(main())
