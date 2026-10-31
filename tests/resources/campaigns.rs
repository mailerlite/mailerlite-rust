//! Campaign request contracts and independently selectable failure scenarios.
use crate::support::{default_client, resource_mock};
use mailerlite_rs::{Error, resources::campaigns::*};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, method, path, query_param},
};

fn campaign() -> Value {
    json!({"id":"001","name":"Campaign","type":"regular","status":"draft"})
}
fn email() -> Email {
    Email::new("Subject", "Sender", "sender@example.com")
}

fn multivariate_settings(content: u8, sending_time: u8) -> MultivariateSettings {
    MultivariateSettings {
        test_variants: TestVariants {
            subject: vec!["Subject".into()],
            sender: vec![Sender {
                from: "sender@example.com".into(),
                from_name: "Sender".into(),
            }],
            content,
            sending_time,
        },
        test_split: 20,
        select_winner_by: Some(WinnerMetric::Opens),
        after_time_amount: Some(1),
        after_time_unit: Some(WaitUnit::Hours),
    }
}

#[tokio::test]
async fn create_rejects_more_than_eight_multivariate_combinations_before_http() {
    let server = MockServer::start().await;
    let request = CreateCampaign::multivariate(
        "Nine combinations",
        vec![MultivariateEmail::new(); 3],
        multivariate_settings(3, 3),
    );
    assert!(matches!(
        default_client(&server).campaigns().create(request).await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_rejects_multivariate_content_count_without_matching_emails() {
    let server = MockServer::start().await;
    let request = CreateCampaign::multivariate(
        "Missing second content",
        vec![MultivariateEmail::new()],
        multivariate_settings(2, 1),
    );
    assert!(matches!(
        default_client(&server).campaigns().create(request).await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn list_sends_status_type_and_page_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/campaigns"))
        .and(query_param("filter[status]", "draft"))
        .and(query_param("filter[type]", "regular"))
        .and(query_param("page", "2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(
                json!({"data":[campaign()],"meta":{"current_page":2,"last_page":2}}),
            ),
        )
        .expect(1)
        .mount(&server)
        .await;
    let response = default_client(&server)
        .campaigns()
        .list()
        .status(CampaignStatus::Draft)
        .kind(CampaignFilterType::Regular)
        .page(2)
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data.len(), 1);
    assert_eq!(response.data.next_page(), None);
}

#[tokio::test]
async fn get_preserves_a_zero_prefixed_identifier() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/campaigns/001",
        200,
        Some(json!({"data":campaign()})),
        None,
    )
    .await;
    assert_eq!(
        default_client(&server)
            .campaigns()
            .get("001")
            .await
            .unwrap()
            .data
            .id
            .as_str(),
        "001"
    );
}

#[tokio::test]
async fn create_regular_campaign_sends_required_email_and_groups() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns", 200, Some(json!({"data":campaign()})),
        Some(json!({"name":"Campaign","type":"regular","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}],"groups":["004"]}))).await;
    default_client(&server)
        .campaigns()
        .create(CreateCampaign::new("Campaign", email()).groups(["004"]))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_sends_replacement_name_and_email() {
    let server = MockServer::start().await;
    resource_mock(&server, "PUT", "/api/campaigns/001", 200, Some(json!({"data":campaign()})),
        Some(json!({"name":"Renamed","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}]}))).await;
    default_client(&server)
        .campaigns()
        .update("001", UpdateCampaign::new("Renamed", email()))
        .await
        .unwrap();
}

#[tokio::test]
async fn schedule_instant_sends_delivery_mode() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/campaigns/001/schedule",
        200,
        Some(json!({"data":campaign()})),
        Some(json!({"delivery":"instant"})),
    )
    .await;
    default_client(&server)
        .campaigns()
        .schedule("001", ScheduleCampaign::instant())
        .await
        .unwrap();
}

#[tokio::test]
async fn cancel_posts_to_the_cancel_endpoint() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/campaigns/001/cancel",
        200,
        Some(json!({"data":campaign()})),
        None,
    )
    .await;
    default_client(&server)
        .campaigns()
        .cancel("001")
        .await
        .unwrap();
}

#[tokio::test]
async fn activity_filters_clicked_subscribers_and_decodes_counts() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/campaigns/001/reports/subscriber-activity"))
        .and(query_param("filter[type]", "clicked"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"005","clicks_count":1,"opens_count":2}],"meta":{"current_page":1,"last_page":1}})))
        .expect(1).mount(&server).await;
    let response = default_client(&server)
        .campaigns()
        .activity("001")
        .kind(ActivityType::Clicked)
        .send()
        .await
        .unwrap();
    assert_eq!(response.data.data[0].clicks_count, Some(1));
}

