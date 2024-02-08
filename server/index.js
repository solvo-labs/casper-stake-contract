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
// const stakeWasm = new Uint8Array(fs.readFileSync(CONTRACT_WASM_PATH));

const keys = Keys.Secp256K1.loadKeyPairFromPrivateFile("secret.pem");
console.log(keys.publicKey.toHex());
// const user = Keys.Secp256K1.loadKeyPairFromPrivateFile("user.pem");

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

const token = "effbcbdc5bf6f974a13e72ff45461337a41c58298cdaa3cd5b890a4561f11d96";
const contractHash = "hash-951f97edb655650d2d5d262eaee9da5cfc99a8c521948ef4014d0dd52243fd75";
const contractPackageHash = "635420b33c2636750e2523afce8ddc468b74fb4bc953853a500408e06485a8bd";

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

  const deploy = contract.install(stakeWasm, args, "260000000000", keys.publicKey, "casper-test", [keys]);

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

// user.pem
const stake = async () => {
  contract.setContractHash(contractHash);

  const args = RuntimeArgs.fromMap({
    amount: CLValueBuilder.u256(10 * Math.pow(10, 8)),
  });

  const deploy = contract.callEntrypoint("stake", args, user.publicKey, "casper-test", "2000000000", [user]);

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
    amount: CLValueBuilder.u256(9 * Math.pow(10, 8)),
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

// install();

// notify_reward_amount();
// increase_allowance();

notify();

// stake();
// claim_reward();

// unstake();

// transfer();
