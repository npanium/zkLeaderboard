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

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-4">
        <Input
          type="number"
          value={betAmount}
          onChange={(e: any) => setBetAmount(e.target.value)}
          placeholder="Bet amount in ETH"
          className="w-40"
          min="0"
          step="0.1"
        />
        <span className="text-sm text-muted-foreground">ETH</span>
      </div>

      <div className="max-h-96 overflow-y-auto">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Address</TableHead>
              {activeStep >= 2 && <TableHead>Place Bet</TableHead>}
            </TableRow>
          </TableHeader>
          <TableBody>
            {gameData.map((item) => (
              <TableRow
                key={item.address}
                className={
                  selectedAddresses.includes(item.address) ? "bg-blue-50" : ""
                }
              >
                <TableCell>{item.address}</TableCell>

                {activeStep >= 2 && (
                  <TableCell>
                    {selectedAddresses.includes(item.address) ? (
                      <div className="flex gap-2">
                        <Button
                          size="sm"
                          variant={
                            bets[item.address] === "top" ? "default" : "outline"
                          }
                          onClick={() => handlePlaceBet(item.address, true)}
                          disabled={placingBet === item.address}
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
                          disabled={placingBet === item.address}
                        >
                          {placingBet === item.address
                            ? "Placing Bet..."
                            : "Bottom 50%"}
                        </Button>
                      </div>
                    ) : (
                      "-"
                    )}
                  </TableCell>
                )}
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