#[tokio::test]
async fn delete_accepts_empty_204() {
    let server = MockServer::start().await;
    resource_mock(&server, "DELETE", "/api/campaigns/001", 204, None, None).await;
    assert_eq!(
        default_client(&server)
            .campaigns()
            .delete("001")
            .await
            .unwrap()
            .metadata
            .status
            .as_u16(),
        204
    );
}

#[tokio::test]
async fn schedule_preserves_field_validation_errors_without_retrying() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns/001/schedule", 422,
        Some(json!({"message":"Invalid schedule","errors":{"schedule.date":["Must be in the future"]}})),
        Some(json!({"delivery":"scheduled","schedule":{"date":"2020-01-01","hours":"10","minutes":"00"}}))).await;
    let error = default_client(&server)
        .campaigns()
        .schedule("001", ScheduleCampaign::scheduled("2020-01-01", "10", "00"))
        .await
        .unwrap_err();
    let Error::Api(error) = error else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 422);
    assert!(error.validation_errors().contains_key("schedule.date"));
}

#[tokio::test]
async fn create_multivariate_email_omits_subject_and_sender_derived_from_variants() {
    let server = MockServer::start().await;
    let params = CreateCampaign::multivariate(
        "Content test",
        vec![MultivariateEmail::new().content("<p>Variant A</p>")],
        MultivariateSettings {
            test_variants: TestVariants {
                subject: vec!["Subject A".into()],
                sender: vec![Sender {
                    from: "sender@example.com".into(),
                    from_name: "Sender".into(),
                }],
                content: 1,
                sending_time: 1,
            },
            test_split: 20,
            select_winner_by: Some(WinnerMetric::Opens),
            after_time_amount: Some(1),
            after_time_unit: Some(WaitUnit::Hours),
        },
    );
    let expected = json!({"name":"Content test","type":"multivariate","emails":[{"content":"<p>Variant A</p>"}],"multivariate_settings":{"test_variants":{"subject":["Subject A"],"sender":[{"from":"sender@example.com","from_name":"Sender"}],"content":1,"sending_time":1},"test_split":20,"select_winner_by":"o","after_time_amount":1,"after_time_unit":"h"}});
    assert_eq!(serde_json::to_value(&params).unwrap(), expected);
    Mock::given(method("POST"))
        .and(path("/api/campaigns"))
        .and(body_json(expected))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":campaign()})))
        .expect(1)
        .mount(&server)
        .await;
    default_client(&server)
        .campaigns()
        .create(params)
        .await
        .unwrap();
}

#[tokio::test]
async fn create_rejects_a_name_over_255_characters_before_http() {
    let server = MockServer::start().await;
    let result = default_client(&server)
        .campaigns()
        .create(CreateCampaign::new("é".repeat(256), email()))
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_accepts_255_unicode_characters_without_counting_utf8_bytes() {
    let server = MockServer::start().await;
    let name = "é".repeat(255);
    resource_mock(&server, "POST", "/api/campaigns", 201, Some(json!({"data":campaign()})),
        Some(json!({"name":name,"type":"regular","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}]}))).await;
    default_client(&server)
        .campaigns()
        .create(CreateCampaign::new(name, email()))
        .await
        .unwrap();
}

#[tokio::test]
async fn create_rejects_ab_split_below_five_percent_before_http() {
    let server = MockServer::start().await;
    let result = default_client(&server)
        .campaigns()
        .create(CreateCampaign::ab(
            "AB",
            email(),
            AbSettings::subject("B", 4, WinnerMetric::Opens, 1, WaitUnit::Hours),
        ))
        .await;
    assert!(matches!(result, Err(Error::Config(_))));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn activity_rejects_an_unsupported_page_size_before_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .campaigns()
            .activity("001")
            .limit(11)
            .send()
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn activity_accepts_maximum_documented_page_size() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/campaigns/001/reports/subscriber-activity"))
        .and(query_param("limit", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[]})))
        .expect(1)
        .mount(&server)
        .await;
    assert!(
        default_client(&server)
            .campaigns()
            .activity("001")
            .limit(100)
            .send()
            .await
            .unwrap()
            .data
            .data
            .is_empty()
    );
}

#[tokio::test]
async fn get_retains_status_and_body_when_a_required_response_field_is_missing() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "GET",
        "/api/campaigns/001",
        200,
        Some(json!({"data":{"id":"001","name":"Campaign","type":"regular"}})),
        None,
    )
    .await;
    let Error::Decode(error) = default_client(&server)
        .campaigns()
        .get("001")
        .await
        .unwrap_err()
    else {
        panic!("expected decoding error")
    };
    assert_eq!(error.status().as_u16(), 200);
    assert!(error.detail().contains("status"));
    assert!(!error.body().is_empty());
}

#[tokio::test]
async fn schedule_does_not_retry_a_server_failure_even_when_read_retries_are_enabled() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/campaigns/001/schedule"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    assert!(matches!(
        crate::support::client(&server)
            .campaigns()
            .schedule("001", ScheduleCampaign::instant())
            .await,
        Err(Error::Api(_))
    ));
}

