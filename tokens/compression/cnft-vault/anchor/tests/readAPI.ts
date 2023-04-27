// I recommend using a WrappedConnection for production
// as it supports more readAPI functionality
// this is just a subset of functions for quick availabiity

import axios from "axios";

// TODO change to your API key (this is mine on free-tier)
const HELIUS_RPC = "https://rpc-devnet.helius.xyz/?api-key=30536abf-e8e7-444f-a255-18e9a0c27e8b";

export async function getAsset(assetId: any, rpcUrl = HELIUS_RPC): Promise<any> {
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


export async function getAssetProof(assetId: any, rpcUrl = HELIUS_RPC): Promise<any> {
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