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

    LET $id = type::thing(\"issue\", $issue_id);
        
    LET $issue = (SELECT id FROM $id);

    IF !$issue {
        CREATE $id 
        SET issue_id = $issue_id;
    };
    
    LET $event = ( 
        CREATE ONLY type::thing(\"event\", [$issue[0].id, time::now()]) 
        SET 
            info = $info,
            eventType = $event_type
    );        
    
    RELATE ($event.id)->belongsTo->($issue.id);
      
    RETURN 
    \"Issue ID=\"+<string> $id + \" \" +
    \"Issue=\"+<string> $issue + \" \" +
    \"Event=\"+<string> $event + \" \" #+
    ;
};"



"BEGIN TRANSACTION;
DELETE event;
DELETE issue;
DELETE belongsTo;
COMMIT TRANSACTION;"



"--all events related to their respective issues
SELECT 
    *, 
    <-belongsTo<-event AS events 
FROM issue
LIMIT 10
;
"



"LET $query = (
SELECT
    *,
    # <-belongsTo<-event WHERE 
#     ( 
#         SELECT 
#             *
#             OMIT info
#         FROM 
#             <-belongsTo<-event
#         WHERE eventType = \"IssueCreated\"
#     
#     ) as creation_event,
    (
        SELECT
            id,
            #*,
            #info.issue.fields.*,# OMIT info.issue.fields, /* OMIT info.changelog, info.issue.fields,*/
            info.issue.fields.summary as Summary,
            info.issue.fields.customfield_10002[0].name as CustomerName,
            #info.issue.fields.customfield_10080 as SLLink,
            info.issue.fields.project.name as Project,
            info.issue.fields.issuetype.name as IssueType,
            info.issue.fields.status.name as Status
        FROM <-belongsTo<-event
        WHERE
            eventType = \"IssueUpdated\"
        ORDER BY id DESC
        LIMIT 1
    ) as last_update
FROM 
    issue
WHERE
    <-belongsTo<-event.*.eventType CONTAINS \"IssueUpdated\" 
    AND \"SD\" IN <string> <-belongsTo<-event.id[0].*.issue_id
    #AND \"SOAR\" IN <string> <-belongsTo<-event.id[0].*.issue_id
# FETCH last_update[0].id
     #CONTAINS \"SD\"
LIMIT 1
);

RETURN count($query);

RETURN $query;
"



"LET $sd = ( 
    SELECT id FROM issue WHERE issue_id CONTAINS \"SD\"
);

# array::first($sd.id);

\"SD\" IN <string> array::first($sd.id);"


,
"LET $query = (
SELECT
    *,
    <-belongsTo<-event[WHERE eventType = \"IssueCreated\"].id as created,
    <-belongsTo<-event[WHERE eventType = \"IssueUpdated\"].id as updates,
    <-belongsTo<-event[WHERE eventType = \"CommentCreated\"].id as comments
FROM 
    issue
WHERE
    \"SOAR\" IN <string> <-belongsTo<-event.id[0].*.issue_id
);

RETURN count($query);

RETURN $query;
"


,
"## Get All Comments

LET $query = (
SELECT
    *,
    <-belongsTo<-event[WHERE eventType = \"CommentCreated\"].id as comment_ids,
    <-belongsTo<-event[WHERE eventType = \"CommentCreated\"].info.comment.body as comments,
    <-belongsTo<-event[WHERE eventType = \"CommentCreated\"].info.issue.self as api_link
FROM 
    issue
WHERE
    <-belongsTo<-event[WHERE eventType = \"CommentCreated\"].id
);

RETURN count($query);

RETURN $query;
"

, "text": "SELECT
    *
FROM
    issue
;" },
 "text": "DELETE webhook_event;" },


"SELECT
    # *,
    # changelog,
    issue_event_type_name,
    changelog.items[*].field,
    changelog.items[*].fieldId,
    issue.fields.customfield_10150,
    issue.key,
    issue.fields.summary
FROM webhook_event
WHERE issue
# LIMIT 10
SPLIT issue_event_type_name"



"SELECT
    # *,
    # changelog,
    issue_event_type_name,
    # changelog.items[*].field,
    # changelog.items[*].fieldId,
    issue.fields.customfield_10150,
    issue.key,
    issue.fields.summary,
    time::from::millis(timestamp)
FROM webhook_event
WHERE issue_event_type_name contains \"updated\"
# LIMIT 100
# SPLIT issue_event_type_name"

 "text": "SELECT
    *
    FROM
    web" },


DEFINE FUNCTION fn::get_all_contracts() {
    let $contracts = (
            SELECT
                *,
                companyID,
                contractName,
                endDate,
                (
                    SELECT 
                        (->onService->service)[WHERE \"-\" IN name][0].name as name,
                        endDate,
                        units
                    FROM <-onContract<-contract_unit
                    GROUP BY name, units, endDate
                )[WHERE name] as units
            FROM
                contract
            ORDER BY contractName ASC
            FETCH
                <-onContract<-contract_unit
            PARALLEL
    );
    RETURN $contracts
};
        
------------------------------------
DEFINE FUNCTION fn::get_contract($short_name: string) {
#     THROW $short_name;
    let $contracts = (
            SELECT
                *,
                companyID,
                contractName,
                endDate,
                (
                    SELECT 
                        (->onService->service)[WHERE \"-\" IN name][0].name as name,
                        endDate,
                        units
                    FROM <-onContract<-contract_unit
                    GROUP BY name, units, endDate
                )[WHERE name] as units
            FROM
                contract
            WHERE
                string::contains(contractName, $short_name)
            ORDER BY contractName ASC
            FETCH
                <-onContract<-contract_unit
            PARALLEL
    );
    RETURN $contracts
}
        


"        # <-onContract<-contract_unit.serviceID) as service_ids,
        let $contracts = (
            SELECT 
                companyID,
                contractName,
                endDate,
                {
                    \"service\": <-onContract<-contract_unit->onService->service ,
                    \"contract_unit\": <-onContract<-contract_unit
                } as contract_units
            FROM
                contract
            WHERE
                \"-\" in ((<-onContract<-contract_unit)[0]->onService->service)[0].name
            ORDER BY contractName ASC
            FETCH 
                contract_units.service,
                contract_units.contract_unit
                #<-onContract<-contract_unit
        );
        
        RETURN $contracts
"



"        # <-onContract<-contract_unit.serviceID) as service_ids,
        let $contracts = (
            SELECT
                #*,
                companyID,
                contractName,
                endDate,
                <-onContract<-contract_unit as contract_unit,
                (
                    SELECT 
                        (->onService->service)[WHERE \"-\" IN name][0].name as name,
                        units
                    FROM contract_unit
                    GROUP BY name, units
                )[WHERE name] as units
            FROM
                contract
            WHERE
                \"-\" in ((<-onContract<-contract_unit)[0]->onService->service)[0].name
            ORDER BY contractName ASC
            FETCH
                contract_unit
            PARALLEL
        );
        
        RETURN $contracts
"



"fn::get_all_contracts();
fn::get_contract(\"ABNC\")"



"LET $all_six_twelve = (SELECT
    *,
    units,
    ->onContract->contract as contract
    # ->onService->service as service

FROM contract_unit
WHERE
    serviceID = 612
FETCH contract );

LET $my_units = SELECT 
    units
from 
    $all_six_twelve;

LET $count = math::sum($my_units.units)

RETURN $all_six_twelve
RETURN $my_units
RETURN $count"



"USE NS nico;
USE DB rs_jira_webhooks;
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

    LET $id = type::thing(\"issue\", $issue_id);
        
    LET $issue = (SELECT id FROM $id);

    IF !$issue {
        CREATE $id 
        SET issue_id = $issue_id;
    };
    
    LET $event = ( 
        CREATE ONLY type::thing(\"event\", [$issue[0].id, time::now()]) 
        SET 
            info = $info,
            eventType = $event_type
    );        
    
    RELATE ($event.id)->belongsTo->($issue.id);
      
    RETURN 
    \"Issue ID=\"+<string> $id + \" \" +
    \"Issue=\"+<string> $issue + \" \" +
    \"Event=\"+<string> $event + \" \" #+
    ;
};
"



"--all events related to their respective issues
SELECT 
    *, 
    <-belongsTo<-event AS events 
FROM issue
LIMIT 10
;
"



"INFO FOR ROOT;
INFO FOR NS;
INFO FOR DB;

"



"LET $query = (
SELECT
    *,
    <-belongsTo<-event[WHERE eventType = \"IssueCreated\"].id as created,
    <-belongsTo<-event[WHERE eventType = \"IssueUpdated\"].id as updates,
    <-belongsTo<-event[WHERE eventType = \"CommentCreated\"].id as comments
FROM 
    issue
WHERE
    \"SOAR\" IN <string> <-belongsTo<-event.id[0].*.issue_id
FETCH updates
);

RETURN count($query);

RETURN $query;
"

 "text": "SELECT 
*
FROM 
    issue" },


"INFO FOR ROOT;
INFO FOR NS;
INFO FOR DB;
INFO FOR TABLE test_table;
SELECT * FROM test_table;"



"INFO FOR ROOT;
INFO FOR NS;
INFO FOR DB;

# DELETE test_table WHERE id != \"test_person\";"

 "text": "DELETE test_table;" },
 "text": "SELECT * FROM ckb;" },


"INFO FOR ROOT;
INFO FOR NS;
INFO FOR DB;

# DELETE test_table WHERE id != \"test_person\";"



"#SELECT * FROM ckb;

SELECT
    Customer, 
    ( SELECT * from Graylog_Alerts where Customer.short_name = \"AT\") as alerts
FROM ckb
WHERE Customer.name = \"Adams Township\";"
