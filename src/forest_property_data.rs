use std::fs;
use serde::{Deserialize, Serialize};
use reqwest::blocking::get;
use quick_xml::de::from_str;

#[derive(Serialize, Deserialize)]
pub struct ForestPropertyData {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:re", skip_serializing_if = "Option::is_none")]
    pub xmlns_re: Option<String>,
    #[serde(rename = "@xmlns:st")]
    pub xmlns_st: String,
    #[serde(rename = "@xmlns:ts")]
    pub xmlns_ts: String,
    #[serde(rename = "@xmlns:tst")]
    pub xmlns_tst: String,
    #[serde(rename = "@xmlns:dts")]
    pub xmlns_dts: String,
    #[serde(rename = "@xmlns:tss")]
    pub xmlns_tss: String,
    #[serde(rename = "@xmlns:op")]
    pub xmlns_op: String,
    #[serde(rename = "@xmlns:sf")]
    pub xmlns_sf: String,
    #[serde(rename = "@xmlns:gdt")]
    pub xmlns_gdt: String,
    #[serde(rename = "@xmlns:co")]
    pub xmlns_co: String,
    #[serde(rename = "@xmlns:gml")]
    pub xmlns_gml: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "@xmlns:xlink")]
    pub xmlns_xlink: String,
    #[serde(rename = "@schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "@schemaPackageVersion", skip_serializing_if = "Option::is_none")]
    pub schema_package_version: Option<String>,
    #[serde(rename = "@schemaPackageSubversion", skip_serializing_if = "Option::is_none")]
    pub schema_package_subversion: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
    pub re_real_estates: Option<ReRealEstates>,
    #[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
    pub st_stands: Option<StStands>
}

impl ForestPropertyData {
    pub fn from_xml_file(path: &str) -> ForestPropertyData {
        let xml = fs::read_to_string(path).expect("Could not read the XML file");
        from_str(&xml).expect("Could not parse the XML")
    }

    pub fn from_xml_str(xml_str: &str) -> ForestPropertyData {
        from_str(xml_str).expect("Could not parse the XML")
    }

    pub fn from_xml_url(url: &str) -> ForestPropertyData {
        let xml = Self::fetch_xml_url(url).unwrap();
        from_str(&xml).expect("Failed to parse XML")
    }
    
    fn fetch_xml_url(url: &str) -> Option<String> {
        match get(url) {
            Ok(resp) => {
                match resp.text() {
                    Ok(text) => Some(text),
                    Err(e) => {
                        println!("Error: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReRealEstates {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "RealEstate")]
    pub re_real_estate: ReRealEstate
}

#[derive(Serialize, Deserialize)]
pub struct ReRealEstate {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MunicipalityNumber")]
    pub re_municipality_number: String,
    #[serde(rename = "AreaNumber")]
    pub re_area_number: String,
    #[serde(rename = "GroupNumber")]
    pub re_group_number: String,
    #[serde(rename = "UnitNumber")]
    pub re_unit_number: String,
    #[serde(rename = "RealEstateName")]
    pub re_real_estate_name: String,
    #[serde(rename = "Parcels")]
    pub re_parcels: ReParcels
}

#[derive(Serialize, Deserialize)]
pub struct ReParcels {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Parcel")]
    pub re_parcel: Vec<ReParcel>
}

#[derive(Serialize, Deserialize)]
pub struct ReParcel {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ParcelNumber")]
    pub re_parcel_number: String,
    #[serde(rename = "Stands")]
    pub st_stands: StStands
}

#[derive(Serialize, Deserialize)]
pub struct StStands {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Stand")]
    pub st_stand: Vec<StStand>
}

#[derive(Serialize, Deserialize)]
pub struct StStand {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "StandBasicData")]
    pub st_stand_basic_data: StStandBasicData,
    #[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
    pub ts_tree_stand_data: Option<TsTreeStandData>,
    #[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
    pub op_operations: Option<OpOperations>,
    #[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
    pub st_special_features: Option<StSpecialFeatures>
}

#[derive(Serialize, Deserialize)]
pub struct StStandBasicData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "CompleteState")]
    pub st_complete_state: String,
    #[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
    pub st_identifiers: Option<StIdentifiers>,
    #[serde(rename = "StandNumber")]
    pub st_stand_number: String,
    #[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
    pub st_stand_number_extension: Option<String>,
    #[serde(rename = "MainGroup")]
    pub st_main_group: String,
    #[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
    pub st_sub_group: Option<String>,
    #[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
    pub st_fertility_class: Option<String>,
    #[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
    pub st_soil_type: Option<String>,
    #[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
    pub st_drainage_state: Option<String>,
    #[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
    pub st_ditching_year: Option<String>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub st_development_class: Option<String>,
    #[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
    pub st_stand_quality: Option<String>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub st_main_tree_species: Option<String>,
    #[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
    pub st_accessibility: Option<String>,
    #[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
    pub st_cutting_restriction: Option<String>,
    #[serde(rename = "SilvicultureRestriction", skip_serializing_if = "Option::is_none")]
    pub st_silviculture_restriction: Option<String>,
    #[serde(rename = "StandBasicDataDate")]
    pub st_stand_basic_data_date: String,
    #[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
    pub st_stand_info: Option<String>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
    pub st_growth_place_data_source: Option<String>,
    #[serde(rename = "Area")]
    pub st_area: String,
    #[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
    pub st_area_decrease: Option<String>,
    #[serde(rename = "PolygonGeometry")]
    pub gdt_polygon_geometry: GdtPolygonGeometry
}

#[derive(Serialize, Deserialize)]
pub struct StIdentifiers {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Identifier")]
    pub st_identifier: Vec<StIdentifier>
}

#[derive(Serialize, Deserialize)]
pub struct StIdentifier {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "IdentifierType")]
    pub co_identifier_type: String,
    #[serde(rename = "IdentifierValue")]
    pub co_identifier_value: String
}

