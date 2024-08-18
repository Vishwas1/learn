Recap

Hyperledger Aries is a set of messaging protocols and implementations of those protocols for enabling secure peer-to-peer messaging that can be used for creating, transmitting, storing, presenting and verifying digital credentials

Aries agents are software components that act on behalf of entities—people, organizations and things. 

Aries agents enable decentralized, self-sovereign identity based on a secure, peer-to-peer communications channel. 

agent variations: 

- Agents for people 
- Agents for organizations
- Agents for routing messages to and from agents for people and organizations. 

Aries agent components: 
- a framework and 
- a controller

The framework contains the standard capabilities that enable an Aries agent to interact with its surroundings—ledgers, storage, verifiable credentials, presentations and other agents.  An Aries framework knows how to initiate connections, respond to requests, send messages, manage secure storage and more.  However, a framework needs to be told when to initiate a connection or to send a request. It doesn’t know what response should be sent to a given request. A deployed framework just sits there until it’s told what to do.

The controller is the component that, well, controls the behavior of an instance of an Aries framework. For example: 
- In a mobile application, the controller is the user interface that enables a person to interact in an Aries ecosystem.
- An issuer, such as Faber College’s agent, has a controller that integrates agent capabilities (requesting proofs, verifying them, issuing credentials and so on) with enterprise systems, such as a "Student Information System" that tracks students and their grades.  


>  The controller executes the business logic that defines how a particular agent instance behaves—how it responds to the events it receives, and when to initiate events. 


> The controller might be a web or native user interface for a person or it might be coded business rules driven by an enterprise system. 

## Aries Cloud Agent (ACA-py) Architecture

![aca](/assets/aca-py.png)

- When the framework receives a message (an event) from the outside world, it sends a webhook (a notification) about the event to the controller so the controller can decide what to do.
In turn, the controller sends a request to the framework to tell the framework how to respond to the event.
- The same controller-to-framework request interface is used when the controller wants the framework to initiate an action, such as sending a message to another agent.



https://github.com/hyperledger/aries-cloudagent-python?tab=readme-ov-file#understanding-the-architecture

