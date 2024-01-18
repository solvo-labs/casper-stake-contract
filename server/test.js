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
const contractHash = "hash-f346f6b197bcf70ed36751e6aa755e953cfb1e7979d423ad8f6253e7959069bc";

async function install() {
    const args = RuntimeArgs.fromMap({
        token: CasperHelpers.stringToKey(token),
        fixed_apr: CLValueBuilder.u64(5),
        min_apr: CLValueBuilder.u64(1),
        max_apr: CLValueBuilder.u64(5),
        max_capacity: CLValueBuilder.u64(1000),
        min_stake: CLValueBuilder.u64(10),
        max_stake: CLValueBuilder.u64(100),
        lock_period: CLValueBuilder.u64(604080),
        deposit_start_time: CLValueBuilder.u64(1705084529000),
        deposit_end_time: CLValueBuilder.u64(1705775729000),
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

const increase_allowance = async () => {
    contract.setContractHash("hash-" + token);

    const args = RuntimeArgs.fromMap({
        // Spender: Contract Package Hash
        spender: CasperHelpers.stringToKey("533b513a3f02ca81b0fa019e2c164d48affc8a7063ffe0df059a5bcdd641c874"),
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


const decrease_allowance = async () => {
    contract.setContractHash("hash-" + token);

    const args = RuntimeArgs.fromMap({
        // Spender: Contract Package Hash
        spender: CasperHelpers.stringToKey("533b513a3f02ca81b0fa019e2c164d48affc8a7063ffe0df059a5bcdd641c874"),
        amount: CLValueBuilder.u256(10 * Math.pow(10, 8)),
    });

    const deploy = contract.callEntrypoint("decrease_allowance", args, user.publicKey, "casper-test", "10000000000", [user]);

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
        amount: CLValueBuilder.u256(10 * Math.pow(10, 8)),
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


// install();

// increase_allowance();

// stake();

// decrease_allowance();

unstake();

