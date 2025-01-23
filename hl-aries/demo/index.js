


// const faberBaseUrl = 'http://34.93.160.82:8021'
const faberBaseUrl = "https://68b7-34-93-160-82.ngrok-free.app"
const url = `${faberBaseUrl}/connections/create-invitation`

// const url = `${faberBaseUrl}/connections/create-static`

async function callApi() {
    console.log(url)

    const body = {
        "alias": "faber.agent",
        "their_seed": "JF4ONHIMvoiMf/imJrh8oMTPIRJygqTb"
    }
    // http://34.93.160.82:8021/connections/create-invitation
    // http://34.93.160.82:8021/connections/create-invitation
    const resp = await fetch(url, {
        method: 'POST',
        body: JSON.stringify({}),
        headers: {
            'accept': 'application/json',
            'Content-Type': 'application/json'
        }
    });
    const json = await resp.json()

    return json.invitation
}


async function test() {
    const t = await callApi();
    t.serviceEndpoint = faberBaseUrl
    console.log(t)
    const base64 = Buffer.from(JSON.stringify(t)).toString('base64')
    // console.log(base64)


    const finalUrl = `${faberBaseUrl}?c_i=${base64}`

    console.log(finalUrl)

    // console.log(Buffer.from(base64, 'base64').toString('ascii'))
}

test()

// 5de0865b-add3-4af7-84af-3210d51e238d - walletid
// a461db10-f9ab-4ccc-a1e9-01769964d9ec - walletkey