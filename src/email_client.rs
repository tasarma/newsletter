use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;

use crate::domain::SubscriberEmail;

#[derive(Debug)]
pub struct EmailClient {
    sender: SubscriberEmail,
    http_client: Client,
    base_url: String,
    autherization_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        autherization_token: Secret<String>,
    ) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            base_url,
            autherization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        let url = format!("{}/email", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            text_body: text_content.to_owned(),
            html_body: html_content.to_owned(),
        };
        let builder = self
            .http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.autherization_token.expose_secret(),
            )
            .json(&request_body);
        Ok(())
    }
}

#[derive(Serialize)]
struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    text_body: String,
    html_body: String,
}

#[cfg(test)]
mod tests {
    use fake::{
        Fake, Faker,
        faker::{
            internet::ar_sa::SafeEmail,
            lorem::ar_sa::{Paragraph, Sentence},
        },
    };
    use secrecy::Secret;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::any};

    use crate::{domain::SubscriberEmail, email_client::EmailClient};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Assert
    }
}
