import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { GameData } from "../lib/types";
import { useState } from "react";
import { Input } from "./ui/input";

interface GameLeaderboardProps {
  gameData: GameData[];
  activeStep: number;
  selectedAddresses: string[];
  bets: Record<string, "top" | "bottom">;
  onPlaceBet: (
    address: string,
    position: boolean,
    amount: string
  ) => Promise<boolean>;
}

export function GameLeaderboard({
  gameData,
  activeStep,
  selectedAddresses,
  bets,
  onPlaceBet,
}: GameLeaderboardProps) {
  const [betAmount, setBetAmount] = useState<string>("1");
  const [placingBet, setPlacingBet] = useState<string | null>(null);

  const handlePlaceBet = async (address: string, position: boolean) => {
    setPlacingBet(address);
    try {
      // Convert betAmount to wei (assuming input is in ETH)
      const amountInWei = (parseFloat(betAmount) * 1e18).toString();
      await onPlaceBet(address, position, amountInWei);
    } catch (error) {
      console.error("Error placing bet:", error);
    } finally {
      setPlacingBet(null);
    }
  };

  const displayData =
    activeStep >= 2
      ? gameData.filter((item) => selectedAddresses.includes(item.address))
      : gameData;
  return (
    <div className="space-y-4">
      {activeStep >= 2 && (
        <div className="flex items-center gap-4">
          <Input
            type="number"
            value={betAmount}
            onChange={(e: any) => setBetAmount(e.target.value)}
            placeholder="Bet amount in $BET"
            className="w-40"
            min="0"
            step="0.1"
          />
          <span className="text-sm text-muted-foreground">$BET</span>
        </div>
      )}

      <div className="max-h-96 overflow-y-auto">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Address</TableHead>
              {activeStep >= 2 && (
                <>
                  <TableHead>Status</TableHead>
                  <TableHead>Actions</TableHead>
                </>
              )}
            </TableRow>
          </TableHeader>
          <TableBody>
            {displayData.map((item) => (
              <TableRow key={item.address}>
                <TableCell>{item.address}</TableCell>
                {activeStep >= 2 && (
                  <>
                    <TableCell>
                      {bets[item.address]
                        ? `Bet placed: ${
                            bets[item.address] === "top"
                              ? "Top 50%"
                              : "Bottom 50%"
                          }`
                        : "No bet placed"}
                    </TableCell>
                    <TableCell>
                      <div className="flex gap-2">
                        <Button
                          size="sm"
                          variant={
                            bets[item.address] === "top" ? "default" : "outline"
                          }
                          onClick={() => handlePlaceBet(item.address, true)}
                          disabled={!!placingBet || !!bets[item.address]} // Fixed boolean conversion
                        >
                          {placingBet === item.address
                            ? "Placing Bet..."
                            : "Top 50%"}
                        </Button>
                        <Button
                          size="sm"
                          variant={
                            bets[item.address] === "bottom"
                              ? "default"
                              : "outline"
                          }
                          onClick={() => handlePlaceBet(item.address, false)}
                          disabled={!!placingBet || !!bets[item.address]} // Fixed boolean conversion
                        >
                          {placingBet === item.address
                            ? "Placing Bet..."
                            : "Bottom 50%"}
                        </Button>
                      </div>
                    </TableCell>
                  </>
                )}
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
