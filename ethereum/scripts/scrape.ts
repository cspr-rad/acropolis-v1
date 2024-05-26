import fs from "fs";
import { ethers } from "hardhat";
import { VoteRecorder } from "../typechain-types";

const main = async (): Promise<void> => {
    const voteRecorderFactory = await ethers.getContractFactory("VoteRecorder");
    const voteRecorder = voteRecorderFactory.attach(
        fs.readFileSync(`${__dirname}/../CONTRACT_ADDRESS`, 'utf8')
    ) as VoteRecorder;
    const electionId = process.env["ELECTION_ID"] as string;
    const votesTallied = await voteRecorder.getVotesTallied(electionId);
    let voteData = "";
    for (let i = 0n; i < votesTallied; i++) {
        voteData += `${await voteRecorder.getVoteReceipt(electionId, i)}\n`;
    }
    const scrapeFile = process.env["VOTE_SCRAPE_FILE"] as string;
    console.log(`Scraped ${votesTallied.toString()} votes from election ${electionId} to ${scrapeFile}`)
    fs.writeFileSync(scrapeFile, voteData, "utf-8");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });