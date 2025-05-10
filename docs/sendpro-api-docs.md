# SendPro API Documentation

## Overview

The SendPro API by Flowmailer provides a reliable email API for sending and managing email messages.

## Authentication

Authentication is done using a Bearer token in the Authorization header.

```
Authorization: Bearer {access_token}
```

## API Endpoints

### GET Messages

Retrieves messages with various filtering options.

**Request Example:**

```php
<?php
$account_id = '';
$daterange = '2021-12-30T00:00:00Z,2021-12-31T00:00:00Z';
$flow_ids = '';
$addevents = false;
$addheaders = false;
$addonlinelink = false;
$addtags = false;
$sortfield = '';
$sortorder = '';
$range = 'items=:10';
$options = [
    'http' => [
        'ignore_errors' => true,
        'method' => 'GET',
        'header' => [
            sprintf('Authorization: Bearer %s', $access_token),
            'Accept: application/vnd.flowmailer.v1.12+json',
            sprintf('Range: %s', $range),
        ],
    ],
];
$context = stream_context_create($options);
$matrix = ';'.http_build_query([
    'daterange' => $daterange,
    'flow_ids' => $flow_ids,
], '', ';');
$query = '?'.http_build_query([
    'addevents' => $addevents,
    'addheaders' => $addheaders,
    'addonlinelink' => $addonlinelink,
    'addtags' => $addtags,
    'sortfield' => $sortfield,
    'sortorder' => $sortorder,
]);
$response = file_get_contents(
    sprintf(
        'https://api.flowmailer.net/%s/messages%s%s',
        $account_id,
        $matrix,
        $query
    ),
    false,
    $context
);
$response = json_decode($response);
$statuscode = substr($http_response_header[0], 9, 3);
```

**HTTP Request:**

```
GET /545/messages;daterange=2021-12-30T00:00:00Z,2021-12-31T00:00:00Z?addevents=true&addheaders=true&addonlinelink=true HTTP/1.1
Host: api.flowmailer.net
Authorization: Bearer {access_token}
Accept: application/vnd.flowmailer.v1.12+json;charset=UTF-8
Range: items=:10
```

**HTTP Response:**

```
HTTP/1.1 206 Partial Content
Content-Range: items 94U34C1I64OJ4CPG64PJ0D1H6OO34CPI60P32C9I6CO32CPG6GO3EP9JC8PJCP9ICKSJ4PJ4C9IMCOPO6O:94V34C1I64OJ4CPG64PJ0D1J6CSJ4E1I60P32C9I6CO32CPG6GOJ4E1P65GM2P9L69IJ4PHPCKR6AP1H64/*
Next-Range: items=94V34C1I64OJ4CPG64PJ0D1J6CSJ4E1I60P32C9I6CO32CPG6GOJ4E1P65GM2P9L69IJ4PHPCKR6AP1H64:10
Content-Type: application/vnd.flowmailer.v1.12+json;charset=UTF-8
```

**Response Body Example:**

