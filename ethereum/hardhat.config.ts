import { HardhatUserConfig, task } from "hardhat/config";
import fs from "fs";
import path from "path";
import "@nomicfoundation/hardhat-toolbox";

task(
  "balances",
  "Prints the list of ETH account balances",
  async (_args, hre): Promise<void> => {
    // @ts-ignore
    const accounts = await hre.ethers.getSigners();
    for (const account of accounts) {
      // @ts-ignore
      const balance = await hre.ethers.provider.getBalance(account.address);
      console.log(`${account.address} has balance ${balance.toString()}`);
    }
  },
);

const config: HardhatUserConfig = {
  solidity: "0.8.24",

  networks: {
    localnet: {
      url: `http://127.0.0.1:${fs.readFileSync(path.resolve(__dirname, 'PORT'), 'utf8')}`,
      // These are private keys associated with prefunded test accounts created by the ethereum-package
      // https://github.com/kurtosis-tech/ethereum-package/blob/main/src/prelaunch_data_generator/genesis_constants/genesis_constants.star
      accounts: [
        // 0x8943545177806ED17B9F23F0a21ee5948eCaa776
        "bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31",

        // 0xE25583099BA105D9ec0A67f5Ae86D90e50036425
        "39725efee3fb28614de3bacaffe4cc4bd8c436257e2c8bb887c4b5c4be45e76d",

        // 0x614561D2d143621E126e87831AEF287678B442b8
        "53321db7c1e331d93a11a41d16f004d7ff63972ec8ec7c25db329728ceeb1710",

        // 0xf93Ee4Cf8c6c40b329b0c0626F28333c132CF241
        "ab63b23eb7941c1251757e24b3d2350d2bc05c3c388d06f8fe6feafefb1e8c70",

        // 0x802dCbE1B1A97554B4F50DB5119E37E8e7336417
        "5d2344259f42259f82d2c140aa66102ba89b57b4883ee441a8b312622bd42491",

        // 0xAe95d8DA9244C37CaC0a3e16BA966a8e852Bb6D6
        "27515f805127bebad2fb9b183508bdacb8c763da16f54e0678b16e8f28ef3fff",

        // 0x2c57d1CFC6d5f8E4182a56b4cf75421472eBAEa4
        "7ff1a4c1d57e5e784d327c4c7651e952350bc271f156afb3d00d20f5ef924856",

        // 0x741bFE4802cE1C4b5b00F9Df2F5f179A1C89171A
        "3a91003acaf4c21b3953d94fa4a6db694fa69e5242b2e37be05dd82761058899",

        // 0xc3913d4D8bAb4914328651C2EAE817C8b78E1f4c
        "bb1d0f125b4fb2bb173c318cdead45468474ca71474e2247776b2b4c0fa2d3f5",

        // 0x65D08a056c17Ae13370565B04cF77D2AfA1cB9FA
        "850643a0224065ecce3882673c21f56bcf6eef86274cc21cadff15930b59fc8c",

        // 0x3e95dFbBaF6B348396E6674C7871546dCC568e56
        "94eb3102993b41ec55c241060f47daa0f6372e2e3ad7e91612ae36c364042e44",
      ],
    },
  },
};

export default config;
