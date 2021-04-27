/*
 * Customer Journey as a Service (CJaaS)
 *
 * Something amazing, something special - the Customer Journey as a Service (CJaaS) is a core data layer to enable Journeys across products built upon serverless multi-cloud architecture, to be available as a SaaS service for applications inside and outside of Cisco. [**Cisco Experimental - Not For Production Use**]
 *
 * The version of the OpenAPI document: 0.5.0
 * Contact: cjaas-earlyaccess@cisco.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineOrchestration {
    #[serde(rename = "typeOfOffer", skip_serializing_if = "Option::is_none")]
    pub type_of_offer: Option<String>,
    #[serde(rename = "displayTitle", skip_serializing_if = "Option::is_none")]
    pub display_title: Option<String>,
    #[serde(rename = "displayMessage", skip_serializing_if = "Option::is_none")]
    pub display_message: Option<String>,
    #[serde(rename = "offerImage", skip_serializing_if = "Option::is_none")]
    pub offer_image: Option<String>,
    #[serde(rename = "maxWidth", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i32>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "oneTimeJWT", skip_serializing_if = "Option::is_none")]
    pub one_time_jwt: Option<String>,
    #[serde(rename = "connectID", skip_serializing_if = "Option::is_none")]
    pub connect_id: Option<String>,
    #[serde(rename = "validTill", skip_serializing_if = "Option::is_none")]
    pub valid_till: Option<String>,
}

impl OnlineOrchestration {
    pub fn new() -> OnlineOrchestration {
        OnlineOrchestration {
            type_of_offer: None,
            display_title: None,
            display_message: None,
            offer_image: None,
            max_width: None,
            link: None,
            one_time_jwt: None,
            connect_id: None,
            valid_till: None,
        }
    }
}


