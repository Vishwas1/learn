## DIDComm  

The core capability of an Aries agent is peer-to-peer messaging—connecting with other agents and sending and receiving messages to and from those agents to accomplish some interaction.

The interaction might be as simple as sending a text message, or more complex, such as negotiating the issuing of a verifiable credential or the presentation of a proof.

With Aries, there are two levels of messaging protocols: 

- At the lower level is the ***DIDComm protocol***, the method by which messages are exchanged, *irrespective of the message content*.  The participants exchange DIDs, and the attributes of those DIDs—signing and encryption keys, and service endpoint details—enabling end-to-end secure messaging via DIDComm. 
- At a higher level are the ***Aries protocols***, the many protocols that define back-and-forth sequences of specific messages (content) to accomplish some shared goal. Aries protocols *deal with content* that goes into the envelopes that are delivered by DIDComm.

> As a aeris dev, we do not go deeper in lower level i.e DIDcomm, intead, we need to work on higher level. Where we have to worry about the *content* or business logic of those messaging 


## Aries protocol messages 

Basic message protocol - [RFC 0095](https://github.com/hyperledger/aries-rfcs/tree/main/features/0095-basic-message)

```
{
    "@id": "caec7c09-e0ef-4e2a-9708-e69316a0b73f",
    "@type": "https://didcomm.org/basicmessage/1.0/message",
    "~l10n": { "locale": "en" },
    "sent_time": "2023-01-15 18:42:01Z",
    "content": "Your hovercraft is full of eels."
}

```

### Message Decorators

"decorators" - standardized message elements that define cross-cutting behavior. 

"Cross-cutting" means the behavior is the same when used in different protocols

Decorator items in messages are indicated by a `~` prefix. 

- The `~l10n` item in the previous section is an example of a decorator - provide information to enable the localization of the message
- The `~thread` decorator is used to link the messages in a protocol instance.
- The `~attachments` decorator
- The `~tracing` decorator
- The `~timing` decorator 



> **Unlike HTTP requests, all DIDComm messages are asynchronous.**


DID Exhcange Request Mesage - [RFC 0023](https://github.com/hyperledger/aries-rfcs/tree/main/features/0023-did-exchange#1-exchange-request)

```
{
  "@id": "5678876542345",
  "@type": "https://didcomm.org/didexchange/1.1/request",
  "~thread": { 
      "thid": "5678876542345",
      "pthid": "<id of invitation>"
  },
  "label": "Bob",
  "goal_code": "aries.rel.build",
  "goal": "To create a relationship",
  "did": "B.did@B:A",
  "did_doc~attach": {
      "@id": "d2ab6f2b-5646-4de3-8c02-762f553ab804",
      "mime-type": "application/json",
      "data": {
         "base64": "eyJ0eXAiOiJKV1Qi... (bytes omitted)",
         "jws": {
            "header": {
               "kid": "did:key:z6MkmjY8GnV5i9YTDtPETC2uUAW6ejw3nk5mXF5yci5ab7th"
            },
            "protected": "eyJhbGciOiJFZERTQSIsImlhdCI6MTU4Mzg4... (bytes omitted)",
            "signature": "3dZWsuru7QAVFUCtTd0s7uc1peYEijx4eyt5... (bytes omitted)"
            }
      }
   }
}
```
--- 

Each Aries framework exposes a way to use the protocols that minimizes what the controller has to do, thus limiting the mistakes the controller can make. In ACA-Py, the endpoints made available via the Admin HTTP API requires the controller to provide the absolute minimum information necessary for ACA-Py to carry out an action.

In ACA-Py (and other Aries frameworks) the type of message to be sent is implied by the API endpoint used by the controller. 

In general, there is one API endpoint per message type. The content is provided as JSON data as defined by the API endpoint.


## Labs

[Using JSON-LD credentials using ACA-Py](https://aca-py.org/latest/demo/AliceWantsAJsonCredential/)