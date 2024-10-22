# wipac-disk-tracking
Archival media tracking service for WIPAC

## Routes
The following routes are supported in the wipac-disk-tracking service:

### API /
These routes live at the root of the service.

#### GET /health
This route allows a client to query the health of the service. Mostly,
this is just a check if the service can access the database that stores
the disk events. This route requires no authentication, and can be
checked with a simple `curl` command:

    curl -v http://localhost:8080/health && echo ""

A healthy response is 200, along with some JSON indicating the status
of the service. Here `count` is a simple count of disk events stored
in the backing database:

    {
        "status": "ok",
        "count": 0
    }

An unhealthy response is 500, along with some JSON indicating the error
the service is experiencing connecting to the database:

    {
        "status": "error",
        "message": "Failed to connect to the database: MongoError(\"Kind: Server selection timeout: No available servers. Topology: { Type: Single, Servers: [ { Address: localhost:27017, Type: Unknown, Error: Kind: I/O error: Connection refused (os error 111), labels: {} } ] }, labels: {}\")"
    }

#### GET /token
This route allows a client to query the service's view of its token
issued by Keycloak for authorization. This route exists for debugging
purposes.

The client can also inspect its token in a service like [jwt.io](https://jwt.io).

This route offers a view of 'how did the service parse my token' or
'how does the service see my authorization grants'?

### API /api/v1
These routes live under /api/v1.

#### GET /events/:event_id
Get the data for a given event. This route is actually multi-purpose.

If you specify the UUID assigned to an event, you can find the record
for a specific event:

    GET /api/v1/events/0badfdd0-963a-4b25-9af6-1acc52c5d334

If you specify the serial number of a disk, you can find the records of
every event referencing that serial number:

    GET /api/v1/events/ZRS1NWBL

In both cases, a successful query will return with the structure like
this one:

    {
        "events": [
            {
                smartctl-event-here
            },
            ...
            {
                smartctl-event-here
            }
        ]
    }

#### POST /events/closed
Create a new 'closed' event. This disk was determined to full/finished
and archival activity stopped.

#### POST /events/formatted
Create a new 'formatted' event. This disk was given a file system to
make it ready for archival purposes.

#### POST /events/opened
Create a new 'opened' event. This disk was given a label and was
designated for active archival activity.

#### POST /events/sighted
Create a new 'sighted' event. This disk was observed to be loaded in a
host that processes archival disks.

## Development
As typical in a Rust project, you can run the unit and integration tests with:

    cargo test -- --show-output

### MongoDB Tests
The MongoDB integration test will look for the temporary MongoDB container
running locally. If you'd rather run the test against a different MongoDB,
you can supply the URL through this environment variable:

    DESTRUCTIVE_TEST_MONGODB_URL

Note that the integration test is destructive. It may wipe out your collections,
indexes, or databases! DON'T point this at any data that you want to keep!

If MongoDB is not available, locally or through the provided URL, then a
message will appear in the output of the tests:

    "MongoDB is not available. Skipping test."