#[derive(Serialize, Deserialize)]
pub struct GdtPolygonGeometry {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "pointProperty")]
    pub gml_point_property: GmlPointProperty,
    #[serde(rename = "polygonProperty")]
    pub gml_polygon_property: GmlPolygonProperty
}

#[derive(Serialize, Deserialize)]
pub struct GmlPointProperty {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Point")]
    pub gml_point: GmlPoint
}

#[derive(Serialize, Deserialize)]
pub struct GmlPoint {
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "coordinates")]
    pub gml_coordinates: String
}

#[derive(Serialize, Deserialize)]
pub struct GmlPolygonProperty {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Polygon")]
    pub gml_polygon: GmlPolygon
}

#[derive(Serialize, Deserialize)]
pub struct GmlPolygon {
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
    pub gml_exterior: Option<GmlExterior>,
    #[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
    pub gml_interior: Option<Vec<GmlInterior>>
}

#[derive(Serialize, Deserialize)]
pub struct GmlInterior {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LinearRing")]
    pub gml_linear_ring: GmlInteriorGmlLinearRing
}

#[derive(Serialize, Deserialize)]
pub struct GmlInteriorGmlLinearRing {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "coordinates")]
    pub gml_coordinates: String
}

#[derive(Serialize, Deserialize)]
pub struct GmlExterior {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "LinearRing")]
    pub gml_linear_ring: GmlExteriorGmlLinearRing
}

#[derive(Serialize, Deserialize)]
pub struct GmlExteriorGmlLinearRing {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "coordinates")]
    pub gml_coordinates: String
}

#[derive(Serialize, Deserialize)]
pub struct StSpecialFeatures {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SpecialFeature")]
    pub st_special_feature: Vec<StSpecialFeature>
}

#[derive(Serialize, Deserialize)]
pub struct StSpecialFeature {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
    pub sf_main_feature: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "FeatureCode")]
    pub sf_feature_code: String,
    #[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
    pub sf_feature_additional_code: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct OpOperations {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Operation")]
    pub op_operation: Vec<OpOperation>
}

#[derive(Serialize, Deserialize)]
pub struct OpOperation {
    #[serde(rename = "@mainType")]
    pub main_type: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "OperationInfo", skip_serializing_if = "Option::is_none")]
    pub op_operation_info: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
    pub co_change_time: Option<String>,
    #[serde(rename = "OperationType")]
    pub op_operation_type: String,
    #[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
    pub op_completion_data: Option<OpCompletionData>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
    pub op_proposal_data: Option<OpProposalData>,
    #[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
    pub op_specifications: Option<OpSpecifications>,
    #[serde(rename = "Cutting", skip_serializing_if = "Option::is_none")]
    pub op_cutting: Option<OpCutting>,
    #[serde(rename = "Silviculture", skip_serializing_if = "Option::is_none")]
    pub op_silviculture: Option<OpSilviculture>
}

#[derive(Serialize, Deserialize)]
pub struct OpCompletionData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CompletionDate")]
    pub op_completion_date: String
}

#[derive(Serialize, Deserialize)]
pub struct OpSpecifications {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Specification")]
    pub op_specification: Vec<OpSpecification>
}

#[derive(Serialize, Deserialize)]
pub struct OpSpecification {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState")]
    pub co_change_state: String,
    #[serde(rename = "SpecificationCode")]
    pub op_specification_code: String
}

#[derive(Serialize, Deserialize)]
pub struct OpSilviculture {
}

