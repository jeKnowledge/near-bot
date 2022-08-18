use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json::{self, json},
    setup_alloc, AccountId,
};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    inside: LookupMap<AccountId, bool>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            inside: LookupMap::new(b"s".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn add_new_persoon(&mut self, person: Person) {
        self.records.insert(&patitent.person_addr, &patitent);

        env::log(b"Person added");
    }

    /*
    conditions come in
    {
        "conditions": [
            {
                "name_of_syndrome": "None2",
                "date": 123456789
            },
            {
                "name_of_syndrome": "None3",
                "date": 123456789
            }
        ],
        "pacient_addr": "polpy.testnet"
    }
    near call ehr.polpy.testnet add_contitions_to_patient '{"conditions":[{"name_of_syndrome":"None2","date":123456789},{"name_of_syndrome":"None3","date":123456789}],"pacient_addr":"polpy.testnet"}' --accountId polpy.testnet
    */
    pub fn add_contitions_to_patient(
        &mut self,
        conditions: Vec<Conditions>,
        pacient_addr: String,
    ) -> bool {
        // Check if current accountId callee is a doctor
        if !self.is_doctor(env::signer_account_id()) {
            env::log(b"You are not a doctor");
            return false;
        }

        let mut pacient: Person = self.records.get(&pacient_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        pacient.syndromes.extend(conditions);
        self.records.insert(&pacient_addr, &pacient);
        env::log(format!("New conditions added to patient, address: {}", pacient_addr).as_bytes());
        true
    }

    pub fn convert_to_doctor(&mut self, account: String, value: bool) -> bool {
        let mut pacient: Person = self.records.get(&account).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        if value == pacient.is_doctor {
            env::log(format!("Unchanged").as_bytes());
            return false;
        }

        pacient.is_doctor = value;
        self.records.insert(&account, &pacient);
        env::log(
            format!(
                "Doctor status from account {} changed to: {}",
                account, value
            )
            .as_bytes(),
        );
        true
    }

    pub fn add_medical_data(&mut self, medical_data: MedicalData, person_addr: String) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        pacient.medical_data.push(medical_data);
        self.records.insert(&person_addr, &pacient);

        env::log(
            format!(
                "New medical data added to patient, address: {}",
                person_addr
            )
            .as_bytes(),
        );
    }

    pub fn remove_medical_data(&mut self, person_addr: String, index: i32) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        pacient.medical_data.remove(index as usize);
        self.records.insert(&person_addr, &pacient);

        env::log(
            format!(
                "Medical data removed from patient, address: {}",
                person_addr
            )
            .as_bytes(),
        );
    }

    // :(
    pub fn add_departure(&mut self, person_addr: String) {
        let mut pacient: Person = self.records.get(&person_addr).unwrap();
        assert!(!pacient.is_none(), "Pacient is none");

        pacient.departed = true;
        self.records.insert(&person_addr, &pacient);
        env::log(format!("Departure added to patient, address: {}", person_addr).as_bytes());
    }

    /*
     * GETTERS
     */
    pub fn get_medical_data(&self, account: String) -> String {
        //i literally thought that we could get the signer_id here to get the medical data from the person who is calling the method but then i realized that we can't get the signer_id since this is a view method :)
        if self.records.get(&account).is_none() {
            let string = format!("No patitent found with the address: {}", account);
            return serde_json::to_string(&json!({
                "status": string,
                "data": []
            }))
            .unwrap();
        }
        let my_person = self.records.get(&account).unwrap();
        let my_med_data = my_person.medical_data;

        //create a json and append it to the string
        let mut json = String::new();
        for medical_data in my_med_data {
            json.push_str(&medical_data.to_json());
        }
        json
    }

    pub fn get_conditions(&self, account: String) -> String {
        if self.records.get(&account).is_none() {
            let string = format!("No patitent found with the address: {}", account);
            return serde_json::to_string(&json!({
                "status": string,
                "data": []
            }))
            .unwrap();
        }
        let my_person = self.records.get(&account).unwrap();
        let my_conditions = my_person.syndromes;

        return serde_json::to_string(&json!({
            "status": "OK",
            "data": my_conditions
        }))
        .unwrap();
    }

    pub fn get_patient(&self, account: String) -> String {
        if self.records.get(&account).is_none() {
            let string = format!("No patitent found with the address: {}", account);
            return serde_json::to_string(&json!({
                "status": string,
                "data": []
            }))
            .unwrap();
        }
        let my_data = self.records.get(&account).unwrap();
        my_data.to_json()
    }

    fn is_doctor(&self, account: String) -> bool {
        if self.records.get(&account).is_none() {
            return false;
        }
        let my_data = self.records.get(&account).unwrap();
        my_data.is_doctor
    }
}
