## DIDComm Message Routing (Agent message routing)

Message routing—how messages get from one agent to another, and especially the routing when a mobile wallet is involved. 

**Mediator** and **Relay agents** are often necessary to enable messages to be securely delivered from the sender to recipient agent because:

- Mobile wallets do not have an endpoint (a physical address) that other agents can use for sending messages. Thus, it is impossible for other agents to send messages directly to a mobile wallet app.
- Entities may want to minimize correlation of their agent’s messages across relationships and so instead of having a unique endpoint per agent, many agents share a common endpoind such that their specific messages are "hidden in a crowd" of lots of other messages.

> mediators and relays are themselves DIDComm agents. They may only do routing activities or they may provide other services on behalf of their controlling entity

When a DIDComm message is sent from one agent to another, it is routed per the instructions provided by the receiver to the sender,

For example, using the following picture, Faber might be told by Alice to send messages to her phone (agent 5) via agents 3 and 4, and Faber might always send out messages via agent 2.   That means a message from Faber to Alice actually involves 5 different Aries agents, flowing from agents 1 to 5 via agents 2, 3 and then 4.


 ![img](/assets/msg-flow.png)

 When Faber and Alice communicate, they want their message to be private—***they don’t want the routing agents to see the contents of their message***.

 So, when there are agents in the flow between the two, Faber and Alice encrypt their messages with wrappers *that provide only enough information* for each routing agent to send (route) the message along on the next step of its end-to-end journey. 

## DID comm Mediators 

**mediator** is a routing agent that the recipient has directed the sender to use when sending a message. 

 In our Faber-Alice example earlier, the mediators are the agents Alice provides Faber through which Faber’s message must be routed.

  If there are *no mediators,* the message *will go directly to Alice*, so Faber’s agent need only encrypt the message for transporting the message to Alice. 


  > For each mediator Alice adds, Faber explicitly **adds another envelope**, another layer of encryption 


![img](/assets/msg-routing.png)

**Paper message analogy**, 

To carry the postal system analogy a bit further, suppose Alice and Bob work in different offices of a corporation and need to send paper messages to one another. 

Alice might write her message for Bob, 
- put it in an *envelope for Bob*
- put that into an *interoffice envelope* addressed to Bob
- and then into a *postal service envelope* addressed to the office in which Bob works

She mails the letter via the postal service and it gets delivered to the mailroom at Bob’s office: 
- The postal envelope is removed in the mailroom
- and the interoffice envelope is used by the internal mail system to get the letter to Bob
- Bob takes the message out of the interoffice envelope, and opens the inside envelope to read Alice’s message


> This matches the DIDComm world exactly, with Alice using encryption for the envelopes, and the postal service and mailroom as routing agents to facilitate delivery.


> Thus, Alice’s "list of agents" is just a list of encryption keys for Faber to use when preparing a message for Alice.


