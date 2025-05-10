# Brevo SendTransacEmail API Documentation

https://developers.brevo.com/reference/sendtransacemail

## Overview

The SendTransacEmail API endpoint allows you to send transactional emails through Brevo (formerly Sendinblue). This endpoint can be used to send emails with either custom HTML content or based on a template with dynamic parameters.

## Endpoint Information

- **URL**: `https://api.brevo.com/v3/smtp/email`
- **Method**: POST
- **Headers**:
  - `api-key`: Your Brevo API key
  - `content-type`: application/json
  - `accept`: application/json

## Authentication

Authentication is handled via an API key which must be included in the request headers:

```
api-key: YOUR_API_KEY
```

You can obtain your API key from your Brevo account settings under SMTP & API.

## Request Parameters

The API accepts a JSON body with the following parameters:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `sender` | Object | Yes | Information about the sender |
| `to` | Array | Yes | List of recipient information |
| `subject` | String | No* | Email subject |
| `htmlContent` | String | No* | HTML content of the email |
| `textContent` | String | No* | Text content of the email |
| `templateId` | Integer | No* | ID of the template to use |
| `params` | Object | No | Dynamic parameters for template personalization |
| `tags` | Array | No | Tags for categorizing and tracking emails |
| `headers` | Object | No | Custom headers for the email |
| `replyTo` | Object | No | Reply-to address configuration |
| `bcc` | Array | No | List of BCC recipients |
| `cc` | Array | No | List of CC recipients |
| `attachment` | Array | No | Files to attach to the email |

\* Either `subject` + `htmlContent`/`textContent` OR `templateId` must be provided.

### Parameter Details

#### Sender Object
```json
{
  "name": "Sender Name",
  "email": "sender@example.com"
}
```

#### Recipient Object
```json
{
  "email": "recipient@example.com",
  "name": "Recipient Name"
}
```

#### Reply-To Object
```json
{
  "email": "replyto@example.com",
  "name": "Reply-To Name"
}
```

#### Headers Object
```json
{
  "X-Mailin-custom": "custom_header_1:custom_value_1|custom_header_2:custom_value_2",
  "charset": "iso-8859-1"
}
```

#### Params Object
```json
{
  "parameter1": "value1",
  "parameter2": "value2",
  "ORDER": "12345",
  "DATE": "12/06/2023"
}
```

#### Attachment Object
```json
[
  {
    "url": "https://example.com/file.pdf",
    "content": "base64_encoded_content",
    "name": "file.pdf"
  }
]
```

## Usage Examples

### Send Email with Custom HTML

```rust
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

async fn send_transactional_email() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let api_key = "YOUR_API_KEY";

    let response = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("accept", "application/json")
        .header("api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "sender": {
                "name": "Sender Name",
                "email": "sender@example.com"
            },
            "to": [
                {
                    "email": "recipient@example.com",
                    "name": "Recipient Name"
                }
            ],
            "subject": "Hello world",
            "htmlContent": "<html><head></head><body><p>Hello,</p><p>This is my first transactional email sent from Brevo.</p></body></html>"
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    println!("Response: {:?}", result);

    Ok(())
}
```

### Send Email Using Template with Dynamic Parameters

```rust
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

async fn send_template_email() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let api_key = "YOUR_API_KEY";

    let response = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("accept", "application/json")
        .header("api-key", api_key)
        .header("content-type", "application/json")
        .json(&json!({
            "to": [
                {
                    "email": "recipient@example.com",
                    "name": "John Doe"
                }
            ],
            "templateId": 8,
            "params": {
                "name": "John",
                "surname": "Doe",
                "ORDER": "12345",
                "DATE": "12/06/2023"
            },
            "headers": {
                "X-Mailin-custom": "custom_header_1:custom_value_1|custom_header_2:custom_value_2"
            }
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    println!("Response: {:?}", result);

    Ok(())
}
```

## Response

### Success Response

Upon successful submission, the API returns a 201 status code with a response body containing the message ID:

```json
{
  "messageId": "<201798300811.5787683@smtp-relay.sendinblue.com>"
}
```

### Error Response

In case of errors, the API returns an appropriate status code (typically 400 for validation errors) along with an error message:

```json
{
  "code": "invalid_parameter",
  "message": "Invalid email address"
}
```

## Tracking Email Events

You can track email events using Brevo's webhook functionality. The following events can be tracked:

- Sent
- Delivered
- Opened
- Clicked
- Soft Bounce
- Hard Bounce
- Invalid Email
- Deferred
- Complaint
- Unsubscribed
- Blocked
- Error

To set up webhooks:
1. Go to the transactional webhooks page in your Brevo account
2. Specify the URL where you want to receive webhook data
3. Select the events you want to track

Alternatively, you can manage webhooks via the API using the Create/Update webhook endpoints.

## Webhook Event Data Structure

When an event occurs, Brevo will send a POST request to your webhook URL with data in the following format:

```json
{
  "event": "delivered",
  "email": "recipient@example.com",
  "id": 26224,
  "date": "YYYY-MM-DD HH:mm:ss",
  "ts": 1598634509,
  "message-id": "<201798300811.5787683@smtp-relay.sendinblue.com>",
  "ts_event": 1598034509,
  "subject": "Subject Line",
  "tag": "[\"transactionalTag\"]",
  "sending_ip": "185.41.28.109",
  "ts_epoch": 1598634509223,
  "tags": [
    "myFirstTransactional"
  ]
}
```

## Best Practices

1. **Use Templates**: For consistent email designs, create templates through the Brevo dashboard and use the `templateId` parameter.

2. **Dynamic Content**: Leverage the `params` object to personalize emails with dynamic content.

3. **Tagging**: Add tags to your emails for better categorization and tracking.

4. **Error Handling**: Implement proper error handling to manage API failures.

5. **Rate Limits**: Be aware of Brevo's rate limits and quota for sending emails.

## Resources

- [Brevo API Documentation](https://developers.brevo.com/)
- [Brevo API Client Libraries](https://developers.brevo.com/docs/api-clients)
- [Send Transactional Email Tutorial](https://developers.brevo.com/docs/send-a-transactional-email)
