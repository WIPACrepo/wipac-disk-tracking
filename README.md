# wipac-disk-tracking
Archival media tracking service for WIPAC

## API
The REST API for the wipac-disk-tracking service is documented below.

The API is rooted at:

    /api/v#
    
Where # is the version number of the API under use.

### Disks
Disks represent a unique archival disk.
While the disk entity does carry some immutable identifying information, it
is mostly a container for archival disk events.

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

    POST /events                Create a new event
    GET  /events/:event_id      Get the data for a given event
    GET  /events/search?query   Find an event based on key-value query
