export interface ClusterWithUrl {
    url: string;
    id: string;
    label: string;
}

export interface TokenAccountParsedInfo {
    mint: string;
    owner: string;
    delegate?: string;
    tokenAmount: {
        amount: string;
        decimals: number;
        uiAmount: number;
        uiAmountString: string;
    };
}

export interface TokenAccountData {
    parsed: {
        info: TokenAccountParsedInfo;
        type: string;
    };
    program: string;
    space: number;
}

export interface TokenAccountEntry {
    account: {
        data: TokenAccountData;
        executable: boolean;
        lamports: number;
        owner: string;
        rentEpoch: number;
    };
    pubkey: string;
}
