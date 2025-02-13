import { useState } from "react";
import { GameData } from "../lib/types";
import { useAccount } from "wagmi";

const BACKEND_PORT = 3001;
// const WALLETADDRESS = "0x67f1452b3099CfB27E708130421c98aD2319C0b7";
const ARBISCAN_TXN = "https://sepolia.arbiscan.io/tx/";
const API_BASE = `http://localhost:${BACKEND_PORT}/api/v0`;

const GET_ADDRESSES = `http://localhost:${BACKEND_PORT}/api/v0/addresses`;
/*
Response:
[
    {
        "id": null,
        "address": "0x1234", // Only show this to the FE
        "score": 189
    },
      {
        "id": null,
        "address": "0x1234",
        "score": 189
    },
    ...
]
*/
const GET_HASH_STORE = `http://localhost:${BACKEND_PORT}/api/v0/addresses/hash/store`;
/*
 Response:
{
  "hash": "0xabcd...1234",
  "timestamp": 1676184000,
  "record_count": 1000,
  "transaction_hash": "0xabcd...1234"
}
 */

// const POST_INIT_ADDR_CONTRACT = `http://localhost:${BACKEND_PORT}/api/v0/addresses/init`; // Already initiated!
/*
Request:
{
  "operator": "0x1234...5678",
  "treasury": "0x9876...4321"
}
Response:
{
  "transaction_hash": "0xabcd...1234"
}
*/
const POST_WINDOW_START = `http://localhost:${BACKEND_PORT}/api/v0/addresses/window/start`;
/*
Response:
{
  "count": 5,
  "addresses": [
    "0x1234...5678",
    "0x2345...6789",
    "0x3456...7890",
    "0x4567...8901",
    "0x5678...9012"
  ],
  "eth_addresses": [
    "0x1234...5678",
    "0x2345...6789",
    "0x3456...7890",
    "0x4567...8901",
    "0x5678...9012"
  ],
  "transaction_hash": "0xabcd...1234"
}
*/
const GET_WINDOW_STATUS = `http://localhost:${BACKEND_PORT}/api/v0/addresses/window/status`;
/*
Response:
{
    "active": false
}
*/
const POST_WINDOW_CLOSE = `http://localhost:${BACKEND_PORT}/api/v0/addresses/window/close`;
/*
Response:
{
 "transaction_hash": "0x012345"
 }
*/
const POST_MINT_TOKENS = `http://localhost:${BACKEND_PORT}/api/v0/token/mint-to`;
/*   
POST: 
{
    "address": "0x1234",
    "amount": 100
}
Response:
{
    "transaction_result": "12345"
}
*/
// const GET_TOKEN_BALANCE = `http://localhost:${BACKEND_PORT}/api/v0/token/balance/${WALLETADDRESS}`;
/*
Response:
{
    "balance": 100
}
*/
const POST_PLACE_BET = `http://localhost:${BACKEND_PORT}/api/v0/addresses/bets`;
/*
Request:
{
  "bettor": "0x1234...5678",
  "selected_address": "0x9876...4321",
  "position": true,
  "amount": "1000000000000000000"
}
Response:
{
  "transaction_hash": "0xabcd...1234"
}
*/
// const POST_PROCESS_PAY = `http://localhost:${BACKEND_PORT}/api/v0/addresses/payouts`; // Solidity SC will handle this
/*
Request:
[true, false, true, false, true]
Response:
{
  "status": "success",
  "message": "Payouts processed successfully",
  "transaction_hash": "0xabcd...1234"
}
*/

const GET_BET_COUNT = `http://localhost:${BACKEND_PORT}/api/v0/addresses/bets/count`;
/*
Response:
{
  "count": "42"
}
*/
const GET_BETTING_AMTS = `http://localhost:${BACKEND_PORT}/api/v0/addresses/amounts/{index}`;
/*
Response:
{
  "up_amount": "2000000000000000000",
  "down_amount": "1000000000000000000"
}
*/
const POST_VERIFY_AND_PAY = `http://localhost:3000/api/verify`;
/*
Request:
{
    "addresses":["0x1234...", "0x4567..."]
}
Response:
{
    "status": "success",
    "journal": "0x123456",
    "zkVerifyAttestation": {
        "attestationId": 42979,
        "proofDetails": {
            "root": "0x0b8861fd1226c0d08468e4053ae521253e8ac43a96cadbda47ab237f9d62870c",
            "proof": [
                "0x9748d439df3f8a81c26cc6d1e6a20e29010e22771b7d1bd7cd9d0c567bbdf805",
                "0xaf46de19988962222e0831bd1b9ee91c18817fbf463130233890af69ad1b899d"
            ],
            "numberOfLeaves": 4,
            "leafIndex": 1,
            "leaf": "0x6b34dab3f2bd512935146cc33f65d6f7f4015d4b1358b6940bf1765f60886f44"
        }
    }
    "transaction_hash": "0x1234"
}
*/

/**
 * Actions:
 * - Get Addresses
 * - Get Hash (and store on chain as a "proof")
 * - Start Betting Window
 * - Show Window status
 * - Mint tokens for betting
 * - Get token balance
 * - Place bet on the address shown from when the betting window was opened
 * - Close betting window
 * - Verify and process payouts
 *
 */

