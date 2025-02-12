"use client";

import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardContent } from "@/components/ui/card";
import { useEffect, useState } from "react";
import { STEPS } from "../lib/constants";
import { ProgressStepper } from "../components/ProgressStepper";
import { GameLeaderboard } from "../components/GameLeaderboard";
import { InstructionPanel } from "../components/InstructionPanel";
import { useGameActions } from "../hooks/useGameActions";
import ArcadeMachine from "@/components/ArcadeMachine";
import { chakra } from "@/lib/fonts";
import { ConnectButton } from "@rainbow-me/rainbowkit";

import { useAccount } from "wagmi";

export default function Dashboard() {
  const [activeStep, setActiveStep] = useState(0);

  const { loading, gameData, selectedAddresses, bets, actions } =
    useGameActions();
  const account = useAccount();
  console.log(account.address);
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
        <div className="flex justify-between items-center mb-16">
          <h1
            className={`text-3xl font-bold m-auto underline underline-offset-4 ${chakra.className}`}
          >
            zkLeaderboard Demo
          </h1>
          <div className="space-x-4"></div>
        </div>

        <ArcadeMachine />
        <div className="flex justify-center">
          <ConnectButton />
        </div>
        {account.isConnected && (
          <>
            <div className="text-center my-10">
              <Button
                onClick={handleStepAction}
                disabled={loading || activeStep >= STEPS.length}
                className="mb-5"
              >
                {loading ? "Processing..." : STEPS[activeStep]?.buttonText}
              </Button>
              <InstructionPanel activeStep={activeStep} />
            </div>
            <ProgressStepper steps={STEPS} activeStep={activeStep} />
            {gameData.length > 0 && (
              <Card className="mb-8">
                <CardHeader>
                  <h2 className="text-xl font-semibold">Game Leaderboard</h2>
                  {activeStep >= 2 && (
                    <div className="mt-2 text-sm text-muted-foreground">
                      Selected addresses for betting:{" "}
                      {selectedAddresses.join(", ")}
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
          </>
        )}
      </div>
    </div>
  );
}
