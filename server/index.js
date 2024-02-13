const fs = require("fs");
const {
  RuntimeArgs,
  CLValueBuilder,
  Contracts,
  CasperClient,
  Keys,
  CLPublicKey,
  CLURef,
  Signer,
  CLKey,
  CasperServiceByJsonRPC,
  CLAccountHash,
  CLByteArray,
  AccessRights,
} = require("casper-js-sdk");

const client = new CasperClient("https://rpc.testnet.casperlabs.io/rpc");

const stakeWasm = new Uint8Array(fs.readFileSync("stake.wasm"));

const keys = Keys.Ed25519.loadKeyPairFromPrivateFile("secret.pem");
// const user = Keys.Secp256K1.loadKeyPairFromPrivateFile("secret.pem");

const contract = new Contracts.Contract(client);

class CasperHelpers {
  static stringToKey(string) {
    return CLValueBuilder.key(this.stringToKeyParameter(string));
  }

  static stringToKeyParameter(string) {
    return CLValueBuilder.byteArray(this.convertHashStrToHashBuff(string));
  }

  static convertHashStrToHashBuff(hashStr) {
    let hashHex = hashStr;
    if (hashStr.startsWith("hash-")) {
      hashHex = hashStr.slice(5);
    }
    return Buffer.from(hashHex, "hex");
  }
}

//token contract hash
const token = "1270fd8b9a96a1509337f4f9909603da820e7e637b30adc77e52c51f84207acf";
const contractHash = "hash-783146e098614a54f14c6fbda9f7273dae43bbbcae775a4958463a46af1c3843";
const contractPackageHash = "a72010c60efcaf2fa9491e15c69cf2eb3c51afae5ea736d8ace1f18c596e5a43";

async function install() {
  const args = RuntimeArgs.fromMap({
    token: CasperHelpers.stringToKey(token),
    min_stake: CLValueBuilder.u256(1 * Math.pow(10, 8)),
    max_stake: CLValueBuilder.u256(50 * Math.pow(10, 8)),
    max_cap: CLValueBuilder.u256(100 * Math.pow(10, 8)),

    // fixed case
    fixed_apr: CLValueBuilder.u64(0),
    min_apr: CLValueBuilder.u64(15),
    max_apr: CLValueBuilder.u64(20),

    lock_period: CLValueBuilder.u64(5000),
    deposit_start_time: CLValueBuilder.u64(Date.now()),
    deposit_end_time: CLValueBuilder.u64(Date.now() + 300000),
    storage_key: new CLAccountHash(Buffer.from("46e7c209fcbce9c5447d162c212b5f63d68f23ba90cba432d50c204a8634f758", "hex")),
  });

  const deploy = contract.install(stakeWasm, args, "150000000000", keys.publicKey, "casper-test", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
}

// user.pem
const increase_allowance = async () => {
  contract.setContractHash("hash-" + token);

  const args = RuntimeArgs.fromMap({
    // Spender: Contract Package Hash
    spender: CasperHelpers.stringToKey(contractPackageHash),
    amount: CLValueBuilder.u256(20 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("increase_allowance", args, keys.publicKey, "casper-test", "10000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const notify = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({});

  const deploy = contract.callEntrypoint("notify", args, keys.publicKey, "casper-test", "10000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const transfer = async () => {
  contract.setContractHash("hash-" + token);

  const args = RuntimeArgs.fromMap({
    // Spender: Contract Package Hash
    recipient: CasperHelpers.stringToKey(contractPackageHash),
    amount: CLValueBuilder.u256(100 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("transfer", args, keys.publicKey, "casper-test", "10000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const increase_allowance_stake = async () => {
  contract.setContractHash("hash-" + token);

  const args = RuntimeArgs.fromMap({
    // Spender: Contract Package Hash
    spender: CasperHelpers.stringToKey(contractPackageHash),
    amount: CLValueBuilder.u256(20 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("increase_allowance", args, keys.publicKey, "casper-test", "10000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

// user.pem
const stake = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    amount: CLValueBuilder.u256(20 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("stake", args, keys.publicKey, "casper-test", "3000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const unstake = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({});

  const deploy = contract.callEntrypoint("unstake", args, keys.publicKey, "casper-test", "2000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const claim_reward = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({});

  const deploy = contract.callEntrypoint("claim", args, keys.publicKey, "casper-test", "2000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const refund_reward = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({});

  const deploy = contract.callEntrypoint("refund_reward", args, keys.publicKey, "casper-test", "2000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

// install();

// increase_allowance();

// notify();

// increase_allowance_stake();

// stake();

// claim_reward();

// unstake();

refund_reward();
