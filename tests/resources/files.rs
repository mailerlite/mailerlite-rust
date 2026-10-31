//! HTTP contract regression tests for the migrated simple-editor API.
#![allow(clippy::unwrap_used, reason = "Tests assert fixture outcomes")]

use crate::editor_support::{expect_request, fixture, mailerlite};
use mailerlite_rs::{
    Response,
    models::files::{FileSort, FileType, MediaFile},
    pagination::NumberedPage,
    resources::files::ListFilesOptions,
};
use mockito::{Mock, Server, ServerGuard};

#[tokio::test]
async fn get_sends_the_filters_and_decodes_the_files() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/files?filter%5Bname%5D=summer+sale&filter%5Btype%5D=image&sort=-size&page=1&limit=25",
        None,
        200,
        &fixture("files"),
    )
    .await;

    let options: ListFilesOptions = ListFilesOptions {
        name: Some("summer sale".to_string()),
        file_type: Some(FileType::Image),
        sort: Some(FileSort::SizeDesc),
        page: Some(1),
        limit: Some(25),
    };
    let response: Response<NumberedPage<MediaFile>> = mailerlite(&server)
        .files()
        .list()
        .options(options)
        .send()
        .await
        .unwrap();

    mock.assert_async().await;
    let image: &MediaFile = &response.data.data[0];
    assert_eq!(image.id, "301");
    assert_eq!(image.file_type, FileType::Image);
    assert_eq!(image.name, "summer sale banner.png");
    assert_eq!(
        image.url,
        "https://storage.example.com/files/summer-sale-banner.png"
    );
    assert_eq!(image.size, 48213);
    assert_eq!(image.width, Some(1200));
    assert_eq!(image.height, Some(600));
    assert_eq!(image.created_at, "2026-10-20 09:30:00");
    let pdf: &MediaFile = &response.data.data[1];
    assert_eq!(pdf.file_type, FileType::Pdf);
    assert_eq!(pdf.width, None);
    assert_eq!(pdf.height, None);
    assert_eq!(response.data.links.next, None);
    assert_eq!(response.data.meta.total, Some(2));
}

#[tokio::test]
async fn get_sends_no_query_without_options() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock =
        expect_request(&mut server, "GET", "/files", None, 200, &fixture("files")).await;

    mailerlite(&server).files().list().send().await.unwrap();

    mock.assert_async().await;
}

#[tokio::test]
async fn get_sends_an_empty_filter_and_an_unknown_type() {
    let mut server: ServerGuard = Server::new_async().await;
    let mock: Mock = expect_request(
        &mut server,
        "GET",
        "/files?filter%5Bname%5D=&filter%5Btype%5D=docx&sort=name",
        None,
        200,
        &fixture("files"),
    )
    .await;

    let options: ListFilesOptions = ListFilesOptions {
        name: Some(String::new()),
        file_type: Some(FileType::Other("docx".to_string())),
        sort: Some(FileSort::Name),
        ..Default::default()
    };
    mailerlite(&server)
        .files()
        .list()
        .options(options)
        .send()
        .await
        .unwrap();

    mock.assert_async().await;
}

#[test]
fn file_sort_values_match_the_api() {
    let sorts: Vec<String> = [
        FileSort::CreatedAt,
        FileSort::CreatedAtDesc,
        FileSort::Name,
        FileSort::NameDesc,
        FileSort::Size,
        FileSort::SizeDesc,
    ]
    .into_iter()
    .map(String::from)
    .collect();

    assert_eq!(
        sorts,
        vec![
            "created_at",
            "-created_at",
            "name",
            "-name",
            "size",
            "-size"
        ]
    );
}
