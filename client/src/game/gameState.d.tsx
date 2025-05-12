import { Grave } from "./graveState";
import { ChatMessage } from "../components/ChatMessage";
import { Role, RoleState } from "./roleState.d";
import { RoleList } from "./roleListState.d";
import { LobbyPreviewData } from "./packet";
import { ChatFilter } from "../menu/game/gameScreenContent/ChatMenu";
import { ControllerID, SavedController } from "./abilityInput";
import translate from "./lang";
import ListMap, { ListMapData } from "../ListMap";

export type State = Disconnected | OutsideLobbyState | LobbyState | GameState;

export type Disconnected = {
    type: "disconnected"
}

export type OutsideLobbyState = {
    type: "outsideLobby",

    selectedRoomCode: string | null,
    lobbies: Map<number, LobbyPreviewData>,
}



export type ClientConnection = "connected" | "disconnected" | "couldReconnect";
export type GameClient = {
    clientType: GameClientType,
    connection: ClientConnection,
    host: boolean,
}
export type GameClientType = {
    type: "spectator",
    index: number
} | {
    type: "player",
    index: number,
}
export type LobbyClientType = {
    type: "spectator"
} | PlayerClientType;
export type PlayerClientType = {
    type: "player",
    name: string,
}



export type PlayerIndex = number;
export type LobbyClientID = number;
export type Verdict = "innocent"|"guilty"|"abstain";
export const PHASES = ["briefing", "obituary", "discussion", "nomination", "testimony", "judgement", "finalWords", "dusk", "night", "recess"] as const;
export type PhaseType = (typeof PHASES)[number];
export type PhaseState = {type: "briefing"} | {type: "recess"} | {type: "dusk"} | {type: "night"} | {type: "obituary"} | {type: "discussion"} | 
{
    type: "nomination",
    trialsLeft: number
} | {
    type: "testimony",
    playerOnTrial: PlayerIndex
    trialsLeft: number
} | {
    type: "judgement",
    playerOnTrial: PlayerIndex
    trialsLeft: number
} | {
    type: "finalWords",
    playerOnTrial: PlayerIndex
}

export type ChatGroup = "all" | "dead" | "mafia" | "cult" | "jail" | "kidnapper" | "interview" | "puppeteer";
export type InsiderGroup = (typeof INSIDER_GROUPS)[number];
export const INSIDER_GROUPS = ["mafia", "cult", "puppeteer"] as const;
export type PhaseTimes = Record<Exclude<PhaseType, "recess">, number>;
export type DefensePower = "none"|"armored"|"protected"|"invincible";

export type Tag = 
    "syndicateGun" |
    "godfatherBackup" |
    "werewolfTracked" |
    "doused" |
    "revolutionaryTarget" |
    "morticianTagged" |
    "puppeteerMarionette" |
    "frame" |
    "forfeitVote" |
    "spiraling";

export const MODIFIERS = [
    "obscuredGraves",
    "skipDay1",
    "deadCanChat", "abstaining",
    "noDeathCause",
    "roleSetGraveKillers", "autoGuilty", 
    "twoThirdsMajority", "noTrialPhases", 
    "noWhispers", "hiddenWhispers",
    "noNightChat", "noChat", 
    "unscheduledNominations"
] as const;
export type ModifierType = (typeof MODIFIERS)[number];


export const CONCLUSIONS = ["town", "mafia", "cult", "fiends", "politician", "niceList", "naughtyList", "draw"] as const;
export type Conclusion = (typeof CONCLUSIONS)[number];

export type WinCondition = {
    type: "gameConclusionReached"
    winIfAny: Conclusion[]
} | {
    type: "roleStateWon"
}

export function translateConclusion(conclusion: Conclusion): string {
    switch (conclusion) {
        case "politician":
            return translate("role.politician.name")
        case "draw":
            return translate("winCondition.draw")
        default:
            return translate(conclusion)
    }
}

export function translateWinCondition(winCondition: WinCondition): string {
    if (winCondition.type === "gameConclusionReached") {
        if (winCondition.winIfAny.length === 0) {
            return translate("winCondition.loser")
        } else if (winCondition.winIfAny.length === 1) {
            return translateConclusion(winCondition.winIfAny[0])
        } else if (winCondition.winIfAny.length === 4 && 
            (["mafia", "fiends", "cult", "politician"] as const).every(team => winCondition.winIfAny.includes(team))
        ) {
            return translate(`winCondition.evil`)
        } else {
            return winCondition.winIfAny.map(conclusion => translateConclusion(conclusion)).join(` ${translate('union')} `)
        }
    } else {
        return translate("winCondition.independent");
    }
}