![](https://github.com/hyperledger/aries-cloudagent-python/blob/main/aca-py_architecture.png)


[Aries Cloud Agent - Python Documentation](https://aries-cloud-agent-python.readthedocs.io/en/latest/)


[Overall architecture explanation](https://docs.google.com/presentation/d/1K7qiQkVi4n-lpJ3nUZY27OniUEM0c8HAIk4imCWCx5Q/edit#slide=id.g5d43fe05cc_0_87)

## Establish connections between Agents

![](/assets/agent-agent-conn.png)

[sequence_diagram](https://sequencediagram.org/index.html#initialData=C4S2BsFMAIEEHNIDtgGdoGED2SmQMag6oBQZAhoVgE5zgj6QLLAkAO51o+IHK0AMXIAjSNWYoyLagE9UHHknjQADADoA7CSRZgMLADcxgkWInAAXIJBIAJulj1G56AAoA-AEoSQ0eMQoALSBAHyODEwBltCoyPbQNgZgMMAAFtRYAK7wqdAAigBKZDp60IbG4c5RVhjUkOR66OTQeADu0GzUIAYNMAAiAJJ9JJWRLNChJn7mVgBEdQCOmZCowNDAWND4OHiEs9AA3uROkAD6nSDkPaWDfQC+xbr6RrS+ZtUT0ADKG3UOJ+htrgCEQkAkkAAzLAAHSQgS2dV6TRakHaF2u-SGPlM-nGk1GMxicWRfzYxBgAFsVqhyIhoI9SuVXjjzAAaAkfaAAdVSWCwvKwAEJoAAVVIwWksJp1LY7EGQWxqMgc8YAHkmb1xKAsmug5DsdAiW31LSw7ViBqpqBpiHQGzKeD1JXFtCAA)

## Labs

Setup and intall 

https://github.com/hyperledger/aries-cloudagent-python/blob/main/docs/features/DevReadMe.md


[A Hyperledger Aries/AnonCreds Workshop Using Traction Sandbox](https://github.com/hyperledger/aries-cloudagent-python/blob/main/docs/demo/Aries-Workshop.md)


## Current Aries Agent Frameworks

- [Aries Cloud Agent (ACA) - Python](https://github.com/hyperledger/aries-cloudagent-python)
- [Aries Framework Go (AF-Go)](https://github.com/hyperledger-archives/aries-framework-go): `Archieved`. supports JSON-LD verifiable credentials using LD-Signatures and BBS+ Signatures, on VDRs other than Indy.  AF-Go was implemented to support the TrustBloc and did:orb VDRs, but also includes an extensible DID resolver that allows the use of many other DID methods. The benefit of being a pure Golang implementation is that the library can be compiled to WASM for use within a browser, opening up the possibility of browser-based Aries wallets.
- [Aries Framework JavaScript](https://github.com/openwallet-foundation/credo-ts): The AFJ team’s initial focus was on building a framework to be embedded in a mobile wallet app, but features are rapidly being added to support the server-side use case as well.  support, mediator capabilities and support for both AnonCreds and JSON-LD (using LD-Signatures and BBS+ Signatures) verifiable credentials.
- [Aries VCX - Rust](https://github.com/hyperledger/aries-vcx) : aries-vcx is set of crates to work with DIDs, DID Documents, DIDComm, Verifiable Credentials and Hyperledger Aries.   Aries VCX is implemented in Rust with a C-callable interface, and official wrappers for Java (including Android), iOS, and NodeJS.


### Nodejs framework 

run Aries agent in nodejs
https://credo.js.org/guides/getting-started

The architecture for the controller and framework with Aries Framework JavaScript (AFJ) is a little different from ACA-Py. 

Instead of embedding the framework in a web server and exposing an HTTP interface for the controller, 
> AFJ is built as a library, and the library is embedded in an application—the controller—that you create.

![ajf.png](/assets/ajf.png)

### Aries frameworm Go

The Aries Framework Go (AFGo) takes the same approach to the controller/framework architecture as ACA-Py—an HTTP interface between the two.

A controller written for ACA-Py should work with an instance of AFGo with relatively few changes.

That also means the framework **does not use the Indy** and AnonCreds Rust language components, and so **does not support out-of-the-box writing Indy ledgers** or the AnonCreds verifiable credentials exchange model. 

AFGo supports using other types of ledgers, W3C Verifiable Credentials Data Model Standard signatures, **such as LD-Signatures and BBS+ Signatures for JSON-LD**.

https://github.com/hyperledger/aries-framework-go

----
> **How do we know that all of those Aries frameworks are following the Aries Interop Profiles  (AIPs)?**


## Aries Agent Test Harness (AATH)

> "we need a test suite that lets us execute tests using agents created from one or more Aries frameworks."

The tests must be defined to exercise the RFCs, and must make it easy to mix and match the agents being tested. Further, we don’t want to run that test suite just once—we want to run the tests continuously against the latest codebases so that as the frameworks evolve, they continue to be interoperable. 
 
https://github.com/hyperledger/aries-agent-test-harness

The test agents are driven by a "Behavior Driven Development"

[Lab](https://github.com/cloudcompass/ToIPLabs/blob/main/docs/LFS173xV2/AriesTestHarness.md)

By monitoring the [Aries Interoperability Test Results website](https://aries-interop.info/):

- you can track Current state of interoperability across a number of Aries frameworks
- You can see what specific tests are passing and failing
- You can drill into the test results and see exactly where the failures are occurring.
- You can see how new protocols and AIP versions are progressing across Aries implementations. This will tell you, for example, is AIP 2.0 ready to use in the wild?


### Aries Mobile Test Harness

https://github.com/openwallet-foundation/mobile-wallet-test-harness

The Aries Mobile Test Harness (AMTH) is a ATDD-based test execution engine and set of tests for evaluating Aries Mobile Wallet/Agent Apps on both Android and iOS. 

## References

- https://aca-py.org/latest/demo/ 
- https://github.com/cloudcompass/ToIPLabs/blob/main/docs/LFS173xV2/agentsConnecting.md
- [Aries RFC](https://github.com/hyperledger/aries-rfcs)
- [Aries Mobile Agent React Native ](https://github.com/openwallet-foundation/bifold-wallet)
- https://credo.js.org/
- [Aries Agent Backchannels](https://github.com/hyperledger/aries-agent-test-harness?tab=readme-ov-file#aries-agent-backchannels)
- [The Mobile Backchannel]() : backchannel that can be used to test mobile wallet apps