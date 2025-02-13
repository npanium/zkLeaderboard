import { Card, CardContent } from "@/components/ui/card";
import { useEffect, useState } from "react";

interface InstructionPanelProps {
  activeStep: number;
  apiResponse?: {
    status?: string;
    transaction_hash?: string;
    attestationId?: number;
    message?: string;
  };
}

const STEP_DETAILS = {
  0: {
    steps: [""],
    instruction: "Click the button above to start generating game data",
  },
  1: {
    steps: [
      "› Generating addresses and score list...",
      "› Creating database...",
      "› Fetching addresses...",
    ],
    instruction:
      "Store the address data hash on-chain to ensure game integrity",
  },
  2: {
    steps: [
      "› Computing data hash...",
      "› Storing hash on blockchain...",
      "› Verifying hash storage...",
    ],
    instruction:
      "Start the betting window to allow players to place their bets",
  },
  3: {
    steps: [
      "› Opening betting window...",
      "› Selecting random addresses...",
      "› Initializing betting contract...",
      "› Activating betting period...",
    ],
    instruction:
      "Close the betting window and process results with zero-knowledge verification",
  },
};

export function InstructionPanel({
  activeStep,
  apiResponse,
}: InstructionPanelProps) {
  const [visibleSubSteps, setVisibleSubSteps] = useState<number>(0);
  const currentStepDetails =
    STEP_DETAILS[activeStep as keyof typeof STEP_DETAILS] || [];

  useEffect(() => {
    setVisibleSubSteps(0);
    if (currentStepDetails.steps.length) {
      const interval = setInterval(() => {
        setVisibleSubSteps((prev) => {
          if (prev < currentStepDetails.steps.length) return prev + 1;
          clearInterval(interval);
          return prev;
        });
      }, 800);
      return () => clearInterval(interval);
    }
    return () => setVisibleSubSteps(0);
  }, [activeStep, currentStepDetails.steps.length, apiResponse]);

  return (
    <Card className="bg-black">
      <CardContent className="p-4 font-mono">
        {currentStepDetails.steps
          .slice(0, visibleSubSteps)
          .map((step, index) => (
            <div key={index} className="text-green-400">
              {step}
              {activeStep !== 0 && "✓"}
            </div>
          ))}
        {visibleSubSteps < currentStepDetails.steps.length && (
          <div className="text-green-400 animate-pulse">
            {currentStepDetails.steps[visibleSubSteps]} ⏳
          </div>
        )}
        <div className="text-blue-400 mt-4">
          {currentStepDetails.instruction}
        </div>
        {/* API Response Display */}
        {apiResponse && (
          <div className="mt-4 border-t border-gray-700 pt-4">
            <div className="text-yellow-400">API Response:</div>
            {apiResponse.transaction_hash && (
              <div className="text-green-400">
                › Transaction Hash: {apiResponse.transaction_hash}
              </div>
            )}
            {apiResponse.attestationId && (
              <div className="text-green-400">
                › Attestation ID: {apiResponse.attestationId}
              </div>
            )}
            {apiResponse.status && (
              <div className="text-green-400">
                › Status: {apiResponse.status}
              </div>
            )}
            {apiResponse.message && (
              <div className="text-green-400">
                › Message: {apiResponse.message}
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
