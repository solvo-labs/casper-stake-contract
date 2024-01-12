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
const user = Keys.Ed25519.loadKeyPairFromPrivateFile("user.pem");

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

const token = "35cbaceb6e33b052f0df1f12b60a49d3fa6c215f4a3cec19ca9f559a38bca99b";
const contractHash = "hash-aebce8084c116aea32ae69c446ca25acaf2054072978ab437f5993c319391583";

async function install() {
    const args = RuntimeArgs.fromMap({
        staked_token: CasperHelpers.stringToKey(token),
        duration: CLValueBuilder.u64(604000 * 100),
        // finish_at: CLValueBuilder.u64(1804810043215),
        // reward_rate: CLValueBuilder.u8(10),
    });

    const deploy = contract.install(stakeWasm, args, "80000000000", keys.publicKey, "casper-test", [keys]);

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
        spender: CasperHelpers.stringToKey("239fbb241fa2d12e39442290a1f8b9c1082781cdb5649ef2cc19ecb9cabd12e4"),
        amount: CLValueBuilder.u256(10 * Math.pow(10, 8)),
    });

    const deploy = contract.callEntrypoint("increase_allowance", args, user.publicKey, "casper-test", "10000000000", [user]);

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

    const deploy = contract.callEntrypoint("stake", args, user.publicKey, "casper-test", "1000000000", [user]);

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

stake();
