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

const token = "ba846af4c747704a9c21f87a8924af5c1996e92bfd33cd30e0e1be5288078bb0";
const contractHash = "hash-ccf8fc64fcacb6c02f8887ba23c9ffa631da625b628f4755048242f941aa9dae";

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

const approve = async () => {
    contract.setContractHash("hash-" + token);

    const args = RuntimeArgs.fromMap({
        // duration: CLValueBuilder.u64(604800),
        spender: CasperHelpers.stringToKey("14407677ef0f1a65f14a3782605961dfe08e3cff2848b050fdaa1575bedee255"),
        amount: CLValueBuilder.u256(100 * Math.pow(10, 8)),
    });

    const deploy = contract.callEntrypoint("approve", args, keys.publicKey, "casper-test", "10000000000", [keys]);

    try {
        const tx = await client.putDeploy(deploy);

        console.log("https://testnet.cspr.live/deploy/" + tx);
    } catch (error) {
        console.log("error", error);
        return error;
    }
};

const allowance = async () => {
    contract.setContractHash("hash-" + token);

    // const args = RuntimeArgs.fromMap({
    //     owner: CLValueBuilder.key(keys.publicKey),
    //     spender: CasperHelpers.stringToKey("da9c4ac649cf16bee27a09734d3a4a34ab530743aa307e2fe2d706cbb991a032"),
    // });

    const args = RuntimeArgs.fromMap({
        // contractin package hashi
        spender: CasperHelpers.stringToKey("40c39b8fe67a33891d2d6bb4aa5150f82dd0bb5492265849ba7c2575377feedf"),
        amount: CLValueBuilder.u256(100 * Math.pow(10, 8)),
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

const stake = async () => {
    contract.setContractHash(contractHash);

    const args = RuntimeArgs.fromMap({
        amount: CLValueBuilder.u256(100 * Math.pow(10, 8)),
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

// install();

// notify_reward_amount();

// approve();
// allowance();

stake();
