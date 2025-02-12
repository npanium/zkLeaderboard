import { Card, CardContent } from "@/components/ui/card";
import { useEffect, useState } from "react";

interface InstructionPanelProps {
  activeStep: number;
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
    instruction: "Start the betting window by clicking the button above",
  },
  2: {
    steps: [
      "› Collecting bet data...",
      "› Initializing proof generation...",
      "› Computing ZK proofs...",
      "› Verifying proof integrity...",
    ],
    instruction: "Generating zero-knowledge proofs to verify your bets ",
  },
  3: {
    steps: [
      "› Processing all bets...",
      "› Calculating final positions...",
      "› Computing rewards...",
      "› Finalizing results...",
    ],
    instruction: "Finalizing results and calculating rewards",
  },
};

export function InstructionPanel({ activeStep }: InstructionPanelProps) {
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
  }, [activeStep, currentStepDetails.steps.length]);

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
      </CardContent>
    </Card>
  );
}