#[derive(Serialize, Deserialize)]
pub struct OpProposalData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ProposalType")]
    pub op_proposal_type: String,
    #[serde(rename = "ProposalYear")]
    pub op_proposal_year: String
}

#[derive(Serialize, Deserialize)]
pub struct OpCutting {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CuttingVolume", skip_serializing_if = "Option::is_none")]
    pub op_cutting_volume: Option<String>,
    #[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
    pub op_assortments: Option<OpAssortments>
}

#[derive(Serialize, Deserialize)]
pub struct OpAssortments {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Assortment")]
    pub op_assortment: Vec<OpAssortment>
}

#[derive(Serialize, Deserialize)]
pub struct OpAssortment {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "TreeSpecies")]
    pub op_tree_species: String,
    #[serde(rename = "StemType")]
    pub op_stem_type: String,
    #[serde(rename = "AssortmentVolume", skip_serializing_if = "Option::is_none")]
    pub op_assortment_volume: Option<String>,
    #[serde(rename = "AssortmentPercent", skip_serializing_if = "Option::is_none")]
    pub op_assortment_percent: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct TsTreeStandData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<TsTreeStandDataDate>
}

#[derive(Serialize, Deserialize)]
pub struct TsTreeStandDataDate {
    #[serde(rename = "@date")]
    pub date: String,
    #[serde(rename = "@type")]
    pub ts_tree_stand_data_date_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
    pub tst_tree_strata: Option<TstTreeStrata>,
    #[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
    pub dts_dead_tree_strata: Option<DtsDeadTreeStrata>,
    #[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
    pub tss_tree_stand_summary: Option<TssTreeStandSummary>
}

#[derive(Serialize, Deserialize)]
pub struct DtsDeadTreeStrata {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "DeadTreeStratum")]
    pub dts_dead_tree_stratum: Vec<DtsDeadTreeStratum>
}

#[derive(Serialize, Deserialize)]
pub struct DtsDeadTreeStratum {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "DeadTreeType")]
    pub dts_dead_tree_type: String,
    #[serde(rename = "TreeSpecies")]
    pub dts_tree_species: String,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub dts_mean_diameter: Option<String>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub dts_volume: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct TstTreeStrata {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStratum")]
    pub tst_tree_stratum: Vec<TstTreeStratum>
}

#[derive(Serialize, Deserialize)]
pub struct TstTreeStratum {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "StratumNumber")]
    pub tst_stratum_number: String,
    #[serde(rename = "TreeSpecies")]
    pub tst_tree_species: String,
    #[serde(rename = "Storey")]
    pub tst_storey: String,
    #[serde(rename = "Age")]
    pub tst_age: String,
    #[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
    pub tst_basal_area: Option<String>,
    #[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
    pub tst_stem_count: Option<String>,
    #[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
    pub tst_mean_diameter: Option<String>,
    #[serde(rename = "MeanHeight")]
    pub tst_mean_height: String,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub tst_volume: Option<String>,
    #[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
    pub tst_saw_log_percent: Option<String>,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub tst_saw_log_volume: Option<String>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub tst_pulp_wood_volume: Option<String>,
    #[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
    pub tst_volume_growth: Option<String>,
    #[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
    pub co_data_source: Option<String>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub tst_leaf_biomass: Option<String>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub tst_branch_biomass: Option<String>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub tst_stem_biomass: Option<String>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub tst_stump_biomass: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct TssTreeStandSummary {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
    pub co_change_state: Option<String>,
    #[serde(rename = "MeanAge")]
    pub tss_mean_age: String,
    #[serde(rename = "BasalArea")]
    pub tss_basal_area: String,
    #[serde(rename = "StemCount")]
    pub tss_stem_count: String,
    #[serde(rename = "MeanDiameter")]
    pub tss_mean_diameter: String,
    #[serde(rename = "MeanHeight")]
    pub tss_mean_height: String,
    #[serde(rename = "Volume")]
    pub tss_volume: String,
    #[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
    pub tss_saw_log_volume: Option<String>,
    #[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
    pub tss_pulp_wood_volume: Option<String>,
    #[serde(rename = "VolumeGrowth")]
    pub tss_volume_growth: String,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub tss_value: Option<String>,
    #[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
    pub tss_value_growth_percent: Option<String>,
    #[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
    pub tss_development_class: Option<String>,
    #[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
    pub tss_leaf_biomass: Option<String>,
    #[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
    pub tss_branch_biomass: Option<String>,
    #[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
    pub tss_stem_biomass: Option<String>,
    #[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
    pub tss_stump_biomass: Option<String>,
    #[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
    pub tss_main_tree_species: Option<String>
}

