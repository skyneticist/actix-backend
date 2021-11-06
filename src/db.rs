use actix_web::web;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::models::{Album, Book, ProjectDetails, ProjectList, YoutubeVid};

// books
pub async fn get_books(client: &Client) -> Result<Vec<Book>, io::Error> {
    let sql = client.prepare("select * from books").await.unwrap();

    let books = client
        .query(&sql, &[])
        .await
        .expect("Error getting books")
        .iter()
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>();

    Ok(books)
}

pub async fn get_book(client: &Client, book_id: i32) -> Result<Vec<Book>, io::Error> {
    let sql = client.prepare("select * from books where id = $1 order by id")
        .await
        .unwrap();

    let book = client
        .query(&sql, &[&book_id])
        .await
        .expect("Error retrieving book with specified ID")
        .iter()
        .map(|row| Book::from_row_ref(row).unwrap())
        .collect::<Vec<Book>>();

    Ok(book)
}

pub async fn add_book(client: &Client, book: web::Json<Book>) -> Result<web::Json<Book>, io::Error> {
    let sql_insert = 
        "insert into books (id, title, author, release_date, publisher) values ($1, $2, $3, $4, $5)";
    let sql = client.prepare(&sql_insert).await.unwrap();
    client
        .query(
            &sql,
            &[
                &book.id,
                &book.title,
                &book.author,
                &book.release_date,
                &book.publisher,
            ],
        )
        .await
        .expect("Error adding book");

    Ok(book)
}

// projects
pub async fn get_projects(client: &Client) -> Result<Vec<ProjectList>, io::Error> {
    let sql = client.prepare("select * from project_list").await.unwrap();

    let projects = client
        .query(&sql, &[])
        .await
        .expect("Error getting project list")
        .iter()
        .map(|row| ProjectList::from_row_ref(row).unwrap())
        .collect::<Vec<ProjectList>>();

    Ok(projects)
}

pub async fn get_project(client: &Client, project_id: i32) -> Result<Vec<ProjectDetails>, io::Error> {
    let sql = client
        .prepare("select * from project_details where project_id = $1 order by id")
        .await
        .unwrap();

    let project = client
        .query(&sql, &[&project_id])
        .await
        .expect("Error getting project")
        .iter()
        .map(|row| ProjectDetails::from_row_ref(row).unwrap())
        .collect::<Vec<ProjectDetails>>();

    Ok(project)
}

pub async fn add_project(client: &Client, project: web::Json<ProjectDetails>) -> Result<web::Json<ProjectDetails>, io::Error> {
    let sql_insert = 
        "insert into project_details (id, title, in_progress, projects_id) values ($1, $2, $3, $4)";
    
    let sql = client.prepare(&sql_insert).await.unwrap();
    
    client.query(&sql, &[
        &project.id, 
        &project.title, 
        &project.in_progress
    ])
    .await
    .expect("Error adding project!");
    
    Ok(project)
}

// albums
pub async fn get_albums(client: &Client) -> Result<Vec<Album>, io::Error> {
    let sql = client.prepare("select * from albums").await.unwrap();

    let albums = client
        .query(&sql, &[])
        .await
        .expect("Error getting albums")
        .iter()
        .map(|row| Album::from_row_ref(row).unwrap())
        .collect::<Vec<Album>>();

    Ok(albums)  
}

pub async fn get_album(client: &Client, album_id: i32) -> Result<Vec<Album>, io::Error> {
    let sql = client
        .prepare("select * from albums where id = $1 order by id")
        .await
        .unwrap();
        
    let album = client
        .query(&sql, &[&album_id])
        .await
        .expect("Unable to retrieve album with specified ID")
        .iter()
        .map(|row| Album::from_row_ref(row).unwrap())
        .collect::<Vec<Album>>();

    Ok(album)
}

pub async fn add_album(client: &Client, album: web::Json<Album>) -> Result<web::Json<Album>, io::Error>  {
    let sql_insert = 
        "insert into albums (id, title, artist, release_date, label) values ($1, $2, $3, $4, $5)";
    
    let sql = client.prepare(&sql_insert).await.unwrap();

    client.query(&sql, &[
        &album.id,
        &album.title,
        &album.artist,
        &album.release_date,
        &album.label,
    ])
    .await
    .expect("Error adding album!");

    Ok(album)
}

// videos
pub async fn get_videos(client: &Client) -> Result<Vec<YoutubeVid>, io::Error> {
    let sql = client.prepare("select * from videos").await.unwrap();
    let videos = client
        .query(&sql, &[])
        .await
        .expect("Error retrieving videos")
        .iter()
        .map(|row| YoutubeVid::from_row_ref(row).unwrap())
        .collect::<Vec<YoutubeVid>>();

    Ok(videos)
}

pub async fn get_video(client: &Client, video_id: i32) -> Result<Vec<YoutubeVid>, io::Error> {
    let sql = client
        .prepare("select * from videos where id = $1 order by id")
        .await
        .unwrap();
        
    let yt_vid = client
        .query(&sql, &[&video_id])
        .await
        .expect("Unable to retrieve album with specified ID")
        .iter()
        .map(|row| YoutubeVid::from_row_ref(row).unwrap())
        .collect::<Vec<YoutubeVid>>();

    Ok(yt_vid)
}

pub async fn add_video(client: &Client, yt_vid: web::Json<YoutubeVid>) -> Result<web::Json<YoutubeVid>, io::Error> {
    let sql_insert = "insert into videos () values ();";
    let sql = client.prepare(&sql_insert).await.unwrap();
    client.query(&sql, &[
        &yt_vid.id,
        &yt_vid.title,
        &yt_vid.channel,
        &yt_vid.rank,
        &yt_vid.category,
    ])
    .await
    .expect("Error adding video!");

    Ok(yt_vid)
}
