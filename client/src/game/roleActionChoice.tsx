import { PlayerIndex } from "./gameState.d";
import { DoomsayerGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeDoomsayerMenu";
import { OjoAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallOjoMenu";
import { KiraGuess } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";
import { RecruiterAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/RecruiterMenu";
import { Role } from "./roleState.d";

export type RoleActionChoice = 
(
    {type: "villager"}|
    {type: "flowerGirl"}|{type: "psychic"}|
    {type: "rabbleRouser"}|
    {type: "mayor"}|
    
    {type: "madeMan"}|

    {type: "disciple"}|
    
    {type: "politician"}|{type: "revolutionary"}
) |
({boolean: boolean} & (
    {type: "armorsmith"}|
    {type: "veteran"}|
    {type: "martyr"}
)) |
({player: PlayerIndex | null} & (
        {type: "detective"}|{type: "tracker"}|{type: "lookout"}|{type: "spy"}|{type: "snoop"}|{type: "gossip"}|
        {type: "doctor"}|{type: "bodyguard"}|{type: "cop"}|{type: "bouncer"}|{type: "engineer"}|
        {type: "vigilante"}|{type: "deputy"}|
        {type: "escort"}|{type: "medium"}|

        {type: "mafioso"}|
        {type: "consort"}|{type: "blackmailer"}|{type: "informant"}|{type: "mortician"}|

        {type: "pyrolisk"}|{type: "werewolf"}|{type: "arsonist"}|

        {type: "apostle"}|{type: "zealot"}|

        {type: "scarecrow"}|{type: "death"}|{type: "jester"}
)) | 
({role: Role | null} & (
    {type: "steward"}|
    
    {type: "mafiaKillingWildcard"}|
    {type: "mafiaSupportWildcard"}|
    
    {type: "fiendsWildcard"}|

    {type: "wildcard"}|{type: "trueWildcard"}
)) |
({twoPlayers: [PlayerIndex, PlayerIndex] | null} & (
    {type: "philosopher"}|
    {type: "transporter"}|{type: "retributionist"}|

    {type: "witch"}|{type: "necromancer"}|{type:"cupid"}|

    {type: "minion"}
)) | 
{
    type: "jailor",
    action: {
        type: "jail",
        target: PlayerIndex | null
    } | {
        type: "attack",
        shouldAttack: boolean
    }
} | {
    type: "auditor",
    chosenOutline: number | null,
} | {
    type: "marksman"
    marks: PlayerIndex[]
    camps: PlayerIndex[] 
} | {
    type: "journalist",
    action: {
        type: "setJournal",
        journal: string
        public: boolean
    } | {
        type: "interviewPlayer",
        player: PlayerIndex | null
    }
} | {
    type: "godfather"
    action: {
        type: "setBackup",
        backup: PlayerIndex | null
    } | {
        type: "setAttack"
        target: PlayerIndex | null
    }
} | {
    type: "retrainer"
    action: {
        type: "setBackup",
        backup: PlayerIndex | null
    } | {
        type: "setAttack"
        target: PlayerIndex | null
    } | {
        type: "retrain"
        role: Role
    }
} | {
    type: "imposter"
    action: {
        type: "setBackup",
        backup: PlayerIndex | null
    } | {
        type: "setAttack"
        target: PlayerIndex | null
    } | {
        type: "setFakeRole"
        role: Role
    }
} | {
    type: "eros"
    action: {
        type: "setAttack"
        target: PlayerIndex | null
    } | {
        type: "setLoveLink"
        targets: [PlayerIndex, PlayerIndex] | null
    }
} | {
    type: "counterfeiter",
    action: {
        type: "setBackup",
        backup: PlayerIndex | null
    } | {
        type: "setForge",
        role: Role,
        alibi: string,
        action: "forge" | "noForge"
    } | {
        type: "setAttack"
        target: PlayerIndex | null
    }
} | {
    type: "recruiter",
    action: {
        type: "setBackup",
        backup: PlayerIndex | null
    } | {
        type: "setAttack"
        target: PlayerIndex | null
    } | {
        type: "setAction",
        action: RecruiterAction
    }
} | {
    type: "doomsayer",
    guesses: [
        [PlayerIndex, DoomsayerGuess],
        [PlayerIndex, DoomsayerGuess],
        [PlayerIndex, DoomsayerGuess],
    ]
} | {
    type: "forger",
    action: {
        type: "setForge",
        role: Role,
        alibi: string,
    } | {
        type: "setTarget"
        target: PlayerIndex | null
    }
} | {
    type: "hypnotist"
    roleblock: boolean,
    
    youWereRoleblockedMessage: boolean,
    youSurvivedAttackMessage: boolean,
    youWereProtectedMessage: boolean,
    youWereTransportedMessage: boolean,
    youWerePossessedMessage: boolean,
    yourTargetWasJailedMessage: boolean
} | {
    type: "framer"
    frameTarget: {
        type: "aura",
        target: PlayerIndex
    } | {
        type: "auraAndVisit",
        target: PlayerIndex,
        visit: PlayerIndex
    } | {
        type: "none"
    }
} | {
    type: "ojo"
    action: OjoAction
} | {
    type: "puppeteer"
    action: {
        type: "string",
        target: PlayerIndex
    } | {
        type: "poison",
        target: PlayerIndex
    } | {
        type: "none"
    }
} | {
    type: "kira"
    guesses: Record<PlayerIndex, KiraGuess>
}