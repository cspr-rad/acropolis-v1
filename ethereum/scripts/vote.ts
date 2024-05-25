import fs from "fs";
import { ethers } from "hardhat";
import { VoteRecorder } from "../typechain-types";

const main = async (): Promise<void> => {
    const voteRecorderFactory = await ethers.getContractFactory("VoteRecorder");
    const voteRecorder = voteRecorderFactory.attach(
        fs.readFileSync(`${__dirname}/../CONTRACT_ADDRESS`, 'utf8')
    ) as VoteRecorder;
    const electionId = process.env["ELECTION_ID"] as string;
    const voteData = fs.readFileSync(process.env["VOTE_DATA_FILE"] as string, 'utf8');
    await voteRecorder.vote(electionId, voteData);
    console.log(`${(await voteRecorder.getVotesTallied(electionId)).toString()} vote(s) cast`)
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });