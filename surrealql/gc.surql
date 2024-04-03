use ns stage1;
use db bastion;
info for db;

# fn::get_gc_info();
SELECT id FROM ckb LIMIT 1;

SELECT 
    id,
    *,
    ->forClient->ckb.`Customer Name`                 as customer_name,
    ->forClient->ckb.`Autotask Client Label`         as short_name,
    ->forClient->ckb.`JIRA Org ID`                   as jira_id,
    labels[*].key                                    as labels             # LABELS
FROM 
    gc_incident;
