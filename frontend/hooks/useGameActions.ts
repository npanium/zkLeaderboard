import { useState } from "react";
import { GameData } from "../lib/types";

const ADDRESSES = "http://localhost:3001/api/v0/addresses";

export function useGameActions() {
  const [loading, setLoading] = useState(false);
  const [gameData, setGameData] = useState<GameData[]>([]);
  const [selectedAddresses, setSelectedAddresses] = useState<string[]>([]);
  const [bets, setBets] = useState<Record<string, "top" | "bottom">>({});

  const simulateGame = async () => {
    setLoading(true);
    try {
      const response = await fetch(ADDRESSES);
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
