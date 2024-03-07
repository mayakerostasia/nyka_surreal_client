USE NS test;
USE DB test;
# EVENT TABLE
# -----------
DEFINE TABLE event SCHEMAFULL
# PERMISSIONS <here>
;
DEFINE FIELD info ON event FLEXIBLE TYPE object;
DEFINE FIELD eventType ON event TYPE string;
# ----------

# ISSUE TABLE
# -----------
DEFINE TABLE issue SCHEMAFULL
# PERMISSIONS <here>
;
DEFINE FIELD issue_id ON issue TYPE string;
DEFINE FIELD data ON issue FLEXIBLE TYPE option<object>;
DEFINE FIELD creation_event ON issue TYPE option<record>;
DEFINE FIELD last_update ON issue TYPE option<record>;
# ----------

## LINK TABLE
## ---------- 
DEFINE TABLE belongsTo SCHEMAFULL
# PERMISSIONS <here>
;
DEFINE FIELD in ON belongsTo TYPE record<event>;
DEFINE FIELD out ON belongsTo TYPE record<issue>;
## ----------

DEFINE FUNCTION fn::newEvent($issue_id: string, $info: object, $event_type: string) {

    LET $id = type::thing("issue", $issue_id);
        
    LET $issue = (SELECT id FROM $id);

    IF !$issue {
        CREATE $id 
        SET issue_id = $issue_id;
    };
    
    LET $event = ( 
        CREATE ONLY type::thing("event", [$issue[0].id, time::now()]) 
        SET 
            info = $info,
            eventType = $event_type
    );        
    
    RELATE ($event.id)->belongsTo->($issue.id);
      
    RETURN 
    "Issue ID="+<string> $id + " " +
    "Issue="+<string> $issue + " " +
    "Event="+<string> $event + " " ;
};