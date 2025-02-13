import { useState } from "react";
import { GameData } from "../lib/types";

const BACKEND_PORT = 3001;
const WALLETADDRESS = "0x67f1452b3099CfB27E708130421c98aD2319C0b7";
const ARBISCAN_TXN = "https://sepolia.arbiscan.io/tx/";

const GET_ADDRESSES = `http://localhost:${BACKEND_PORT}/api/v0/addresses`;
/*
Response:
[
    {
        "id": null,
        "address": "0x1234",
        "score": 189
    },
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
const GET_TOKEN_BALANCE = `http://localhost:${BACKEND_PORT}/api/v0/token/balance/${WALLETADDRESS}`;
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
}
*/

export function useGameActions() {
  const [loading, setLoading] = useState(false);
  const [gameData, setGameData] = useState<GameData[]>([]);
  const [selectedAddresses, setSelectedAddresses] = useState<string[]>([]);
  const [bets, setBets] = useState<Record<string, "top" | "bottom">>({});

  const simulateGame = async () => {
    setLoading(true);
    try {
      const response = await fetch(GET_ADDRESSES);
      const data = await response.json();
      setGameData(data);
      return true;
    } catch (error) {
      console.error("Error simulating game:", error);
      return false;
    } finally {
      setLoading(false);
    }
  };

  const startBetting = () => {
    const randomAddresses = gameData
      .sort(() => 0.5 - Math.random())
      .slice(0, 5)
      .map((item) => item.address);
    setSelectedAddresses(randomAddresses);
    return true;
  };

  const generateProofs = async () => {
    setLoading(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 2000));
      return true;
    } catch (error) {
      console.error("Error generating proofs:", error);
      return false;
    } finally {
      setLoading(false);
    }
  };

  const resolveBets = () => {
    // Implement bet resolution logic
    return true;
  };

  const placeBet = (address: string, direction: "top" | "bottom") => {
    setBets((prev: any) => ({ ...prev, [address]: direction }));
  };

  return {
    loading,
    gameData,
    selectedAddresses,
    bets,
    actions: {
      simulateGame,
      startBetting,
      generateProofs,
      resolveBets,
      placeBet,
    },
  };
}
