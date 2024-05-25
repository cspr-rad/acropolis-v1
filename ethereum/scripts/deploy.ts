import fs from "fs";
import { ethers } from "hardhat";
import "@nomicfoundation/hardhat-toolbox";

const main = async (): Promise<void> => {
    const voteRecorderFactory  = await ethers.getContractFactory("VoteRecorder");
    console.log("Deploying VoteRecorder...");
    const voteRecorder = await voteRecorderFactory.deploy();
    await voteRecorder.waitForDeployment();
    const voteRecorderAddress = await voteRecorder.getAddress();
    console.log(`VoteRecorder address: ${voteRecorderAddress}`);
    fs.writeFileSync(`${__dirname}/../CONTRACT_ADDRESS`, voteRecorderAddress);
} 

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });