"use client";

import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardContent } from "@/components/ui/card";
import { useState } from "react";
import { STEPS } from "../lib/constants";
import { ProgressStepper } from "../components/ProgressStepper";
import { GameLeaderboard } from "../components/GameLeaderboard";
import { InstructionPanel } from "../components/InstructionPanel";
import { useGameActions } from "../hooks/useGameActions";

export default function Dashboard() {
  const [activeStep, setActiveStep] = useState(0);
  const { loading, gameData, selectedAddresses, bets, actions } =
    useGameActions();

  const handleStepAction = async () => {
    let success = false;

    switch (activeStep) {
      case 0:
        success = await actions.simulateGame();
        break;
      case 1:
        success = actions.startBetting();
        break;
      case 2:
        success = await actions.generateProofs();
        break;
      case 3:
        success = actions.resolveBets();
        break;
    }

    if (success) {
      setActiveStep((prev) => prev + 1);
    }
  };

  return (
    <div className="min-h-screen p-8">
      <div className="max-w-6xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-3xl font-bold">zkLeaderboard</h1>
          <div className="space-x-4">
            <Button
              onClick={handleStepAction}
              disabled={loading || activeStep >= STEPS.length}
            >
              {loading ? "Processing..." : STEPS[activeStep]?.buttonText}
            </Button>
          </div>
        </div>

        <ProgressStepper steps={STEPS} activeStep={activeStep} />

        {gameData.length > 0 && (
          <Card className="mb-8">
            <CardHeader>
              <h2 className="text-xl font-semibold">Game Leaderboard</h2>
              {activeStep >= 2 && (
                <div className="mt-2 text-sm text-muted-foreground">
                  Selected addresses for betting: {selectedAddresses.join(", ")}
                </div>
              )}
            </CardHeader>
            <CardContent>
              <GameLeaderboard
                gameData={gameData}
                activeStep={activeStep}
                selectedAddresses={selectedAddresses}
                bets={bets}
                onPlaceBet={actions.placeBet}
              />
            </CardContent>
          </Card>
        )}

        <InstructionPanel activeStep={activeStep} />
      </div>
    </div>
  );
}