```json
[
  {
    "submitted": "2021-12-30T13:04:07.000Z",
    "id": "20211230130407e3b36e2e92fdbefc86",
    "transactionId": "5f133c15a8de4ddca8fff9a26652b513",
    "messageIdHeader": "<20211230130407e3b36e2e92fdbefc86@return.flowmailer.local>",
    "messageType": "EMAIL",
    "source": {
      "id": "6953",
      "description": "test source a"
    },
    "flow": {
      "id": "16801",
      "description": "test flow 1 (updated)"
    },
    "senderAddress": "support@flowmailer.local",
    "recipientAddress": "richard@flowmailer.local",
    "backendStart": "2021-12-30T13:04:13.122Z",
    "backendDone": "2021-12-30T13:04:28.577Z",
    "headersIn": [
      {
        "name": "Received",
        "value": "from 127.0.0.1 by (Flowmailer API) with HTTP for <richard@flowmailer.local>; Thu, 30 Dec 2021 14:04:08 +0100 (CET)"
      },
      {
        "name": "MIME-Version",
        "value": "1.0"
      },
      {
        "name": "From",
        "value": "Casper Mout <casper@flowmailer.local>"
      },
      {
        "name": "To",
        "value": "Richard van Looijen <richard@flowmailer.local>"
      },
      {
        "name": "X-Flow",
        "value": "flow1"
      },
      {
        "name": "Subject",
        "value": "test message 1"
      },
      {
        "name": "Content-Type",
        "value": "text/plain; charset=UTF-8"
      },
      {
        "name": "Content-Transfer-Encoding",
        "value": "quoted-printable"
      }
    ],
    "headersOut": [
      {
        "name": "MIME-Version",
        "value": "1.0"
      },
      {
        "name": "From",
        "value": "Casper Mout <casper@flowmailer.local>"
      },
      {
        "name": "To",
        "value": "Richard van Looijen <richard@flowmailer.local>"
      },
      {
        "name": "X-Flow",
        "value": "flow1"
      },
      {
        "name": "Subject",
        "value": "test message 1"
      },
      {
        "name": "Content-Type",
        "value": "text/plain; charset=UTF-8"
      },
      {
        "name": "Content-Transfer-Encoding",
        "value": "quoted-printable"
      },
      {
        "name": "Feedback-ID",
        "value": "545:545-6953:545-16801:flowmailer"
      },
      {
        "name": "Message-ID",
        "value": "<20211230130407e3b36e2e92fdbefc86@return.flowmailer.local>"
      },
      {
        "name": "Date",
        "value": "Thu, 30 Dec 2021 13:04:25 +0000"
      },
      {
        "name": "X-Job",
        "value": "fm-6953-16801"
      }
    ],
    "onlineLink": "https://web.flowmailer.net/viewonline.html?id=VRbzZZWMpsc:yqB0A-5zF1Nm5Ra5FC5KcA:I8SgIqmRZHC7oUQ0cOIVv8d-EjZC_GKiE6elZzbebzcluS05rh_0p2Q3WC3jRronKznxAP2D6f3X_pRx6r3W4QjKf0HJ1fhBCm9xKM3KeFX9lZe0xJiWhsYCgimBJiY2:2GnJcdfrRr9p7dKL9aU1pfg7VaqTSRCMMd-Yoo5pt9Y",
    "status": "DELIVERED",
    "subject": "test message 1",
    "from": "casper@flowmailer.local",
    "events": [
      {
        "id": "70844a11-30b2-467a-8bd2-bffb8992e958",
        "messageId": "20211230130407e3b36e2e92fdbefc86",
        "type": "DELIVERED",
        "received": "2021-12-30T13:04:28.898Z",
        "inserted": "2021-12-30T13:04:34.282Z",
        "snippet": "smtp;250 Ok",
        "mta": "[127.0.0.1] (127.0.0.1)",
        "data": null,
        "sourceMta": "mta.flowmailer.local"
      },
      {
        "id": null,
        "messageId": "20211230130407e3b36e2e92fdbefc86",
        "type": "PROCESSED",
        "received": "2021-12-30T13:04:28.577Z",
        "inserted": "2021-12-30T13:04:16.023Z",
        "snippet": null,
        "mta": null,
        "data": null
      },
      {
        "id": null,
        "messageId": "20211230130407e3b36e2e92fdbefc86",
        "type": "SUBMITTED",
        "received": "2021-12-30T13:04:07.000Z",
        "inserted": "2021-12-30T13:04:16.023Z",
        "snippet": null,
        "mta": null,
        "data": null
      }
    ],
    "messageDetailsLink": "https://web.flowmailer.net/viewmessage.html?id=VRbzZZWMpsc:bRjmfqt7SSFksLcJGFM-7g:ftgUpVhCyQxLVZyqon1QOcpakh346CvwRYh2ct16YdjgfQzdsU0U8PBkU6jAp1ILORApJB-EU35shb1Bm65v8KtWHuEr0_Qwe7Zl2S_6MBRJkmrFqdMMj0x_1tIvTG7s:jncD05ijczY6lHquBzdTwF02KR7Bf6w0ThDCss-aQiE&code=462940c8b48fd88aff19b30863637cdee6c0bab2",
    "fromAddress": {
      "name": "Casper Mout",
      "address": "casper@flowmailer.local"
    },
    "toAddressList": [
      {
        "name": "Richard van Looijen",
        "address": "richard@flowmailer.local"
      }
    ]
  },
  {
    "submitted": "2021-12-30T13:04:10.000Z",
    "id": "202112301304100c7ded281d3069537b",
    "transactionId": "4d09377e3d454dc8a69003ca8b1f0a63",
    "messageIdHeader": "<202112301304100c7ded281d3069537b@return.flowmailer.local>",
    "messageType": "EMAIL",
    "source": {
      "id": "6953",
      "description": "test source a"
    },
    "flow": {
      "id": "16814",
      "description": "test flow 2"
    },
    "senderAddress": "support@flowmailer.local",
    "recipientAddress": "richard@flowmailer.local",
    "backendStart": "2021-12-30T13:04:28.433Z",
    "backendDone": "2021-12-30T13:04:28.577Z",
    "headersIn": [
      {
        "name": "Received",
        "value": "from 127.0.0.1 by (Flowmailer API) with HTTP for <richard@flowmailer.local>; Thu, 30 Dec 2021 14:04:10 +0100 (CET)"
      },
      {
        "name": "MIME-Version",
        "value": "1.0"
      },
      {
        "name": "From",
        "value": "Casper Mout <casper@flowmailer.local>"
      },
      {
        "name": "To",
        "value": "Richard van Looijen <richard@flowmailer.local>"
      },
      {
        "name": "X-Flow",
        "value": "flow2"
      },
      {
        "name": "Subject",
        "value": "test message 2"
      },
      {
        "name": "Content-Type",
        "value": "text/plain; charset=UTF-8"
      },
      {
        "name": "Content-Transfer-Encoding",
        "value": "quoted-printable"
      }
    ],
    "headersOut": [
      /* Similar structure to previous message */
    ],
    "onlineLink": "https://web.flowmailer.net/viewonline.html?id=VRbzZZWMpsc:vOzNF1QJuDA8rJA0N8qXMw:wV7Ts97CcI2kjYEPXX9oBwaX_lzeNKb_fctcaxtXS8t22DmM-W1WgHKVHvjCqqNQChtDIOOY5765JChm-fufyoi9ej3Ozo-BM2d2KA-wE5_fYX0EsgpBVCo9LRfkXtJT:BNybztpiItklSftbSZuiRn-lZCsFfAaD6s13pBE4occ",
    "status": "DELIVERED",
    "subject": "test message 2",
    "from": "casper@flowmailer.local",
    "events": [
      /* Message events */
    ],
    "messageDetailsLink": "https://web.flowmailer.net/viewmessage.html?id=VRbzZZWMpsc:3_8jv4YsOTEFqY-2SUEZiQ:Qs6lIfp-38XbTCxnKsp_l5tu84qgfF6xt7eWodni00yY_HB2qbWI2sNlS7xqDHfIJW-f3uF1BlFP0ri8EWMXK8xa0YWsAZ57DrII04JUbVcDy1vCpLV3f9oay4ODWTay:w4juMDpotut85I2n1zeLzR72_L6cOalZEwKXHdWmFLE&code=aa4487413f9257a84fe1911fb4fd6717a2cbace2",
    "fromAddress": {
      "name": "Casper Mout",
      "address": "casper@flowmailer.local"
    },
    "toAddressList": [
      {
        "name": "Richard van Looijen",
        "address": "richard@flowmailer.local"
      }
    ]
  },
  {
    "submitted": "2021-12-30T13:04:12.000Z",
    "id": "20211230130412891aae52e2f9e6ed11",
    "transactionId": "5d7ac76c8f7b41c1b417a71505c9ef80",
    "messageIdHeader": null,
    "messageType": "EMAIL",
    "source": {
      "id": "6953",
      "description": "test source a"
    },
    "flow": {
      "id": "16814",
      "description": "test flow 2"
    },
    "senderAddress": "support@flowmailer.local",
    "recipientAddress": "richard@flowmailer.local",
    "backendStart": "2021-12-30T13:04:33.897Z",
    "backendDone": null,
    "headersIn": [
      /* Message headers */
    ],
    "status": "ERROR",
    "subject": "test message 3",
    "from": "casper@flowmailer.local",
    "events": [
      {
        "id": "4254bf5b-5482-40a4-979a-514c3721910b",
        "messageId": "20211230130412891aae52e2f9e6ed11",
        "type": "ERROR",
        "received": "2021-12-30T13:04:34.129Z",
        "inserted": "2021-12-30T13:04:34.194Z",
        "snippet": "The following has evaluated to null or missing:",
        "mta": null,
        "data": null,
        "extraData": {
          "errorText": "The following has evaluated to null or missing:\n==> var1 [in template \"48681\" at line 1, column 29]\n\n----\nTip: If the failing expression is known to be legally refer to something that's sometimes null or missing, either specify a default value like myOptionalVar!myDefault, or use <#if myOptionalVar??>when-present<#else>when-missing<\/#if>. (These only cover the last step of the expression; to cover the whole expression, use parenthesis: (myOptionalVar.foo)!myDefault, (myOptionalVar.foo)??\n----\n\n\t- Failed at: ${var1} [in template \"48681\" at line 1, column 27]\n",
          "errorAfter": "QUEUE"
        }
      },
      {
        "id": null,
        "messageId": "20211230130412891aae52e2f9e6ed11",
        "type": "SUBMITTED",
        "received": "2021-12-30T13:04:12.000Z",
        "inserted": "2021-12-30T13:04:33.928Z",
        "snippet": null,
        "mta": null,
        "data": null
      }
    ],
    "fromAddress": {
      "name": "Casper Mout",
      "address": "casper@flowmailer.local"
    },
    "toAddressList": [
      {
        "name": "Richard van Looijen",
        "address": "richard@flowmailer.local"
      }
    ]
  }
]
```

