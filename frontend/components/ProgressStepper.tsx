import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Step } from "../lib/types";

interface ProgressStepperProps {
  steps: Step[];
  activeStep: number;
}

export function ProgressStepper({ steps, activeStep }: ProgressStepperProps) {
  return (
    <div className="mb-12">
      <div className="flex justify-between relative">
        {steps.map((step, index) => (
          <div key={step.id} className="flex flex-col items-center w-1/4">
            <Badge
              variant={index <= activeStep - 1 ? "default" : "outline"}
              className="mb-2"
            >
              {step.id}
            </Badge>
            <span
              className={
                index <= activeStep - 1 ? "font-medium" : "text-gray-400"
              }
            >
              {step.name}
            </span>
          </div>
        ))}
      </div>
      <Progress
        value={(activeStep / steps.length) * 100}
        className="mt-4 h-2"
      />
    </div>
  );
}
