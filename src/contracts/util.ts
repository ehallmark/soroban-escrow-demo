export const rpcUrl = import.meta.env.PUBLIC_STELLAR_RPC_URL ?? "http://localhost:8000/rpc"
export const networkPassphrase = import.meta.env.PUBLIC_STELLAR_NETWORK_PASSPHRASE ?? "Standalone Network ; February 2017"
export const explorerUrl = import.meta.env.PUBLIC_STELLAR_EXPLORER_URL ?? "https://stellar.expert/explorer/testnet"

export const TOKENS = {
    "Test SDF Network ; September 2015": {
        "XLM": {
            "contract": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
        },
        "USDC": {
            "contract": "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA"
        }
    }
};

export const TOKEN_SYMBOLS = {
    "Test SDF Network ; September 2015": {
        "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC": "XLM",
        "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA": "USDC",
    }
}

export const SYMBOL_TOKENS = {
    "Test SDF Network ; September 2015": {
        "XLM": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
        "USDC": "CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA",
    }
}

export function urlForAccountAddress(address: string) {
    return `${explorerUrl}/account/${address}`
}