## Message Object Structure

A message object contains the following fields:

- `submitted`: Timestamp when the message was submitted
- `id`: Unique message identifier
- `transactionId`: Transaction identifier
- `messageIdHeader`: Message ID header
- `messageType`: Type of message (e.g., "EMAIL")
- `source`: Source information object
- `flow`: Flow information object
- `senderAddress`: Email address of the sender
- `recipientAddress`: Email address of the recipient
- `backendStart`: Timestamp when backend processing started
- `backendDone`: Timestamp when backend processing completed
- `headersIn`: Array of input headers
- `headersOut`: Array of output headers
- `onlineLink`: URL to view the message online
- `status`: Message status (e.g., "DELIVERED", "ERROR")
- `subject`: Email subject
- `from`: Sender email address
- `events`: Array of event objects related to the message
- `messageDetailsLink`: URL to view message details
- `fromAddress`: Object containing sender name and address
- `toAddressList`: Array of recipient objects with name and address

## Event Object Structure

Events have the following structure:

- `id`: Event identifier
- `messageId`: ID of the associated message
- `type`: Event type (e.g., "DELIVERED", "PROCESSED", "SUBMITTED", "ERROR")
- `received`: Timestamp when the event was received
- `inserted`: Timestamp when the event was inserted into the system
- `snippet`: Optional snippet of relevant information
- `mta`: Mail transfer agent information
- `data`: Additional data (usually null)
- `sourceMta`: Source mail transfer agent
- `extraData`: Additional structured data for certain event types (e.g., error information)

## Pagination

The API supports pagination using the Range header in the request and Content-Range in the response. A `Next-Range` header is provided to retrieve the next set of results.

## Error Handling

Error responses include detailed information about what went wrong, as shown in the third message example with "status": "ERROR".
