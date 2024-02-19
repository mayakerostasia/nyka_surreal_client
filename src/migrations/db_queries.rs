// use crate::debug;
pub const SETUP_DB: &str = r#"
DEFINE TABLE contract;
DEFINE TABLE service;
DEFINE TABLE contract_unit;

DEFINE TABLE onContract SCHEMAFULL;
DEFINE FIELD in ON onContract TYPE record<contract_unit>;
DEFINE FIELD out ON onContract TYPE record<contract>;

DEFINE TABLE onService SCHEMAFULL;
DEFINE FIELD in ON onService TYPE record<contract_unit>;
DEFINE FIELD out ON onService TYPE record<service>;

DEFINE FUNCTION fn::createContractUnit($data: object) {
    LET $id = type::thing("contract_unit", $data.id);

    LET $contract_unit = ( CREATE ONLY $id SET 
        contractID = $data.contractID,
        serviceID = $data.serviceID,
        units = $data.units,
        endDate = $data.endDate
    );

    LET $assoc_contract = ( SELECT id FROM ONLY type::thing("contract", $contract_unit.contractID));
    LET $assoc_service = ( SELECT id FROM ONLY type::thing("service", $contract_unit.serviceID));

    RELATE ($contract_unit.id)->onContract->($assoc_contract.id);
    RELATE ($contract_unit.id)->onService->($assoc_service.id);
    RETURN $contract_unit;
};

DEFINE FUNCTION fn::create_contract($data: object) {
    LET $id = type::thing("contract", $data.id);

    LET $contract = ( CREATE $id SET 
        companyID = $data.companyID,
        contractCategory = $data.contractCategory,
        contractName = $data.contractName,
        contractType = $data.contractType,
        endDate = $data.endDate,
        startDate = $data.startDate
    );

    RETURN $contract;
};

DEFINE FUNCTION fn::create_service($data: object) {
    LET $id = type::thing("service", $data.id);
    LET $service = ( CREATE $id SET 
        name = $data.name, 
        description = $data.description
    );
    RETURN $service;
};
"#;
