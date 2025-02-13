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
  const [showAdditionalActions, setShowAdditionalActions] = useState(false);
  const [isBettingWindowActive, setIsBettingWindowActive] = useState(false);
  const [apiResponse, setApiResponse] = useState<any>(null);

  const { loading, gameData, selectedAddresses, bets, tokenBalance, actions } =
    useGameActions();

  const account = useAccount();

  // Handle primary game flow
  const handleStepAction = async () => {
    let success = false;
    let response = null;

    switch (activeStep) {
      case 0: // Get Addresses
        response = await actions.getAddresses();
        if (response) {
          setShowAdditionalActions(true);
          setApiResponse({
            status: "success",
            message: `Retrieved ${response.length} addresses`,
          });
          success = true;
        }
        break;

      case 1: // Store Hash
        response = await actions.getAndStoreHash();
        if (response) {
          setApiResponse({
            status: "success",
            transaction_hash: response.transaction_hash,
            message: `Hash stored with ${response.record_count} records`,
          });
          success = true;
        }
        break;

      case 2: // Start Betting Window
        response = await actions.startBettingWindow();
        if (response) {
          setIsBettingWindowActive(true);
          setApiResponse({
            status: "success",
            transaction_hash: response.transaction_hash,
            message: `Betting window opened with ${response.count} addresses`,
          });
          // Get initial betting amounts and count
          await actions.getBettingAmounts(0);
          await actions.getBetCount();
          success = true;
        }
        break;

      case 3: // Close Window and Process
        response = await actions.closeBettingWindow();
        if (response) {
          setIsBettingWindowActive(false);
          setApiResponse({
            status: "success",
            transaction_hash: response.transaction_hash,
            message: "Betting window closed",
          });

          // Verify and process payouts
          const verificationResult = await actions.verifyAndProcessPayouts();
          if (verificationResult) {
            setApiResponse({
              status: "success",
              transaction_hash: verificationResult.transaction_hash,
              attestationId:
                verificationResult.zkVerifyAttestation?.attestationId,
              message: "Verification complete and payouts processed",
            });
            success = true;
          }
        }
        break;
    }

    if (success) {
      setActiveStep((prev) => prev + 1);
    }
  };

  // Handle additional actions that can be performed anytime
  const handleMintTokens = async () => {
    if (!account.address) return;
    await actions.mintTokens(100); // Example amount
  };

  const handleGetBalance = async () => {
    if (!account.address) return;
    await actions.getTokenBalance();
  };

  const handleGetHash = async () => {
    await actions.getAndStoreHash();
  };

  // Betting window specific actions
  const handleGetBettingInfo = async () => {
    if (!isBettingWindowActive) return;
    const amounts = await actions.getBettingAmounts(0);
    const count = await actions.getBetCount();
    console.log({ amounts, count });
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
        </div>

        <ArcadeMachine />
        <div className="flex justify-center">
          <ConnectButton />
        </div>

        {account.isConnected && (
          <>
            <div className="text-center my-10">
              {/* Main game flow button */}
              <Button
                onClick={handleStepAction}
                disabled={loading || activeStep >= STEPS.length}
                className="mb-5"
              >
                {loading ? "Processing..." : STEPS[activeStep]?.buttonText}
              </Button>

              {/* Additional actions that can be performed anytime after initial setup */}
              {showAdditionalActions && (
                <div className="flex justify-center gap-4 mt-4">
                  <Button onClick={handleMintTokens} disabled={loading}>
                    Mint Tokens
                  </Button>
                  <Button onClick={handleGetBalance} disabled={loading}>
                    Get Balance
                  </Button>
                  <Button onClick={handleGetHash} disabled={loading}>
                    Get Hash
                  </Button>
                  {isBettingWindowActive && (
                    <Button onClick={handleGetBettingInfo} disabled={loading}>
                      Get Betting Info
                    </Button>
                  )}
                </div>
              )}

              {/* Display token balance if available */}
              {tokenBalance && (
                <div className="mt-4">Token Balance: {tokenBalance}</div>
              )}

              <InstructionPanel
                activeStep={activeStep}
                apiResponse={apiResponse}
              />
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
