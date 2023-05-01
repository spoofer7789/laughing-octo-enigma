import * as IPFS from 'ipfs';
import OrbitDB from 'orbit-db';
import fetch from 'node-fetch';
async function sendRequest(url, method, body) {
  const response = await fetch(url, {
    method: method,
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
  return await response.json();
}
async function main () {
  const ipfsOptions = { repo: './ipfs',}
  const ipfs = await IPFS.create(ipfsOptions)
  const orbitdb = await OrbitDB.createInstance(ipfs)
  const db = await orbitdb.keyvalue('first-database')
  console.log(db.address.toString())
  // /orbitdb/Qmd8TmZrWASypEp4Er9tgWP4kCNQnW4ncSnvjvyHQ3EVSU/first-database
  const usersStore = await orbitdb.keyvalue('users');
  await usersStore.load();

  // Create a document store for storing Solidity contracts and data
  const contractsStore = await orbitdb.docstore('contracts');
  await contractsStore.load();

  // Usage examples
  const serverUrl = 'http://localhost:3000';
  await addUser(usersStore, serverUrl, 'username', 'password', 'publicKey');
  await addContract(contractsStore, 'SolidityContractCode', { metadata: 'Some metadata' });

  console.log(usersStore.all);
  console.log(contractsStore.query(doc => doc.type === 'contract'));
}
async function addUser(store, serverUrl, username, password, publicKey) {
  await store.set(username, { password, publicKey });
  const response = await sendRequest(serverUrl, '/add_user', 'POST', { username, password, publicKey });
  console.log(response);
}



async function addContract(store, contractCode, metadata) {
  await store.put({
    _id: new Date().toISOString(),
    type: 'contract',
    code: contractCode,
    metadata
  });
}
main();
