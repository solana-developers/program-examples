// I recommend using a WrappedConnection for production
// as it supports more readAPI functionality
// this is just a subset of functions for quick availabiity

import axios from "axios";

// const RPC_PATH = "https://rpc-devnet.aws.metaplex.com/";
const RPC_PATH = "hhttps://api.devnet.solana.com";

export async function getAsset(assetId: any, rpcUrl = RPC_PATH): Promise<any> {
  try {
    const axiosInstance = axios.create({
      baseURL: rpcUrl,
    });
    const response = await axiosInstance.post(rpcUrl, {
      jsonrpc: "2.0",
      method: "getAsset",
      id: "rpd-op-123",
      params: {
        id: assetId
      },
    });
    return response.data.result;
  } catch (error) {
    console.error(error);
  }
}


export async function getAssetProof(assetId: any, rpcUrl = RPC_PATH): Promise<any> {
  try {

    const axiosInstance = axios.create({
      baseURL: rpcUrl,
    });
    const response = await axiosInstance.post(rpcUrl, {
      jsonrpc: "2.0",
      method: "getAssetProof",
      id: "rpd-op-123",
      params: {
        id: assetId
      },
    });
    return response.data.result;
  } catch (error) {
    console.error(error);
  }
}