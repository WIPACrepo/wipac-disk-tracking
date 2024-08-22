# wipac-disk-tracking
Archival media tracking service for WIPAC

## API
The REST API for the wipac-disk-tracking service is documented below.

The API is rooted at:

    /api/v1

### Disks
Disks represent a collection of unique archival disks. While the disk
entity does carry some immutable identifying information, it is mostly
a container for archival disk events.

#### Routes
These routes are implemented to work with disks:

    GET  /disks/:disk_id                    Get the data for a given disk
    GET  /disks/:disk_id/events             Get all of the events for a given disk
    GET  /disks/:disk_id/events/:event_id   Get the data for a given event for a given disk
    GET  /disks/:disk_id/search?query       Find a disk based on a key-value query

### Events
Events represent a record of an event involving an archival disk.  
There are four distinct events that are tracked by the system.

    sighted     This disk was observed to be loaded in a host that processes archival disks
    formatted   This disk was given a file system to make it ready for archival purposes
    opened      This disk was given a label and was designated for active archival activity
    closed      This disk was determined to full/finished and archival activity stopped

The format for each of these events is specified with a JSON Schema file.

#### Routes
These routes are implemented to work with events:

    POST /events/closed         Create a new 'closed' event
    POST /events/formatted      Create a new 'formatted' event
    POST /events/opened         Create a new 'opened' event
    POST /events/sighted        Create a new 'sighted' event
    GET  /events/:event_id      Get the data for a given event

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