#[tokio::test]
async fn create_sending_time_ab_accepts_exactly_one_hundred_percent() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"name":"Delivery times","type":"ab","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}],"ab_settings":{"test_type":"sending_time","test_split":100}}))).await;
    default_client(&server)
        .campaigns()
        .create(CreateCampaign::ab(
            "Delivery times",
            email(),
            AbSettings::sending_time(),
        ))
        .await
        .unwrap();
}

async fn assert_sending_time_split_rejected(split: u8) {
    let server = MockServer::start().await;
    let mut settings = AbSettings::sending_time();
    settings.test_split = split;
    assert!(matches!(
        default_client(&server)
            .campaigns()
            .create(CreateCampaign::ab("Delivery times", email(), settings))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_sending_time_ab_rejects_ninety_nine_percent_before_http() {
    assert_sending_time_split_rejected(99).await;
}

#[tokio::test]
async fn create_sending_time_ab_rejects_one_hundred_one_percent_before_http() {
    assert_sending_time_split_rejected(101).await;
}

#[tokio::test]
async fn create_subject_ab_accepts_five_percent_lower_boundary() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"name":"Subjects","type":"ab","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}],"ab_settings":{"test_type":"subject","test_split":5,"select_winner_by":"o","after_time_amount":1,"after_time_unit":"h","b_value":{"subject":"Alternative"}}}))).await;
    default_client(&server)
        .campaigns()
        .create(CreateCampaign::ab(
            "Subjects",
            email(),
            AbSettings::subject("Alternative", 5, WinnerMetric::Opens, 1, WaitUnit::Hours),
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn create_sender_ab_accepts_twenty_five_percent_upper_boundary() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"name":"Senders","type":"ab","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}],"ab_settings":{"test_type":"sender","test_split":25,"select_winner_by":"c","after_time_amount":2,"after_time_unit":"d","b_value":{"from":"alternative@example.com","from_name":"Alternative sender"}}}))).await;
    default_client(&server)
        .campaigns()
        .create(CreateCampaign::ab(
            "Senders",
            email(),
            AbSettings::sender(
                "alternative@example.com",
                "Alternative sender",
                25,
                WinnerMetric::Clicks,
                2,
                WaitUnit::Days,
            ),
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn create_sender_ab_rejects_twenty_six_percent_before_http() {
    let server = MockServer::start().await;
    assert!(matches!(
        default_client(&server)
            .campaigns()
            .create(CreateCampaign::ab(
                "Senders",
                email(),
                AbSettings::sender(
                    "alternative@example.com",
                    "Alternative sender",
                    26,
                    WinnerMetric::Clicks,
                    2,
                    WaitUnit::Days
                )
            ))
            .await,
        Err(Error::Config(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_resend_selects_unopened_recipients_and_a_second_subject() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"name":"Resend","type":"resend","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}],"resend_settings":{"test_type":"subject","select_winner_by":"o","b_value":{"subject":"A second subject"}}}))).await;
    default_client(&server)
        .campaigns()
        .create(CreateCampaign::resend(
            "Resend",
            email(),
            ResendSettings::new("A second subject", WinnerMetric::Opens),
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn update_resend_selects_unclicked_recipients_without_changing_campaign_type() {
    let server = MockServer::start().await;
    resource_mock(&server, "PUT", "/api/campaigns/001", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"name":"Resend","emails":[{"subject":"Subject","from_name":"Sender","from":"sender@example.com"}],"resend_settings":{"test_type":"subject","select_winner_by":"c","b_value":{"subject":"A second subject"}}}))).await;
    default_client(&server)
        .campaigns()
        .update(
            "001",
            UpdateCampaign::new("Resend", email()).resend_settings(ResendSettings::new(
                "A second subject",
                WinnerMetric::Clicks,
            )),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn schedule_next_day_resend_omits_explicit_date_and_timezone() {
    let server = MockServer::start().await;
    resource_mock(
        &server,
        "POST",
        "/api/campaigns/001/schedule",
        200,
        Some(json!({"data":campaign()})),
        Some(json!({"delivery":"instant","resend":{"delivery":"day"}})),
    )
    .await;
    default_client(&server)
        .campaigns()
        .schedule(
            "001",
            ScheduleCampaign::instant().resend(ResendSchedule {
                delivery: ResendDelivery::Day,
                date: None,
                hours: None,
                minutes: None,
                timezone_id: None,
            }),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn schedule_resend_preserves_explicit_date_time_and_numeric_timezone() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns/001/schedule", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"delivery":"scheduled","schedule":{"date":"2027-01-02","hours":"09","minutes":"05"},"resend":{"delivery":"scheduled","date":"2027-01-04","hours":"08","minutes":"07","timezone_id":4}}))).await;
    default_client(&server)
        .campaigns()
        .schedule(
            "001",
            ScheduleCampaign::scheduled("2027-01-02", "09", "05").resend(ResendSchedule {
                delivery: ResendDelivery::Scheduled,
                date: Some("2027-01-04".into()),
                hours: Some("08".into()),
                minutes: Some("07".into()),
                timezone_id: Some(4),
            }),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn schedule_sending_time_ab_preserves_two_times_and_string_timezone_ids() {
    let server = MockServer::start().await;
    resource_mock(&server, "POST", "/api/campaigns/001/schedule", 200,
        Some(json!({"data":campaign()})),
        Some(json!({"delivery":"instant","sending_time_test_schedule":[{"date":"2027-01-02","hours":"09","minutes":"05","timezone_id":"004"},{"date":"2027-01-03","hours":"08","minutes":"07","timezone_id":"004"}]}))).await;
    default_client(&server)
        .campaigns()
        .schedule(
            "001",
            ScheduleCampaign::instant().sending_times([
                SendingTime {
                    date: "2027-01-02".into(),
                    hours: "09".into(),
                    minutes: "05".into(),
                    timezone_id: "004".into(),
                },
                SendingTime {
                    date: "2027-01-03".into(),
                    hours: "08".into(),
                    minutes: "07".into(),
                    timezone_id: "004".into(),
                },
            ]),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn schedule_resend_preserves_server_date_validation_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/api/campaigns/001/schedule"))
        .and(body_json(json!({"delivery":"scheduled","schedule":{"date":"2027-01-02","hours":"09","minutes":"05"},"resend":{"delivery":"scheduled","date":"2027-01-02","hours":"09","minutes":"06"}})))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({"message":"Invalid resend schedule","errors":{"resend.date":["Please select a time that is at least 24 hours from first schedule"]}})))
        .expect(1).mount(&server).await;
    let Error::Api(error) = crate::support::client(&server)
        .campaigns()
        .schedule(
            "001",
            ScheduleCampaign::scheduled("2027-01-02", "09", "05").resend(ResendSchedule {
                delivery: ResendDelivery::Scheduled,
                date: Some("2027-01-02".into()),
                hours: Some("09".into()),
                minutes: Some("06".into()),
                timezone_id: None,
            }),
        )
        .await
        .unwrap_err()
    else {
        panic!("expected API error")
    };
    assert_eq!(error.status().as_u16(), 422);
    assert_eq!(
        error.validation_errors()["resend.date"],
        ["Please select a time that is at least 24 hours from first schedule"]
    );
}
