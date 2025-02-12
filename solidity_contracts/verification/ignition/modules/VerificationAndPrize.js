const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

const VerificationAndPrizeModule = buildModule("VerificationAndPrize", (m) => {
  const verificationAndPrize = m.contract("VerificationAndPrize", [
    "0xaedda71dee300fdcae185407aa30b75b254f4caf",
  ]);
  return { verificationAndPrize };
});

module.exports = VerificationAndPrizeModule;
