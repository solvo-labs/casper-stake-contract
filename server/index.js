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
const user = Keys.Secp256K1.loadKeyPairFromPrivateFile("user.pem");

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
const contractHash = "hash-cb02e99946eeaaf6996a84500e8957426a70c670e7747b84403e7a9bff9f79ab";

async function install() {
  const args = RuntimeArgs.fromMap({
    token: CasperHelpers.stringToKey(token),
    max_cap: CLValueBuilder.u256(20 * Math.pow(10, 8)),
    min_stake: CLValueBuilder.u256(1 * Math.pow(10, 8)),
    max_stake: CLValueBuilder.u256(10 * Math.pow(10, 8)),

    // fixed case
    fixed_apr: CLValueBuilder.u64(0),
    min_apr: CLValueBuilder.u64(15),
    max_apr: CLValueBuilder.u64(20),

    lock_period: CLValueBuilder.u64(1200000),
    deposit_start_time: CLValueBuilder.u64(Date.now()),
    deposit_end_time: CLValueBuilder.u64(Date.now() + 600000),
  });

  const deploy = contract.install(stakeWasm, args, "90000000000", keys.publicKey, "casper-test", [keys]);

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

// const approve = async () => {
//     contract.setContractHash("hash-" + token);
//
//     const args = RuntimeArgs.fromMap({
//         // duration: CLValueBuilder.u64(604800),
//         spender: CasperHelpers.stringToKey("14407677ef0f1a65f14a3782605961dfe08e3cff2848b050fdaa1575bedee255"),
//         amount: CLValueBuilder.u256(100 * Math.pow(10, 8)),
//     });
//
//     const deploy = contract.callEntrypoint("approve", args, keys.publicKey, "casper-test", "10000000000", [keys]);
//
//     try {
//         const tx = await client.putDeploy(deploy);
//
//         console.log("https://testnet.cspr.live/deploy/" + tx);
//     } catch (error) {
//         console.log("error", error);
//         return error;
//     }
// };

// user.pem
const increase_allowance = async () => {
  contract.setContractHash("hash-" + token);

  const args = RuntimeArgs.fromMap({
    // Spender: Contract Package Hash
    spender: CasperHelpers.stringToKey("b1b83010b59ef2b03923406ac7483a9a5b84571885c1c6cb2821b5ce9aca619e"),
    amount: CLValueBuilder.u256(10 * Math.pow(10, 8)),
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
    amount: CLValueBuilder.u256(10 * Math.pow(10, 8)),
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

const unstake = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    amount: CLValueBuilder.u256(5 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("unstake", args, user.publicKey, "casper-test", "1000000000", [user]);

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

  const deploy = contract.callEntrypoint("claim_reward", args, user.publicKey, "casper-test", "1000000000", [user]);

  try {
    const tx = await client.putDeploy(deploy);

    console.log("https://testnet.cspr.live/deploy/" + tx);
  } catch (error) {
    console.log("error", error);
    return error;
  }
};

install();

// notify_reward_amount();

// increase_allowance();

// stake();
// claim_reward();

// unstake();
