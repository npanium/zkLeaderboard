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

interface GameLeaderboardProps {
  gameData: GameData[];
  activeStep: number;
  selectedAddresses: string[];
  bets: Record<string, "top" | "bottom">;
  onPlaceBet: (address: string, direction: "top" | "bottom") => void;
}

export function GameLeaderboard({
  gameData,
  activeStep,
  selectedAddresses,
  bets,
  onPlaceBet,
}: GameLeaderboardProps) {
  return (
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
                        onClick={() => onPlaceBet(item.address, "top")}
                      >
                        Top 50%
                      </Button>
                      <Button
                        size="sm"
                        variant={
                          bets[item.address] === "bottom"
                            ? "default"
                            : "outline"
                        }
                        onClick={() => onPlaceBet(item.address, "bottom")}
                      >
                        Bottom 50%
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
  );
}
