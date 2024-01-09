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

const stakeWasm = new Uint8Array(fs.readFileSync("/Users/afarukcali/Desktop/casper/casper-stake-contract/target/wasm32-unknown-unknown/release/stake.wasm"));
// const stakeWasm = new Uint8Array(fs.readFileSync(CONTRACT_WASM_PATH));

const keys = Keys.Secp256K1.loadKeyPairFromPrivateFile("secret.pem");

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

const token = "a8b1c5e58ffed7a289e8780081dd7d1a9037747b56177f8714c664f387436071";
const contractHash = "hash-18eaa53f8751702055caa99cb931d60d3b67a5f8e2c545c3f64714a1b3205b77";

async function install() {
  const args = RuntimeArgs.fromMap({
    staked_token: CasperHelpers.stringToKey(token),
    finish_at: CLValueBuilder.u64(1804810043215),
    duration: CLValueBuilder.u64(3600),
    reward_rate: CLValueBuilder.u8(10),
  });

  const deploy = contract.install(stakeWasm, args, "60000000000", keys.publicKey, "casper-test", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
}

const set_rewards_duration = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    duration: CLValueBuilder.u64(604800),
  });

  const deploy = contract.callEntrypoint("set_rewards_duration", args, keys.publicKey, "casper-test", "1000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const notify_reward_amount = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    // duration: CLValueBuilder.u64(604800),
    reward: CLValueBuilder.u256(1 * 1_000_000_000),
  });

  const deploy = contract.callEntrypoint("notify_reward_amount", args, keys.publicKey, "casper-test", "1000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

const stake = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    amount: CLValueBuilder.u256(2 * 1_000_000_000),
    staked_token: CasperHelpers.stringToKey(token),
  });

  const deploy = contract.callEntrypoint("stake", args, keys.publicKey, "casper-test", "1000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

install();
// set_rewards_duration();
// notify_reward_amount();
// stake();
