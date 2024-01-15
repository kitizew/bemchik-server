use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn main(){
    let to_user = "jura.kupetskyi@gmail.com";
    let code = "444";
    let text = format!("<h1>Ваш код підтвердження {code}</h1>");

    let email = Message::builder()
        .from("bemchik <napevnotvink@gmail.com>".parse().unwrap())
        .to(to_user.parse().unwrap())
        .subject("Код підтвердження")
        .header(ContentType::TEXT_HTML)
        .body(String::from(text))
        .unwrap();

    let creds = Credentials::new("napevnotvink@gmail.com".to_owned(), "xbey tngw qbma pgly".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}