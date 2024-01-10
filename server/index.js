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

const stakeWasm = new Uint8Array(fs.readFileSync("/home/oguz/Project/casper-stake-contract/target/wasm32-unknown-unknown/release/stake.wasm"));
// const stakeWasm = new Uint8Array(fs.readFileSync(CONTRACT_WASM_PATH));

const keys = Keys.Ed25519.loadKeyPairFromPrivateFile("secret.pem");

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

const token = "e65c886eaef177d2517e608f79073b963633236582685188900a2d6cd6432254";
const contractHash = "hash-68be7ad23c2b561ed91a4057dfb03963238fc320349ed4a5851d7db76b6b64df";

async function install() {
  const args = RuntimeArgs.fromMap({
    staked_token: CasperHelpers.stringToKey(token),
    duration: CLValueBuilder.u64(604000 * 100),
    // finish_at: CLValueBuilder.u64(1804810043215),
    // reward_rate: CLValueBuilder.u8(10),
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

// const set_rewards_duration = async () => {
//   contract.setContractHash(contractHash);

//   const args = RuntimeArgs.fromMap({
//     duration: CLValueBuilder.u64(604800),
//   });

//   const deploy = contract.callEntrypoint("set_rewards_duration", args, keys.publicKey, "casper-test", "1000000000", [keys]);

//   try {
//     const tx = await client.putDeploy(deploy);

//     console.log("https://testnet.cspr.live/deploy/" + tx);
//   } catch (error) {
//     console.log("error", error);
//     return error;
//   }
// };

const notify_reward_amount = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    // duration: CLValueBuilder.u64(604800),
    amount: CLValueBuilder.u256(100 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("notify_reward_amount", args, keys.publicKey, "casper-test", "10000000000", [keys]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

// const stake = async () => {
//   contract.setContractHash(contractHash);

//   const args = RuntimeArgs.fromMap({
//     amount: CLValueBuilder.u256(2 * 1_000_000_000),
//     staked_token: CasperHelpers.stringToKey(token),
//   });

//   const deploy = contract.callEntrypoint("stake", args, keys.publicKey, "casper-test", "1000000000", [keys]);

//   try {
//     const tx = await client.putDeploy(deploy);

//     console.log("https://testnet.cspr.live/deploy/" + tx);
//   } catch (error) {
//     console.log("error", error);
//     return error;
//   }
// };

// install();
// set_rewards_duration();
notify_reward_amount();
// stake();