export function useGameActions() {
  const [loading, setLoading] = useState(false);
  const [gameData, setGameData] = useState<GameData[]>([]);
  const [selectedAddresses, setSelectedAddresses] = useState<string[]>([]);
  const [bets, setBets] = useState<Record<string, "top" | "bottom">>({});
  const [windowActive, setWindowActive] = useState(false);
  const [tokenBalance, setTokenBalance] = useState<string>("0");
  const [error, setError] = useState<string | null>(null);

  const account = useAccount();
  // Helper function for API calls
  const apiCall = async (endpoint: string, options: RequestInit = {}) => {
    try {
      const response = await fetch(`${API_BASE}${endpoint}`, {
        ...options,
        headers: {
          "Content-Type": "application/json",
          ...options?.headers,
        },
      });
      if (!response.ok) throw new Error(`API error: ${response.statusText}`);
      return await response.json();
    } catch (err: any) {
      setError(err.message);
      throw err;
    }
  };

  // Get all addresses
  const getAddresses = async () => {
    setLoading(true);
    try {
      const data = await apiCall("/addresses");
      setGameData(data);
      return data;
    } finally {
      setLoading(false);
    }
  };

  // Get and store hash on chain
  const getAndStoreHash = async () => {
    setLoading(true);
    try {
      const data = await apiCall("/addresses/hash/store");
      return data;
    } finally {
      setLoading(false);
    }
  };

  // Start betting window
  const startBettingWindow = async () => {
    setLoading(true);
    try {
      const data = await apiCall("/addresses/window/start", {
        method: "POST",
      });
      setSelectedAddresses(data.addresses);
      setWindowActive(true);
      return data;
    } finally {
      setLoading(false);
    }
  };

  // Get window status
  const getWindowStatus = async () => {
    try {
      const data = await apiCall("/addresses/window/status");
      setWindowActive(data.active);
      return data.active;
    } catch (error) {
      console.error("Error checking window status:", error);
      return false;
    }
  };

  // Mint tokens for betting
  const mintTokens = async (amount: number) => {
    setLoading(true);
    try {
      const data = await apiCall("/token/mint-to", {
        method: "POST",
        body: JSON.stringify({
          address: account.address,
          amount,
        }),
      });
      await getTokenBalance(); // Refresh balance after minting
      return data;
    } finally {
      setLoading(false);
    }
  };

  // Get token balance
  const getTokenBalance = async () => {
    try {
      const data = await apiCall(`/token/balance/${account.address}`);
      setTokenBalance(data.balance);
      return data.balance;
    } catch (error) {
      console.error("Error getting token balance:", error);
      return "0";
    }
  };

  // Place bet
  const placeBet = async (
    selectedAddress: string,
    position: boolean,
    amount: string
  ) => {
    setLoading(true);
    try {
      const data = await apiCall("/addresses/bets", {
        method: "POST",
        body: JSON.stringify({
          bettor: account.address,
          selected_address: selectedAddress,
          position,
          amount,
        }),
      });
      setBets((prev) => ({
        ...prev,
        [selectedAddress]: position ? "top" : "bottom",
      }));
      return data;
    } finally {
      setLoading(false);
    }
  };

  // Close betting window
  const closeBettingWindow = async () => {
    setLoading(true);
    try {
      const data = await apiCall("/addresses/window/close", {
        method: "POST",
      });
      setWindowActive(false);
      return data;
    } finally {
      setLoading(false);
    }
  };

  // Verify and process payouts using Next.js API route
  const verifyAndProcessPayouts = async () => {
    setLoading(true);
    try {
      // Using Next.js API route instead of backend route
      const response = await fetch("/api/verify", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          addresses: selectedAddresses,
        }),
      });

      if (!response.ok) {
        throw new Error(`Verification failed: ${response.statusText}`);
      }

      const data = await response.json();

      // Log successful verification and attestation details
      console.log("Verification successful:", {
        attestationId: data.zkVerifyAttestation?.attestationId,
        transactionHash: data.transaction_hash,
      });

      return data;
    } catch (error) {
      console.error("Error in verification process:", error);
      throw error;
    } finally {
      setLoading(false);
    }
  };

  // Get betting amounts for an address
  const getBettingAmounts = async (index: number) => {
    try {
      const data = await apiCall(`/addresses/amounts/${index}`);
      return data;
    } catch (error) {
      console.error("Error getting betting amounts:", error);
      return { up_amount: "0", down_amount: "0" };
    }
  };

  // Get total bet count
  const getBetCount = async () => {
    try {
      const data = await apiCall("/addresses/bets/count");
      return data.count;
    } catch (error) {
      console.error("Error getting bet count:", error);
      return "0";
    }
  };

  return {
    loading,
    gameData,
    selectedAddresses,
    bets,
    windowActive,
    tokenBalance,
    error,
    actions: {
      getAddresses,
      getAndStoreHash,
      startBettingWindow,
      getWindowStatus,
      mintTokens,
      getTokenBalance,
      placeBet,
      closeBettingWindow,
      verifyAndProcessPayouts,
      getBettingAmounts,
      getBetCount,
    },
  };
}
