import translate, { translateChecked } from "../game/lang";
import React from "react";
import GAME_MANAGER, { find, replaceMentions } from "..";
import StyledText, { KeywordDataMap, PLAYER_SENDER_KEYWORD_DATA } from "./StyledText";
import "./chatMessage.css"
import { ChatGroup, PhaseState, PlayerIndex, Tag, Verdict } from "../game/gameState.d";
import { Role } from "../game/roleState.d";
import { Grave } from "../game/graveState";
import DOMPurify from "dompurify";
import GraveComponent from "./grave";
import { RoleOutline, translateRoleOutline } from "../game/roleListState.d";
import { AuditorResult } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeAuditorMenu";
import { OjoAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallOjoMenu";
import { PuppeteerAction } from "../menu/game/gameScreenContent/RoleSpecificMenus/SmallPuppeteerMenu";
import { KiraGuess, KiraGuessResult, kiraGuessTranslate } from "../menu/game/gameScreenContent/RoleSpecificMenus/LargeKiraMenu";
import { CopyButton } from "./ClipboardButtons";

const ChatElement = React.memo((
    props: {
        message: ChatMessage,
        playerNames?: string[],
        playerKeywordData?: KeywordDataMap,
        playerSenderKeywordData?: KeywordDataMap
    }, 
) => {
    const [mouseHovering, setMouseHovering] = React.useState(false); 

    const message = props.message;
    const playerNames = props.playerNames ?? GAME_MANAGER.getPlayerNames();
    const chatMessageStyles = require("../resources/styling/chatMessage.json");
    if(message.variant === undefined){
        console.error("ChatElement message with undefined variant:");
        console.error(message);
    }
    let style = typeof chatMessageStyles[message.variant.type] === "string" ? chatMessageStyles[message.variant.type] : "";

    let chatGroupIcon = null;
    if(message.chatGroup !== null){
        if(message.chatGroup !== "all"){
            chatGroupIcon = translateChecked("chatGroup."+message.chatGroup+".icon");
        }else{
            chatGroupIcon = "";
        }
    }else{
        chatGroupIcon = translate("noGroup.icon");
    }

    // Special chat messages that don't play by the rules
    switch (message.variant.type) {
        case "lobbyMessage":
            if (containsMention(message.variant, playerNames)) {
                style += " mention";
            }

            return <div className={"chat-message-div"}><span className={`chat-message ${style}`}>
                <StyledText
                    playerKeywordData={props.playerSenderKeywordData ?? PLAYER_SENDER_KEYWORD_DATA}
                >{chatGroupIcon ?? ""} {message.variant.sender}: </StyledText>
                <StyledText
                    playerKeywordData={props.playerKeywordData}
                >{translateChatMessage(message.variant, playerNames)}</StyledText>
            </span></div>;
        case "normal":
            if(message.variant.messageSender.type !== "player" && message.variant.messageSender.type !== "livingToDead"){
                style += " discreet";
            } else if (message.chatGroup === "dead") {
                style += " dead player";
            } else {
                style += " player"
            }
            
            if (message.variant.messageSender.type === "livingToDead") {
                chatGroupIcon += translate("messageSender.livingToDead.icon")
            }

            let messageSender = "";
            if (message.variant.messageSender.type === "player"||message.variant.messageSender.type === "livingToDead") {
                messageSender = playerNames[message.variant.messageSender.player];
            }else if(message.variant.messageSender.type === "jailor" || message.variant.messageSender.type === "journalist"){
                messageSender = translate("role."+message.variant.messageSender.type+".name");
            }
            
            if (containsMention(message.variant, playerNames)) {
                style += " mention";
            }

            return <div
                className={"chat-message-div"}
                onMouseOver={() => setMouseHovering(true)}
                onMouseOut={() => setMouseHovering(false)}
            >
                <span className={`chat-message ${style}`}>
                    <StyledText
                        playerKeywordData={props.playerSenderKeywordData ?? PLAYER_SENDER_KEYWORD_DATA}
                    >
                        {chatGroupIcon ?? ""} {messageSender}: </StyledText>
                    <StyledText
                        playerKeywordData={props.playerSenderKeywordData}
                    >
                        {translateChatMessage(message.variant, playerNames)}
                    </StyledText>
                </span>
                {
                    mouseHovering &&
                    GAME_MANAGER.state.stateType === "game" && 
                    GAME_MANAGER.state.clientState.type === "player" && 
                    GAME_MANAGER.state.clientState.roleState?.type === "forger" &&
                    <CopyButton
                        className="chat-message-div-copy-button"
                        text={translateChatMessage(message.variant, playerNames)}
                    />
                }
            </div>;
        case "targetsMessage":
            return <div className={"chat-message-div"}><span className="chat-message result">
                <StyledText className={"chat-message " + style}
                    playerKeywordData={props.playerKeywordData}
                >
                    {(chatGroupIcon??"")} {translateChatMessage(message.variant, playerNames)}
                </StyledText>
                <ChatElement {...props} message={{
                    variant: message.variant.message,
                    chatGroup: message.chatGroup,
                }}/>
            </span>
        </div>
        case "kiraResult":
            let out = [];

            let sortedPlayerIndexes = Object.keys(message.variant.result.guesses).map((k)=>{return Number.parseInt(k)}).sort();

            for(let playerIndex of sortedPlayerIndexes){
                let resultStyle = "";
                let resultIcon = "";
                let resultString = "";

                if(message.variant.result.guesses[playerIndex][1] === "correct"){
                    resultStyle = "correct";
                    resultIcon = "🟩";
                    resultString = translate("kiraResult.correct");
                }else if(message.variant.result.guesses[playerIndex][1] === "wrongSpot"){
                    resultStyle = "wrongSpot";
                    resultIcon = "🟨";
                    resultString = translate("kiraResult.wrongSpot");
                }else if(message.variant.result.guesses[playerIndex][1] === "notInGame"){
                    resultStyle = "notInGame";
                    resultIcon = "🟥";
                    resultString = translate("kiraResult.notInGame");
                }

                if(message.variant.result.guesses[playerIndex][0] === "none"){
                    resultStyle = "";
                    resultIcon = "";
                    resultString = "";
                }

                out.push(<div key={playerIndex} className={"kira-guess-result "+resultStyle}>
                    <StyledText
                        playerKeywordData={props.playerKeywordData}
                    >
                        {playerNames[playerIndex]} {kiraGuessTranslate(message.variant.result.guesses[playerIndex][0])} {resultIcon} {resultString}
                    </StyledText>
                </div>)
            }

            return <div className={"chat-message-div chat-message kira-guess-results " + style}>
                <StyledText
                    className="chat-message result"
                    playerKeywordData={props.playerKeywordData}
                >{chatGroupIcon ?? ""} {translate("chatMessage.kiraResult")}</StyledText>
                {out}
            </div>
        case "playerDied":

            let graveRoleString: string;
            switch (message.variant.grave.information.type) {
                case "obscured":
                    graveRoleString = translate("obscured");
                    break;
                case "normal":
                    graveRoleString = translate("role."+message.variant.grave.information.role+".name");
                    break;
            }

            return <div className={"chat-message-div"}>
                <details open={GAME_MANAGER.getMySpectator()}>
                    <summary>
                        <StyledText className={"chat-message " + style}
                            playerKeywordData={props.playerKeywordData}
                        >
                            {(chatGroupIcon??"")} {translate("chatMessage.playerDied",
                                playerNames[message.variant.grave.player], graveRoleString
                            )}
                        </StyledText>
                    </summary>
                    <div className="grave-message">
                        <GraveComponent grave={message.variant.grave} playerNames={playerNames}/>
                    </div>
                </details>
            </div>;
    }

    return <div
        className={"chat-message-div " + style}
        onMouseOver={() => setMouseHovering(true)}
        onMouseOut={() => setMouseHovering(false)}
    >
        <StyledText className={"chat-message " + style} playerKeywordData={props.playerKeywordData}>
            {(chatGroupIcon??"")} {translateChatMessage(message.variant, playerNames)}
        </StyledText>
        {
            mouseHovering &&
            GAME_MANAGER.state.stateType === "game" && 
            GAME_MANAGER.state.clientState.type === "player" && 
            GAME_MANAGER.state.clientState.roleState?.type === "forger" &&
            <CopyButton
                className="chat-message-div-copy-button"
                text={translateChatMessage(message.variant, playerNames)}
            />
        }
    </div>;
});

function containsMention(message: ChatMessageVariant & { text: string }, playerNames: string[]): boolean {
    let myNumber: number | null = null;
    let myName: string | null = null;
    if (GAME_MANAGER.state.stateType === "game" && GAME_MANAGER.state.clientState.type === "player" && GAME_MANAGER.state.clientState.myIndex !== null) {
        myName = GAME_MANAGER.state.players[GAME_MANAGER.state.clientState.myIndex].name ?? ""
        myNumber = GAME_MANAGER.state.clientState.myIndex;
    } else if (GAME_MANAGER.state.stateType === "lobby") {
        const myPlayer = GAME_MANAGER.state.players.get(GAME_MANAGER.state.myId!);
        if (myPlayer !== null && myPlayer?.clientType.type === "player") {
            myName = myPlayer.clientType.name;
        } else {
            return false
        }
    }
    if (myName === null) {
        return false;
    }
    return (
        find(myName).test(sanitizePlayerMessage(replaceMentions(message.text, playerNames))) ||
        (
            myNumber !== null && 
            find("" + (myNumber + 1)).test(sanitizePlayerMessage(replaceMentions(message.text, playerNames)))
        )
    )
}

export default ChatElement;

function playerListToString(playerList: PlayerIndex[], playerNames: string[]): string {

    return playerList.map((playerIndex) => {
        return playerNames[playerIndex];
    }).join(", ");
}

export function sanitizePlayerMessage(text: string): string {
    return DOMPurify.sanitize(text, { 
        ALLOWED_TAGS: []
    });
}

export function translateChatMessage(message: ChatMessageVariant, playerNames?: string[]): string {

    if (playerNames === undefined) {
        playerNames = GAME_MANAGER.getPlayerNames();
    }

    switch (message.type) {
        case "lobbyMessage":
            return sanitizePlayerMessage(replaceMentions(message.text, playerNames));
        case "normal":
            return sanitizePlayerMessage(replaceMentions(message.text, playerNames));
        case "whisper":
            return translate("chatMessage.whisper", 
                playerNames[message.fromPlayerIndex],
                playerNames[message.toPlayerIndex],
                sanitizePlayerMessage(replaceMentions(message.text, playerNames))
            );
        case "broadcastWhisper":
            return translate("chatMessage.broadcastWhisper",
                playerNames[message.whisperer],
                playerNames[message.whisperee],
            );
        case "roleAssignment":
            return translate("chatMessage.roleAssignment", 
                translate("role."+message.role+".name")
            );
        case "playersRoleRevealed":
            return translate("chatMessage.playersRoleRevealed",
                playerNames[message.player],
                translate("role."+message.role+".name")
            );
        case "playersRoleConcealed":
            return translate("chatMessage.playersRoleConcealed",
                playerNames[message.player]
            );
        case "tagAdded":
            return translate("chatMessage.tagAdded",
                playerNames[message.player],
                translate("tag."+message.tag+".name"),
                translate("tag."+message.tag)
            );
        case "tagRemoved":
            return translate("chatMessage.tagRemoved",
                playerNames[message.player],
                translate("tag."+message.tag+".name"),
                translate("tag."+message.tag)
            );
        case "playerWonOrLost":
            if(message.won){
                return translate("chatMessage.playerWon",
                    playerNames[message.player], translate("role."+message.role+".name")
                );
            }else{
                return translate("chatMessage.playerLost",
                    playerNames[message.player], translate("role."+message.role+".name")
                );
            }
        case "playerQuit":
            return translate("chatMessage.playerQuit",
                playerNames[message.playerIndex]
            );
        case "youDied":
            return translate("chatMessage.youDied");
        case "phaseChange":
            switch (message.phase.type) {
                case "nomination":
                    return translate("chatMessage.phaseChange.nomination",
                        translate("phase."+message.phase.type),
                        message.dayNumber,
                        message.phase.trialsLeft
                    );
                case "testimony":
                case "judgement":
                case "finalWords":
                    return translate("chatMessage.phaseChange.trial",
                        translate("phase."+message.phase.type),
                        message.dayNumber,
                        playerNames[message.phase.playerOnTrial]
                    );
                default:
                    return translate("chatMessage.phaseChange",
                        translate("phase."+message.phase.type),
                        message.dayNumber
                    );
            }
            
        case "trialInformation":
            return translate("chatMessage.trialInformation",
                message.requiredVotes,
                message.trialsLeft
            );
        case "voted":
            if (message.votee !== null) {
                return translate("chatMessage.voted",
                    playerNames[message.voter],
                    playerNames[message.votee],
                );
            } else {
                return translate("chatMessage.voted.cleared",
                    playerNames[message.voter],
                );
            }
        case "playerNominated":
            return translate("chatMessage.playerNominated",
                playerNames[message.playerIndex],
                playerListToString(message.playersVoted, playerNames)
            );
        case "judgementVerdict":
            return translate("chatMessage.judgementVerdict",
                playerNames[message.voterPlayerIndex],
                translate("verdict."+message.verdict.toLowerCase())
            );
        case "trialVerdict":
            return translate("chatMessage.trialVerdict",
                playerNames[message.playerOnTrial],
                message.innocent>=message.guilty?translate("verdict.innocent"):translate("verdict.guilty"),
                message.innocent,
                message.guilty
            );
        case "targeted":
            if (message.targets.length > 0) {
                return translate("chatMessage.targeted",
                    playerNames[message.targeter],
                    playerListToString(message.targets, playerNames));
            } else {
                return translate("chatMessage.targeted.cleared",
                    playerNames[message.targeter],
                );
            }
        case "mayorRevealed":
            return translate("chatMessage.mayorRevealed",
                playerNames[message.playerIndex],
            );
        case "martyrRevealed":
            return translate("chatMessage.martyrRevealed",
                playerNames[message.martyr],
            );
        case "journalistJournal":
            return translate("chatMessage.journalistJournal",
                sanitizePlayerMessage(replaceMentions(message.journal, playerNames))
            );
        case "youAreInterviewingPlayer":
            return translate("chatMessage.youAreInterviewingPlayer",
                playerNames[message.playerIndex],
            );
        case "playerIsBeingInterviewed":
            return translate("chatMessage.playerIsBeingInterviewed",
                playerNames[message.playerIndex],
            );
        case "jailedTarget":
            return translate("chatMessage.jailedTarget",
                playerNames[message.playerIndex],
            );
        case "jailedSomeone":
            return translate("chatMessage.jailedSomeone",
                playerNames[message.playerIndex]
            );
        case "deputyKilled":
            return translate("chatMessage.deputyKilled",
                playerNames[message.shotIndex]
            );
        case "puppeteerPlayerIsNowMarionette":
            return translate("chatMessage.puppeteerPlayerIsNowMarionette",
                playerNames[message.player]
            );
        case "jailorDecideExecute":
            if (message.target !== null) {
                return translate("chatMessage.jailorDecideExecute", playerNames[message.target]);
            } else {
                return translate("chatMessage.jailorDecideExecute.nobody");
            }
        case "godfatherBackup":
            if (message.backup !== null) {
                return translate("chatMessage.godfatherBackup", playerNames[message.backup]);
            } else {
                return translate("chatMessage.godfatherBackup.nobody");
            }
        /* NIGHT */
        case "godfatherBackupKilled":
            return translate("chatMessage.godfatherBackupKilled", playerNames[message.backup]);
        case "roleBlocked":
            return translate("chatMessage.roleBlocked" + (message.immune ? ".immune" : ""));
        case "sheriffResult":
            return translate("chatMessage.sheriffResult." + (message.suspicious ? "suspicious" : "innocent"));
        case "snoopResult":
            return translate("chatMessage.snoopResult." + (message.townie ? "townie" : "inconclusive"));
        case "gossipResult":
            return translate("chatMessage.gossipResult." + (message.enemies ? "enemies" : "none"));
        case "lookoutResult":
            if (message.players.length === 0) {
                return translate("chatMessage.lookoutResult.nobody");
            } else {
                return translate("chatMessage.lookoutResult", playerListToString(message.players, playerNames));
            }
        case "spyMafiaVisit":
            if (message.players.length === 0) {
                return translate("chatMessage.spyMafiaVisit.nobody");
            } else {
                return translate("chatMessage.spyMafiaVisit", playerListToString(message.players, playerNames));
            }
        case "spyCultistCount":
            if(message.count === 1){
                return translate("chatMessage.spyCultistCount.one");
            }else{
                return translate("chatMessage.spyCultistCount", message.count);
            }
        case "spyBug":
            return translate("chatMessage.spyBug."+message.bug);
        case "trackerResult":
            if (message.players.length === 0) {
                return translate("chatMessage.trackerResult.nobody");
            } else {
                return translate("chatMessage.trackerResult", playerListToString(message.players, playerNames));
            }
        case "seerResult":
            return translate("chatMessage.seerResult." + (message.enemies ? "enemies" : "friends"));
        case "psychicEvil":
            return translate("chatMessage.psychicEvil", playerListToString(message.players, playerNames));
        case "psychicGood":
            return translate("chatMessage.psychicGood", playerListToString(message.players, playerNames));
        case "auditorResult":
            if(message.result.type === "one"){
                return translate("chatMessage.auditorResult.one", 
                    translateRoleOutline(message.roleOutline),
                    translate("role."+message.result.role+".name")
                );
            }else{
                return translate("chatMessage.auditorResult.two", 
                    translateRoleOutline(message.roleOutline),
                    translate("role."+message.result.roles[0]+".name"),
                    translate("role."+message.result.roles[1]+".name")
                );
            }
        case "engineerVisitorsRole":
            return translate("chatMessage.engineerVisitorsRole", translate("role."+message.role+".name"));
        case "trapState":
            return translate("chatMessage.trapState."+message.state.type);
        case "trapStateEndOfNight":
            return translate("chatMessage.trapStateEndOfNight."+message.state.type);
        case "playerRoleAndAlibi":
            return translate("chatMessage.playerRoleAndAlibi",
                playerNames[message.player],
                translate("role."+message.role+".name"),
                sanitizePlayerMessage(replaceMentions(message.will, playerNames))
            );
        case "informantResult":
            const visitedNobody = message.visited.length === 0;
            const visitedByNobody = message.visitedBy.length === 0;

            return translate("chatMessage.informantResult",
                translate("chatMessage.informantResult.role", translate("role."+message.role+".name")),
                visitedNobody 
                    ? translate("chatMessage.informantResult.visited.nobody") 
                    : translate("chatMessage.informantResult.visited", playerListToString(message.visited, playerNames)),
                visitedByNobody 
                    ? translate("chatMessage.informantResult.visitedBy.nobody") 
                    : translate("chatMessage.informantResult.visitedBy", playerListToString(message.visitedBy, playerNames))
            );
        case "scarecrowResult":
            if(message.players.length === 0){
                return translate("chatMessage.scarecrowResult.nobody");
            }

            return translate("chatMessage.scarecrowResult",
                playerListToString(message.players, playerNames)
            );
        case "ojoActionChosen":
            switch (message.action.type) {
                case "kill":
                    return translate("chatMessage.ojoActionChosen.kill", translate("role."+message.action.role+".name"));
                case "see":
                    return translate("chatMessage.ojoActionChosen.see", translate("role."+message.action.role+".name"));
                case "none":
                    return translate("chatMessage.ojoActionChosen.none");
            }
            break;
        case "puppeteerActionChosen":
            return translate("chatMessage.puppeteerActionChosen."+message.action);
        case "erosActionChosen":
            return translate("chatMessage.erosActionChosen."+message.action);
        case "marksmanChosenMarks":
            if(message.marks.length === 0){
                return translate("chatMessage.marksmanChosenMarks.none");
            }
            return translate("chatMessage.marksmanChosenMarks", playerListToString(message.marks, playerNames));
        case "silenced":
            return translate("chatMessage.silenced");
        case "mediumHauntStarted":
            return translate("chatMessage.mediumHauntStarted", playerNames[message.medium], playerNames[message.player]);
        case "youWerePossessed":
            return translate("chatMessage.youWerePossessed" + (message.immune ? ".immune" : ""));
        case "possessionTargetsRole":
            return translate("chatMessage.possessionTargetsRole", translate("role."+message.role+".name"));
        case "werewolfTrackingResult":
            if(message.players.length === 0){
                return translate(
                    "chatMessage.werewolfTrackingResult.nobody", 
                    playerNames[message.trackedPlayer]
                );
            }else{
                return translate("chatMessage.werewolfTrackingResult", 
                    playerNames[message.trackedPlayer],
                    playerListToString(message.players, playerNames)
                );
            }
        case "cultSacrificesRequired":
            switch (message.required) {
                case 0:
                    return translate("chatMessage.cultSacrificesRequired.0");
                case 1:
                    return translate("chatMessage.cultSacrificesRequired.1");
                default:
                    return translate("chatMessage.cultSacrificesRequired", message.required);
            }
        case "wildcardConvertFailed":
            return translate("chatMessage.wildcardConvertFailed", translate("role."+message.role+".name"));
        case "youAreLoveLinked":
            return translate("chatMessage.youAreLoveLinked", playerNames[message.player]);
        case "playerDiedOfABrokenHeart":
            return translate("chatMessage.playerDiedOfBrokenHeart", playerNames[message.player], playerNames[message.lover]);
        case "deputyShotYou":
        case "deathCollectedSouls":
        case "targetWasAttacked":
        case "youWereProtected":
        case "provocateurWon":
        case "gameOver":
        case "jesterWon":
        case "wardblocked":
        case "yourConvertFailed":
        case "apostleCanConvertTonight":
        case "apostleCantConvertTonight":
        case "someoneSurvivedYourAttack":
        case "transported":
        case "veteranAttackedVisitor":
        case "veteranAttackedYou":
        case "vigilanteSuicide":
        case "targetIsPossessionImmune":
        case "youSurvivedAttack":
        case "doomsayerFailed":
        case "doomsayerWon":
        case "martyrFailed":
        case "martyrWon":
        case "targetsMessage":
        case "psychicFailed":
        case "phaseFastForwarded":
        case "mayorCantWhisper":
        case "youAttackedSomeone":
        case "youWereAttacked":
        case "puppeteerYouArePoisoned":
        case "armorsmithArmorBroke":
            return translate("chatMessage."+message.type);
        case "playerDied":
        case "kiraResult":
        default:
            console.error("Unknown message type " + (message as any).type + ":");
            console.error(message);
            return "FIXME: " + translate("chatMessage." + message);
    }
}
export type ChatMessage = {
    variant: ChatMessageVariant
    chatGroup: ChatGroup | null
}
export type ChatMessageVariant = {
    type: "lobbyMessage",
    sender: string,
    text: string
} | {
    type: "normal", 
    messageSender: MessageSender,
    text: string
} | {
    type: "whisper", 
    fromPlayerIndex: PlayerIndex, 
    toPlayerIndex: PlayerIndex, 
    text: string
} | {
    type: "broadcastWhisper", 
    whisperer: PlayerIndex, 
    whisperee: PlayerIndex 
} | 
// System
{
    type: "roleAssignment", 
    role: Role
} | {
    type: "playerDied", 
    grave: Grave
} | {
    type: "playersRoleRevealed",
    role: Role,
    player: PlayerIndex
} | {
    type: "playersRoleConcealed",
    player: PlayerIndex
} | {
    type: "tagAdded",
    player: PlayerIndex,
    tag: Tag
} | {
    type: "tagRemoved",
    player: PlayerIndex,
    tag: Tag
} | {
    type: "gameOver"
} | {
    type: "playerWonOrLost",
    player: PlayerIndex,
    won: boolean,
    role: Role
} | {
    type: "playerQuit",
    playerIndex: PlayerIndex
} | {
    type: "phaseChange", 
    phase: PhaseState,
    dayNumber: number
} | 
// Trial
{
    type: "trialInformation", 
    requiredVotes: number, 
    trialsLeft: number
} | {
    type: "voted", 
    voter: PlayerIndex, 
    votee: PlayerIndex | null 
} | {
    type: "playerNominated", 
    playerIndex: PlayerIndex,
    playersVoted: PlayerIndex[]
} | {
    type: "judgementVerdict", 
    voterPlayerIndex: PlayerIndex, 
    verdict: Verdict
} | {
    type: "trialVerdict", 
    playerOnTrial: PlayerIndex, 
    innocent: number, 
    guilty: number
} | 
// Misc.
{
    type: "targeted", 
    targeter: PlayerIndex, 
    targets: PlayerIndex[]
} | {
    type: "phaseFastForwarded"
} |
// Role-specific
{
    type: "mayorRevealed", 
    playerIndex: PlayerIndex
} | {
    type: "mayorCantWhisper"
} | {
    type: "journalistJournal",
    journal: string
} | {
    type: "youAreInterviewingPlayer",
    playerIndex: PlayerIndex
} | {
    type: "playerIsBeingInterviewed",
    playerIndex: PlayerIndex
} | {
    type: "jailedTarget"
    playerIndex: PlayerIndex
} | {
    type: "jailedSomeone",
    playerIndex: PlayerIndex
} | {
    type: "jailorDecideExecute"
    target: PlayerIndex | null
} | {
    type: "yourConvertFailed"
} | {
    type: "apostleCanConvertTonight"
} | {
    type: "apostleCantConvertTonight"
} | {
    type: "cultSacrificesRequired"
    required: number
} | {
    type: "mediumHauntStarted",
    medium: PlayerIndex,
    player: PlayerIndex
} | {
    type: "deputyKilled",
    shotIndex: PlayerIndex
} | {
    type: "deputyShotYou"
} | {
    type: "puppeteerPlayerIsNowMarionette",
    player: PlayerIndex
} | {
    type: "puppeteerYouArePoisoned"
} | {
    type: "roleBlocked", 
    immune : boolean
} | {
    type: "someoneSurvivedYourAttack"
} | {
    type: "youSurvivedAttack"
} | {
    type: "youWereAttacked"
} | {
    type: "youAttackedSomeone"
} |
/* Role-specific */
{
    type: "wardblocked"
} | {
    type: "sheriffResult", 
    suspicious: boolean
} | {
    type: "snoopResult", 
    townie: boolean
} | {
    type: "gossipResult",
    enemies: boolean
} | {
    type: "lookoutResult", 
    players: PlayerIndex[]
} | {
    type: "spyMafiaVisit", 
    players: PlayerIndex[]
} | {
    type: "spyCultistCount",
    count: number
} | {
    type: "spyBug", 
    bug: "silenced" | "roleblocked" | "protected" | "transported" | "possessed"
} | {
    type: "trackerResult",
    players: PlayerIndex[]
} | {
    type: "seerResult",
    enemies: boolean
} | {
    type: "psychicGood",
    players: PlayerIndex[]
} | {
    type: "psychicEvil",
    players: PlayerIndex[]
} | {
    type: "psychicFailed"
} | {
    type: "auditorResult",
    roleOutline: RoleOutline,
    result: AuditorResult,
} | {
    type: "veteranAttackedYou"
} | {
    type: "veteranAttackedVisitor"
} | {
    type: "engineerVisitorsRole",
    role: Role
} | {
    type: "trapState",
    state: {
        type: "dismantled" | "ready" | "set"
    }
} | {
    type: "trapStateEndOfNight",
    state: {
        type: "dismantled" | "ready" | "set"
    }
} | {
    type: "armorsmithArmorBroke"
} | {
    type: "vigilanteSuicide"
} | {
    type: "targetWasAttacked"
} | {
    type: "youAreLoveLinked",
    player: PlayerIndex
} | {
    type: "playerDiedOfABrokenHeart",
    player: PlayerIndex
    lover: PlayerIndex
} | {
    type: "youWereProtected"
} | {
    type: "youDied"
} | {
    type: "transported"
} | {
    type: "godfatherBackup",
    backup: PlayerIndex | null
} | {
    type: "godfatherBackupKilled",
    backup: PlayerIndex
} | {
    type: "silenced"
} | {
    type: "playerRoleAndAlibi",
    player: PlayerIndex,
    role: Role,
    will: string
} | {
    type: "informantResult", 
    role: Role,
    visitedBy: PlayerIndex[],
    visited: PlayerIndex[]
} | {
    type: "scarecrowResult",
    players: PlayerIndex[]
} | {
    type: "ojoActionChosen",
    action: OjoAction,
} | {
    type: "puppeteerActionChosen",
    action: PuppeteerAction,
} | {
    type: "erosActionChosen",
    action: "loveLink" | "kill",
} | {
    type: "marksmanChosenMarks",
    marks: PlayerIndex[],
} | {
    type: "targetIsPossessionImmune"
} | {
    type: "youWerePossessed",
    immune: boolean
} | {
    type: "possessionTargetsRole",
    role: Role
} | {
    type: "targetsMessage",
    message: ChatMessageVariant
} | {
    type: "werewolfTrackingResult",
    trackedPlayer: PlayerIndex
    players: PlayerIndex[]
} | {
    type: "jesterWon"
} | {
    type: "wildcardConvertFailed",
    role: Role
} | {
    type: "deathCollectedSouls"
} | {
    type: "provocateurWon"
} | {
    type: "doomsayerFailed"
} | {
    type: "doomsayerWon"
} | {
    type: "kiraResult",
    result: {
        guesses: Record<PlayerIndex, [KiraGuess, KiraGuessResult]>
    }
} | {
    type: "martyrFailed"
} | {
    type: "martyrWon"
} | {
    type: "martyrRevealed",
    martyr: PlayerIndex
}

export type MessageSender = {
    type: "player", 
    player: PlayerIndex
} | {
    type: "livingToDead",
    player: PlayerIndex,
} | {
    type: "jailor" | "journalist"
}
