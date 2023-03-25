#![allow(clippy::expect_used, clippy::print_stdout)]

use std::fs;

use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};
use fhir_model::r4b::{
	codes::{RequestIntent, RequestStatus, RiskProbability},
	resources::{
		Basic, Patient, RequestGroup, RequestGroupAction, RequestGroupActionTiming, Resource,
		WrongResourceType,
	},
	types::{CodeableConcept, Coding},
};
use serde_json::Value;

#[test]
fn serialization_deserialization() {
	let config = Config::new(CompareMode::Strict).numeric_mode(NumericMode::AssumeFloat);

	for entry in fs::read_dir(format!("{}/tests/examples-json/", env!("CARGO_MANIFEST_DIR")))
		.expect("read dir")
	{
		let file = entry.expect("file entry").path();
		assert!(file.is_file());

		println!("Checking file `{}`..", file.display());

		let string = fs::read_to_string(file).expect("reading file");
		let json: Value = serde_json::from_str(&string).expect("deserialize to value");
		let deserialized: Resource = serde_json::from_value(json.clone()).expect("deserializing");
		let serialized = serde_json::to_value(&deserialized).expect("serializing");
		assert_json_matches!(serialized, json, config.clone());
	}
}

#[test]
fn builder_works() {
	let _request_group = RequestGroup::builder()
		.status(RequestStatus::Active)
		.intent(RequestIntent::Order)
		.action(vec![Some(
			RequestGroupAction::builder()
				.timing(RequestGroupActionTiming::DateTime("2023".to_owned()))
				.code(vec![Some(
					CodeableConcept::builder()
						.coding(vec![Some(
							Coding::builder()
								.system("system".to_owned())
								.code("code".to_owned())
								.display("display".to_owned())
								.build(),
						)])
						.build(),
				)])
				.build(),
		)])
		.build();
}

#[test]
fn resource_conversion() {
	let patient = Patient::builder().build();
	let resource: Resource = patient.into();
	let patient: Patient = resource.try_into().expect("It is a Patient resource");
	let resource: Resource = patient.into();
	let _patient: &Patient = (&resource).try_into().expect("It is a Patient resource");
	let result: Result<Basic, WrongResourceType> = resource.try_into();
	assert!(result.is_err());
}

#[test]
fn coding_concepts() {
	let code = RiskProbability::_Custom("Test".to_owned());
	let coding = Coding::from(code.clone());
	assert!(coding.code.is_some());
	assert!(coding.system.is_some());
	let concept = CodeableConcept::from(code);
	assert_eq!(concept.coding.len(), 1);
	assert!(concept.text.is_some());
}
