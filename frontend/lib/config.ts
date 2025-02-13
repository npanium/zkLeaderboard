export const RISC0_SERVICE_URL = "http://localhost:8080";
export const ZKVERIFY_SEED_PHRASE = process.env.ZKVERIFY_SEED_PHRASE || "";

export const BACKEND_PORT = 3001;
export const WALLET_ADDRESS = "0x67f1452b3099CfB27E708130421c98aD2319C0b7";
export const ARBISCAN_TXN = "https://sepolia.arbiscan.io/tx/";
export const API_BASE = `http://localhost:${BACKEND_PORT}/api/v0`;

export const API_ENDPOINTS = {
  addresses: "/addresses",
  hashStore: "/addresses/hash/store",
  windowStart: "/addresses/window/start",
  windowStatus: "/addresses/window/status",
  windowClose: "/addresses/window/close",
  mintTokens: "/token/mint-to",
  tokenBalance: (address: string) => `/token/balance/${address}`,
  bets: "/addresses/bets",
  betsCount: "/addresses/bets/count",
  bettingAmounts: (index: number) => `/addresses/amounts/${index}`,
  verify: "/verify",
} as const;
