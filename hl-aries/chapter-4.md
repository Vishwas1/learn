"wallet" used to mean "secure storage."

"wallet" for a mobile application

Aries Agent configuration

- location of the genesis file(s) for ledger(s)
- If it needs objects (DIDs, schema, etc.) on the ledger, checking that they exist on ledger and in secure storage, and creating those objects if they don’t exist.
- Transport (such as HTTP or web sockets) endpoints for messaging other agents.
- Storage options for keys and other data.
- Interface details between the agent framework and the controller for events and requests.

At minimum, the agent must know about things such as 
- how to access a database for secure storage, 
- how to connect to ledgers, 
- the HTTP addresses for interacting with the controlle


>  As of this writing (January 2023), there are 112 ACA-Py startup options! So **getting a handle on startup options** is your first Aries developer task.


## Command line options
ACA-Py can be configured via command line options in the form of `--option <extra info>`

For example, to specify the name of the genesis file the agent is to use, the command line option is `--genesis-file <genesis-file>`

or, environment variable `ACAPY_GENESIS_FILE` can be set to `<genesis-file>` before running the command to have the same effect

or, A YAML configuration file can also be used to set any and all of the command line options


You can mix and match the configuration settings using all three of the techniques. If you do, the order of precedence is `command line options` > override `environment variables` > override `YAML file values` >  override `code defaults`. 


## 

- A basic ACA-Py agent is a stateful component that persists data in its secure storage and as needed, to a ledger.
-  When an agent starts up for the first time, it has no persistent storage and so it must create a secure storage database, and then any ledger objects it will need to fulfill its role. 
- ACA-Py provides two major modes of operation: "provision" and "start." 
    - **provision**: intended to be used one time per agent deployment to establish its secure storage database and the required ledger objects.
    - **start**: used for normal operations and assumes that everything is in place in secure storage and on ledger.

## Startup Option Groups

## Exploring Aries Protocol 

- Aries agents *communicate with each other* via a messaging mechanism called *DIDComm*
- DIDComm uses DIDs (usually private, pairwise DIDs) to enable secure, asynchronous, end-to-end encrypted messaging between agents, including the option of routing messages through a ***configuration of mediator*** agents. 
- Aries agents commonly use peer-to-peer DID methods that do not publish DIDs to a ledger, but instead share them privately between the communicating parties, usually just two agents.

**Protocol1: OUT OF BAND & DID EXCHANGE**
The [Out of Band - RFC 0434](https://github.com/hyperledger/aries-rfcs/blob/main/features/0434-outofband/README.md) and [DID Exchange - RFC 0023](https://github.com/hyperledger/aries-rfcs/blob/main/features/0023-did-exchange/README.md) protocols enable two agents to establish a connection (or reuse an existing connection) through a series of messages—an invitation, a connection request (or reuse connection), and a connection response.

**Protocol2: Issue Credential**
The [Issue Credential protocol - RFC 0453](https://github.com/hyperledger/aries-rfcs/blob/main/features/0453-issue-credential-v2/README.md) enables an agent to issue a credential to another agent.

**Protocol3: Present Proof Protocol 2.0**
The [Present Proof protocol - RFC 0454](https://github.com/hyperledger/aries-rfcs/blob/main/features/0454-present-proof-v2/README.md) enables an agent to request and receive a proof from another agent.


## Protocol Versions: AIP 1.0 and AIP 2.0

[**AIP 1.0**](https://github.com/hyperledger/aries-rfcs/tree/main/concepts/0302-aries-interop-profile#aries-interop-profile-version-10)

[**AIP 2.0**](https://github.com/hyperledger/aries-rfcs/tree/main/concepts/0302-aries-interop-profile#base-requirements)

https://hyperledger.github.io/aries-rfcs/latest/aip2/0003-protocols/


**AIP 3.0** - *comming soon!*

- The primary motivation (at least right now) for AIP 3.0 is to add support for DIDComm Messaging V2.
- DIDComm V2 handles the protocol-specific message body and attachments differently from DIDComm V1


https://github.com/hyperledger/aries-rfcs/tree/main/concepts/0302-aries-interop-profile


## Controller

![aries-protocol-controller](/assets//aries-protocol-controller.png)

[Aries Protocol Triggered By External Agent - Sequence diagram](https://sequencediagram.org/index.html#initialData=C4S2BsFMAIEECcSQM7QArwPbEwY0+NACqIDmpk8kAJtAEICe0AogB7CUB2AhobBZ2AAoId1w540AK7JKQgA7d4oXCEWDo+QVnBR4CpSrXcNY7iMXKQq9cGiR2XXgFpuA4SKGdsMTADdKTUxtAj0AGmgzAC5oAEYAOhYA+AZgAAsQTlJoNO5UACNISE5oTLAQXhAALxoAHU4AJkSAYWDOSHEQYNRcgOhC4vtkYG588BBkNJpPIQcOeB5wV3doZwA+SNxuGOaqamLQXmgAeQAzU8CAJUgAR2HoAG0AEQBJJ9aAWw-oD5RkN0gAF0RPlMKxoP5AtE4okXvtBCBTkwtJx2p1gvUmtAXpxytwONB2gB3UqcYYmXC+U7QABEL2QyCkMF2NAOFXANPq0G50HkWBw+HAAG5oJcAGLNaAABgALABWADMIjMqzWawAFLEAJRBEK6Sg7PZso5nC6SB4AdUg+TSmEwAGt7AFBMDRKDwWQ0nZMNSVQBicakL2kKgMGKAZ+JoABxTAoCElInccpZaCnTCSbBTSQfZCkVAiLTAHR6VUahU6mQGwmYN1g6Ce73UwvFwIBkBB4AhyBh6CRxPJ7JpySVyQ4esoeTBahCEerdZlrXN0JV2C4SnyOwAelFkAAVh0PEJdUXl5J55qddCABJEIhoHc3Jn3dWb3BGhG8AD6VEfKGAWs8FVnHWexHAWFwAUEGI31ZD9CB-J9hCAA)

Demo Controller:  https://github.com/hyperledger/aries-acapy-controllers/tree/main/AliceFaberAcmeDemo

[Aries ACA-py Controllers](https://github.com/hyperledger/aries-acapy-controllers)




## Lab -- Alice Gets a Credential

https://github.com/cloudcompass/ToIPLabs/blob/main/docs/LFS173xV2/AliceGetsCredential.md

## Lab -- ACA-py Controller Admin API

https://github.com/cloudcompass/ToIPLabs/blob/main/docs/LFS173xV2/OpenAPIIntroduction.md

## Lab -- Help Alice get a job


https://github.com/cloudcompass/ToIPLabs/blob/main/docs/LFS173xV2/HelpAliceGetAJob.md


## Aries Plugins 

ACA-Py plugins enable standardized extensibility without overloading the core ACA-Py code base. Plugins may be features that you create specific to your deployment, or that you deploy from the [ACA-Py Plugins "Store"](https://plugins.aca-py.org/latest/). 

https://github.com/hyperledger/aries-cloudagent-python/blob/main/docs/features/PlugIns.md


#

