use std::error::Error;
use sqlx::FromRow;
use sqlx::PgPool;
use sqlx::Row;


#[derive(Debug)]
struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(),Box<dyn Error>> {
    
    let query = "INSERT INTO book (title,author,isbn) VALUES ($1,$2,$3)";

    sqlx::query(query)
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .execute(pool)
    .await?;

    Ok(())
}

async fn update(book: &Book, pool: &sqlx::PgPool,isbn: &str) -> Result<(),Box<dyn Error>>{
    
    let query = "UPDATE book SET title = $1,author = $2 WHERE isbn = $3";

    sqlx::query(query)
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .execute(pool)
    .await?;

    Ok(())
}

async fn reed(conn: &PgPool) -> Result<Option<Book>, Box<dyn Error>> {
    let q = "SELECT title, author, isbn FROM book";
    let query = sqlx::query(q);

    let maybe_row = query.fetch_optional(conn).await?;

    let book = maybe_row.map(|row|{
        Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        }
    });

    Ok(book)
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let url = "postgres://ozgur:123456@localhost:5432/bookstore";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
        
    // let book = Book{
    //     title: "Postgresql sqlx rust".to_string(),
    //     author: "Özgür Zara Deniz Melek---".to_string(),
    //     isbn: "19854555".to_string()
    // };
    
    // create(&book, &pool).await?;
    // update(&book, &pool, &book.isbn).await?;

    let books = reed(&pool).await?;
    println!("{:?}",books[1].title);

    Ok(())
}