[Aries Mediator Service](https://github.com/hyperledger/aries-mediator-service)


## DIDComm Relays


The term **relay** is used in DIDComm to indicate that the *message is being routed* through *one or more additional agents* **without** the knowledge of the sender. 

Difference is that the 
- sender **must know** about all recipient’s mediators and explicitly add envelope wrappers for each, whereas 
- they **don’t know (or care)** about the recipient’s relays


To stretch our Alice and Bob paper message analogy a bit more, the mailroom in Bob’s building might deliver all the messages for Bob’s floor to an assistant who then distributes the messages to their intended recipients. 

> Alice doesn’t know about that process, and doesn't add an envelope for the assistant. 

>  The mailroom may add an envelope (encryption layer) around the message for the assistant to open, so only they know the message is for Bob


[Mediators & Relays - RFC 0046](https://github.com/hyperledger/aries-rfcs/blob/main/concepts/0046-mediators-and-relays/README.md)


To get back to our Faber and Alice picture, **agents 3 and 4 are mediators** because Alice *explicitly* tells Faber, "please send your messages to me through those agents." 

Since Faber is sending its outbound messages via agent 2, then **agent 2 is acting as a relay agent**, but not because Alice referenced, but rather because that’s how Faber’s agent is configured.

---

## Message Encryption 

[Encryption envelope RFC 0587](https://github.com/hyperledger/aries-rfcs/tree/main/features/0587-encryption-envelope-v2)

[Repudiation - RFC 0049](https://github.com/hyperledger/aries-rfcs/tree/main/concepts/0049-repudiation)

## Privacy 

Each time a mediator and relay agent receive a message and pass it on, they can record information about the message—metadata. 

Example: 
- Telephone metadata (who called who, when and for how long), routinely collected by telcos, enables the detailed tracking of an individual’s social and business relationships.
- Facebook and Google tracking your logins to other services provides them with information on your interests and frequency of use of services.
-  Internet Service Provides (ISPs) can likewise learn about your interests by tracking the websites you visit from your home computer and mobile phone

> Goals in designing the DIDComm messaging protocol was to try to limit the metadata exposed in passing messages. 


A security/privacy maven would likely be able to find others. In comparison with the examples we gave (telcos, "login with" services, ISPs), the amount of exposed metadata is far less with DIDComm.

In theory by using different vendors for different agents you could further reduce the metadata each participant can gather


## Lab

[Using Mediator](https://github.com/cloudcompass/ToIPLabs/blob/main/docs/LFS173xV2/MediatorLab.md)


## Notes

While routing is an important thing to understand about DIDComm and Aries agents in general, it is **especially important for mobile wallets** — mediators are a requirement for using Aries wallets.

As noted, other agents cannot send data directly to mobile wallets. All data that gets to any mobile application (wallets, games, email—any app) does so via the mobile app making a request to receive the data.

> Faber’s agent cannot directly send a message to Alice’s mobile wallet.


---
> ""Wait!," you say, what about all those notifications I get on my phone? Couldn’t those be used to send DIDComm messages to mobile wallets? While there is some ability to send notifications to a mobile app, the use of notifications is restricted by mobile operating system vendors (for example, Google and Apple). Only registered services may send notifications to an app, so the many arbitrary agents that might connect with a wallet can’t use OS level notifications for messaging the wallet. Further, mobile operating systems limit the volume of notifications that can be sent, and require that notifications have an associated message displayed to the user when sent. As such, notifications cannot be used as a way to send Aries/DIDComm messages to a mobile wallet."
---

> bottom line is that in order to operate, a **mobile wallet must have a mediator agent** through which all inbound messages must flow. 



## PRocess Flow 

Faber is an enterprise agent (an issuer/verifier) and is establishing a connection to Alice, who uses an Aries mobile wallet.

The maker of Alice’s mobile wallet has deployed a mediator (agent 4) that is used by all of the mobile wallet apps that have been installed on their customers’ mobile phones.

![img](/assets/fab-alice.png)

We need 4 DID and DIDDocs by each participant for each relataionship:

![img](/assets/did-doc-relantion.png)

- Alice for Faber (`AF(DID/DIDDoc)`)
- Faber for Alice (`FA(DID/DIDDoc)`)
- Alice for the mediator (`AM(DID/DIDDoc)`)
- The mediator for Alice (`MA(DID/DIDDoc)`)

DIDDocs would contain `serviceEndPoint` property with following info:

```json
{
  "service": [{
    "id": "did:example:123456789abcdefghi#did-communication",
    "type": "did-communication",
    "priority" : 0,
    "recipientKeys" : [ "did:example:123456789abcdefghi#1" ], // keys for the ultimate target(s) of the message
    "routingKeys" : [ "did:key:98490275222" ], // keys for the mediators
    "serviceEndpoint": "https://agent.example.com/"
  }]
}
```

In the above example, there is one recipient of the message and one mediator.


**`AF(DID/DIDDoc)`**

```json
{
  "service": [{
    "id": "did:peer:alice-for-faber#did-communication",
    "type": "did-communication",
    "priority" : 0,
    "recipientKeys" : [ "did:peer:alice-for-faber#keyagreement" ], //
    "routingKeys" : [ "did:key:mediator123" ],
    "serviceEndpoint": "https://agents-r-us.ca/mediator" //  endpoint (often an HTTP URL) for the mediator. Could be empty if routingKeys (below) contains a public DID for the mediator. In that case, the public DID would contain the endpoint.
  }]
}
```

**`FA(DID/DIDDoc)`**

```json
{
  "service": [{
    "id": "did:peer:faber-for-alice#did-communication",
    "type": "did-communication",
    "priority" : 0,
    "recipientKeys" : [ "did:peer:faber-for-alice#keyagreement" ], // faber's key specific for alice connection
    "routingKeys" : [ ], //empty, as messages go directly to Faber’s agent.
    "serviceEndpoint": "https://faber-agent-url/"
  }]
}
```

**`AM(DID/DIDDoc)`**

```json
{
  "service": [{
    "id": "did:peer:alice-for-mediator#did-communication",
    "type": "did-communication",
    "priority" : 0,
    "recipientKeys" : [ "did:peer:alice-for-mediator#keyagreement" ], // key referebce of Alice's mobile wallet specifically for mediator
    "routingKeys" : [ ],  // empty as there is no routing between alice and mediator
    "serviceEndpoint": null, // null because Alice's mobile wallet has no endpoint
  }]
}
```

**`MA(DID/DIDDoc)`**

```json
{
  "service": [{
    "id": "did:peer:mediator-for-alice#did-communication",
    "type": "did-communication",
    "priority" : 0,
    "recipientKeys" : [ "did:peer:mediator-for-alice#keyagreement" ], // key referebce of mediator  specifically for alcice
    "routingKeys" : [],  // empty as there is no routing between mediator and alice
    "serviceEndpoint": "https://agents-r-us/mediator123", // mediator endpoint
  }]
}
```


> **How does the wallet get its messages from the mediator?**

- For mobile wallets, the most common is to open a ***web socket*** with the mediator
- Alternatively, a DIDComm protocol called **Transport Return Route** [(RFC 0092)](https://github.com/hyperledger/aries-rfcs/tree/main/features/0092-transport-return-route) defines how a mediator can put DIDComm messages for its mediator clients


### Preparing Alice’s DIDDoc for Faber

- Initialization: Alice and the mediator
- Connecting to Faber
- Completing the connection


![img](/assets/alice-faber-msg-sequence.png)

[sequence diagram](https://sequencediagram.org/index.html#initialData=C4S2BsFMAJoWUgZ0QQwOYwGLgPYHdoAjSYPSSAO2hXBAGMYUKATaAMxWICdpgALLjgCuaPtAC2kZiBTAcXAFAKUdOTxr1IAfTw0owBQAcUXUHRDGKwCVJlqjJsxabWO3LekoGFaQUMPQAJIUYDK0AF6yIDgUyrQMOnokALQAfJLSsvIAXNAARBl2kMlckACOQkjAeQoUOMAwOABukDyFWVy5AOKUrbJI1NAAIoFD7PLU8TAA2nAAglrSzAC6Cu1qaRoJuuD6uQW2-cm+LnnQAN7ziyDMAL4KlMxKvsIBAMIxFJCqIBRovDhoJhOK0FG5Wh4MFZkmlUlttDs9gBlOhMRDQACKACVoHQcMwZj1gOi8RQvj8YtBfk1QpS8GAxMDuNAAKIsQw4X7AVZKOoNaDNVqTTSJXYkbq9Lj9dEoYajcY8cE8aZzTDXFa8+qNFrqKaivYAKgNqvVQxwdAAdIhWjSGEbsgAdCjnUrmQwgLwAaUgAE9ELkVWqlssADROvygP7ev0Bq7BsMUa1cW2QNnMDlc-Z8YDAQz+gD0+fW8mSniseXucRFiJSsOLnXyAGtfbREMBkv5mP0zucTUt7vD9SlUkrIV5cr2gzd7o9nn53jhxIZ9L9-vwYB8yd9QDFalqBTr2CCuGOrLkAAqlYyldHrmzITwK6BzKYAcnRNeAFr3-MFiuPp7ALkbJ0FwPqGMS1AsNAy4qAM-CyPeqAYE6VJWICsp5Gw8i6FwzBnJID4YE+L6aO+NiZGo35ggBZbtukhxqLkSKPLefCIYRyEwHIvB8DA9bQI8GZWEo9YwqkcJ6p+uScZ4SiPEAA)




